use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoAbyssActivityData {
    pub RoleList: Vec<AbyssDangoRoleData>,
    pub AbyssPluginItemInfo: Vec<AbyssPluginItemInfo>,
    pub AbyssRewardInfo: Vec<AbyssRewardInfo>,
    pub UnlockChallengeIdList: Vec<i32>,
    pub LikeCount: i32,
    pub AbyssChallengeData: Vec<AbyssChallengeData>,
    pub StartTime: i64,
    pub EndTime: i64,
}

impl PbMessage for DangoAbyssActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.AbyssPluginItemInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.AbyssRewardInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.UnlockChallengeIdList {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LikeCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LikeCount);
        }
        for item in &self.AbyssChallengeData {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RoleList
                    .push(<AbyssDangoRoleData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AbyssPluginItemInfo
                    .push(<AbyssPluginItemInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.AbyssRewardInfo
                    .push(<AbyssRewardInfo as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockChallengeIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.LikeCount = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.AbyssChallengeData
                    .push(<AbyssChallengeData as PbMessage>::decode(bytes)?);
            }
            7 => self.StartTime = r.read_varint_i64()?,
            8 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeData {
    pub ChallengeId: i32,
    pub CanUnlock: bool,
    pub CanChallenge: bool,
    pub UnlockTime: i64,
    pub ConditionFinishState: bool,
    pub MaxProgress: i32,
    pub MinPassTime: i32,
    pub IsPassed: bool,
}

impl PbMessage for AbyssChallengeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CanUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanChallenge {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.ConditionFinishState {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxProgress != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxProgress);
        }
        if self.MinPassTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinPassTime);
        }
        if self.IsPassed {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.CanUnlock = r.read_bool()?,
            3 => self.CanChallenge = r.read_bool()?,
            4 => self.UnlockTime = r.read_varint_i64()?,
            5 => self.ConditionFinishState = r.read_bool()?,
            6 => self.MaxProgress = r.read_varint_i32()?,
            7 => self.MinPassTime = r.read_varint_i32()?,
            8 => self.IsPassed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssDangoRoleData {
    pub Id: i32,
    pub Level: i32,
    pub EquipItems: Vec<i32>,
}

impl PbMessage for AbyssDangoRoleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.EquipItems {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginItemInfo {
    pub ItemId: i32,
    pub Count: i32,
    pub IncrId: i32,
    pub FuncValue: i32,
}

impl PbMessage for AbyssPluginItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncValue);
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
            3 => self.IncrId = r.read_varint_i32()?,
            4 => self.FuncValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRewardInfo {
    pub Id: i32,
    pub CanGetReward: bool,
    pub CurrentProgress: i32,
    pub TargetProgress: i32,
    pub CanUnlock: bool,
}

impl PbMessage for AbyssRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CanGetReward {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurrentProgress != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentProgress);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetProgress);
        }
        if self.CanUnlock {
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
            4 => self.CanGetReward = r.read_bool()?,
            6 => self.CurrentProgress = r.read_varint_i32()?,
            7 => self.TargetProgress = r.read_varint_i32()?,
            9 => self.CanUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddCountItemInfo {
    pub Id: i32,
    pub Count: i32,
    pub IncrId: i32,
}

impl PbMessage for AddCountItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            3 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemLockRequest {
    pub ItemId: i32,
    pub IncrId: i32,
}

impl ItemLockRequest {
    pub const MSG_ID: u16 = 26312;
}

impl PbMessage for ItemLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.ItemId = r.read_varint_i32()?,
            4 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemLockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ItemLockResponse {
    pub const MSG_ID: u16 = 22732;
}

impl PbMessage for ItemLockResponse {
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
pub struct RewardItemInfo {
    pub ShowPlanId: i32,
    pub ItemId: i32,
    pub Count: i32,
    pub IncrId: i32,
}

impl PbMessage for RewardItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowPlanId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowPlanId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
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
            1 => self.ShowPlanId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            4 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemRewardNotify {
    pub DropId: i32,
    pub Reason: i32,
    pub Magnification: i32,
    pub DropFrom: Option<DoubleDropFrom>,
    pub RewardItems: BTreeMap<i32, RewardItemInfoList>,
}

impl ItemRewardNotify {
    pub const MSG_ID: u16 = 22096;
}

impl PbMessage for ItemRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropId);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.Magnification != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Magnification);
        }
        if let Some(v) = &self.DropFrom {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 7, &self.RewardItems);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.DropId = r.read_varint_i32()?,
            14 => self.Reason = r.read_varint_i32()?,
            4 => self.Magnification = r.read_varint_i32()?,
            9 => {
                self.DropFrom = Some(
                    <DoubleDropFrom as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RewardItemInfoList {
    pub ItemList: Vec<RewardItemInfo>,
}

impl PbMessage for RewardItemInfoList {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
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
                self.ItemList
                    .push(<RewardItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalItemRequest {}

impl NormalItemRequest {
    pub const MSG_ID: u16 = 17423;
}

impl PbMessage for NormalItemRequest {
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
pub struct NormalItemResponse {
    pub NormalItemList: Vec<NormalItem>,
}

impl NormalItemResponse {
    pub const MSG_ID: u16 = 22771;
}

impl PbMessage for NormalItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NormalItemList {
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
                self.NormalItemList
                    .push(<NormalItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalItemUpdateNotify {
    pub NormalItemList: Vec<NormalItem>,
    pub NoTips: bool,
}

impl NormalItemUpdateNotify {
    pub const MSG_ID: u16 = 27265;
}

impl PbMessage for NormalItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NormalItemList {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.NoTips {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.NormalItemList
                    .push(<NormalItem as PbMessage>::decode(bytes)?);
            }
            4 => self.NoTips = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalItemAddNotify {
    pub NormalItemList: Vec<NormalItem>,
    pub NoTips: bool,
    pub Reason: i32,
}

impl NormalItemAddNotify {
    pub const MSG_ID: u16 = 24714;
}

impl PbMessage for NormalItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NormalItemList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.NoTips {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.NormalItemList
                    .push(<NormalItem as PbMessage>::decode(bytes)?);
            }
            15 => self.NoTips = r.read_bool()?,
            12 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponItemRequest {}

impl WeaponItemRequest {
    pub const MSG_ID: u16 = 28901;
}

impl PbMessage for WeaponItemRequest {
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
pub struct WeaponItemResponse {
    pub WeaponItemList: Vec<WeaponItem>,
}

impl WeaponItemResponse {
    pub const MSG_ID: u16 = 24074;
}

impl PbMessage for WeaponItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponItemList {
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
                self.WeaponItemList
                    .push(<WeaponItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponItemAddNotify {
    pub WeaponItemList: Vec<WeaponItem>,
    pub AddFromRole: bool,
    pub Reason: i32,
}

impl WeaponItemAddNotify {
    pub const MSG_ID: u16 = 22519;
}

impl PbMessage for WeaponItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponItemList {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.AddFromRole {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponItemList
                    .push(<WeaponItem as PbMessage>::decode(bytes)?);
            }
            7 => self.AddFromRole = r.read_bool()?,
            5 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponItemRemoveNotify {
    pub WeaponItemIncrIdList: Vec<i32>,
}

impl WeaponItemRemoveNotify {
    pub const MSG_ID: u16 = 19935;
}

impl PbMessage for WeaponItemRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponItemIncrIdList {
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
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.WeaponItemIncrIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItemRequest {}

impl PhantomItemRequest {
    pub const MSG_ID: u16 = 24954;
}

impl PbMessage for PhantomItemRequest {
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
pub struct RolePhantomEquipInfo {
    pub RoleId: i32,
    pub PhantomItemIncrId: Vec<i32>,
}

impl PbMessage for RolePhantomEquipInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.PhantomItemIncrId {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomItemIncrId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RolePhantomPropInfo {
    pub RoleId: i32,
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
}

impl PbMessage for RolePhantomPropInfo {
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItemResponse {
    pub PhantomItemList: Vec<PhantomItem>,
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
    pub PropInfo: Vec<RolePhantomPropInfo>,
    pub MaxCost: i32,
    pub PhantomSkinList: Vec<i32>,
    pub DirectRefineWeekTimes: i32,
}

impl PhantomItemResponse {
    pub const MSG_ID: u16 = 24418;
}

impl PbMessage for PhantomItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomItemList {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.EquipInfoList {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.PropInfo {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.MaxCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCost);
        }
        for item in &self.PhantomSkinList {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.DirectRefineWeekTimes != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomItemList
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.EquipInfoList
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PropInfo
                    .push(<RolePhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.MaxCost = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomSkinList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.DirectRefineWeekTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItemAddNotify {
    pub PhantomItemList: Vec<PhantomItem>,
    pub Reason: i32,
}

impl PhantomItemAddNotify {
    pub const MSG_ID: u16 = 25657;
}

impl PbMessage for PhantomItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomItemList {
            crate::wire::write_submessage(out, 15, item);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomItemList
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            7 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItemRemoveNotify {
    pub PhantomItemIncrIdList: Vec<i32>,
}

impl PhantomItemRemoveNotify {
    pub const MSG_ID: u16 = 15222;
}

impl PbMessage for PhantomItemRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomItemIncrIdList {
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
            11 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomItemIncrIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemFuncValueUpdateNotify {
    pub IncrId: i32,
    pub FuncValue: i32,
}

impl ItemFuncValueUpdateNotify {
    pub const MSG_ID: u16 = 25472;
}

impl PbMessage for ItemFuncValueUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncValue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.IncrId = r.read_varint_i32()?,
            15 => self.FuncValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponItem {
    pub Id: i32,
    pub IncrId: i32,
    pub FuncValue: i32,
    pub WeaponLevel: i32,
    pub WeaponExp: i32,
    pub WeaponBreach: i32,
    pub WeaponResonLevel: i32,
    pub RoleId: i32,
}

impl PbMessage for WeaponItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncValue);
        }
        if self.WeaponLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponLevel);
        }
        if self.WeaponExp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponExp);
        }
        if self.WeaponBreach != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBreach);
        }
        if self.WeaponResonLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponResonLevel);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.IncrId = r.read_varint_i32()?,
            3 => self.FuncValue = r.read_varint_i32()?,
            4 => self.WeaponLevel = r.read_varint_i32()?,
            5 => self.WeaponExp = r.read_varint_i32()?,
            6 => self.WeaponBreach = r.read_varint_i32()?,
            7 => self.WeaponResonLevel = r.read_varint_i32()?,
            8 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItem {
    pub Id: i32,
    pub IncrId: i32,
    pub FuncValue: i32,
    pub PhantomLevel: i32,
    pub PhantomExp: i32,
    pub PhantomMainProp: Vec<PhantomPropInfo>,
    pub PhantomSubProp: Vec<PhantomPropInfo>,
    pub FetterGroupId: i32,
    pub SkinId: i32,
    pub UnAckSubProp: Vec<PhantomPropInfo>,
    pub LockPropIndex: Vec<i32>,
    pub StackCount: i32,
    pub AddTime: i64,
}

impl PbMessage for PhantomItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncValue);
        }
        if self.PhantomLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomLevel);
        }
        if self.PhantomExp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomExp);
        }
        for item in &self.PhantomMainProp {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PhantomSubProp {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.FetterGroupId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FetterGroupId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        for item in &self.UnAckSubProp {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.LockPropIndex {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.AddTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AddTime);
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
            2 => self.IncrId = r.read_varint_i32()?,
            3 => self.FuncValue = r.read_varint_i32()?,
            4 => self.PhantomLevel = r.read_varint_i32()?,
            5 => self.PhantomExp = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomMainProp
                    .push(<PhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomSubProp
                    .push(<PhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.FetterGroupId = r.read_varint_i32()?,
            9 => self.SkinId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.UnAckSubProp
                    .push(<PhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockPropIndex, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            12 => self.StackCount = r.read_varint_i32()?,
            13 => self.AddTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalItem {
    pub Id: i32,
    pub Count: i32,
    pub ExpireTime: i64,
}

impl PbMessage for NormalItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.ExpireTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExpireTime);
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
            4 => self.ExpireTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPropInfo {
    pub PhantomPropId: i32,
    pub Value: i32,
}

impl PbMessage for PhantomPropInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomPropId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomPropId);
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
            1 => self.PhantomPropId = r.read_varint_i32()?,
            2 => self.Value = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemPkgOpenNotify {
    pub OpenPkg: Vec<i32>,
}

impl ItemPkgOpenNotify {
    pub const MSG_ID: u16 = 17095;
}

impl PbMessage for ItemPkgOpenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OpenPkg {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.OpenPkg, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidTimeItemRequest {}

impl ValidTimeItemRequest {
    pub const MSG_ID: u16 = 17241;
}

impl PbMessage for ValidTimeItemRequest {
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
pub struct ValidTimeItem {
    pub Id: i32,
    pub Count: i32,
    pub IncrId: i32,
    pub ExpireTime: i64,
}

impl PbMessage for ValidTimeItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.ExpireTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExpireTime);
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
            3 => self.IncrId = r.read_varint_i32()?,
            4 => self.ExpireTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidTimeItemResponse {
    pub ItemList: Vec<ValidTimeItem>,
}

impl ValidTimeItemResponse {
    pub const MSG_ID: u16 = 16298;
}

impl PbMessage for ValidTimeItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
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
                self.ItemList
                    .push(<ValidTimeItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RobotRoleInfo {
    pub RoleId: i32,
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
    pub RoleEquipmentPropData: Option<RolePhantomPropInfo>,
}

impl PbMessage for RobotRoleInfo {
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
        if let Some(v) = &self.RoleEquipmentPropData {
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
                self.RoleEquipmentPropData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RobotRolePropRequest {
    pub RoleIds: Vec<i32>,
}

impl RobotRolePropRequest {
    pub const MSG_ID: u16 = 29283;
}

impl PbMessage for RobotRolePropRequest {
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
pub struct RobotRolePropResponse {
    pub Error: Option<ErrorCode>,
    pub RobotRoleInfo: Vec<RobotRoleInfo>,
}

impl RobotRolePropResponse {
    pub const MSG_ID: u16 = 16928;
}

impl PbMessage for RobotRolePropResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RobotRoleInfo {
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
            5 => {
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RobotRoleInfo
                    .push(<RobotRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDeprecateRequest {
    pub ItemId: i32,
    pub IncrId: i32,
}

impl ItemDeprecateRequest {
    pub const MSG_ID: u16 = 20767;
}

impl PbMessage for ItemDeprecateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
            1 => self.ItemId = r.read_varint_i32()?,
            12 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDeprecateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ItemDeprecateResponse {
    pub const MSG_ID: u16 = 28866;
}

impl PbMessage for ItemDeprecateResponse {
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
pub struct AccessPathTimeServerConfigRequest {}

impl AccessPathTimeServerConfigRequest {
    pub const MSG_ID: u16 = 28506;
}

impl PbMessage for AccessPathTimeServerConfigRequest {
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
pub struct AccessPathTimeServerConfig {
    pub Id: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for AccessPathTimeServerConfig {
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
pub struct AccessPathTimeServerConfigResponse {
    pub AccessPathTimeServerConfig: Vec<AccessPathTimeServerConfig>,
}

impl AccessPathTimeServerConfigResponse {
    pub const MSG_ID: u16 = 22626;
}

impl PbMessage for AccessPathTimeServerConfigResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AccessPathTimeServerConfig {
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
                self.AccessPathTimeServerConfig
                    .push(<AccessPathTimeServerConfig as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DErrorResult {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl PbMessage for DErrorResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbGetRoleListNotify {
    pub RoleList: Vec<RoleInfo>,
}

impl PbGetRoleListNotify {
    pub const MSG_ID: u16 = 23798;
}

impl PbMessage for PbGetRoleListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
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
                self.RoleList.push(<RoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbUpLevelRoleRequest {
    pub RoleId: i32,
    pub ItemList: Vec<ArrayIntInt>,
}

impl PbUpLevelRoleRequest {
    pub const MSG_ID: u16 = 17360;
}

impl PbMessage for PbUpLevelRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.ItemList {
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
            14 => self.RoleId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbUpLevelRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub Exp: i32,
    pub Level: i32,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl PbUpLevelRoleResponse {
    pub const MSG_ID: u16 = 24873;
}

impl PbMessage for PbUpLevelRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 8, &self.ItemMap);
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
            10 => self.RoleId = r.read_varint_i32()?,
            13 => self.Exp = r.read_varint_i32()?,
            14 => self.Level = r.read_varint_i32()?,
            8 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbOverRoleRequest {
    pub RoleId: i32,
}

impl PbOverRoleRequest {
    pub const MSG_ID: u16 = 17152;
}

impl PbMessage for PbOverRoleRequest {
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
pub struct PbOverRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub Breakthrough: i32,
}

impl PbOverRoleResponse {
    pub const MSG_ID: u16 = 25986;
}

impl PbMessage for PbOverRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Breakthrough != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Breakthrough);
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
            6 => self.RoleId = r.read_varint_i32()?,
            1 => self.Breakthrough = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbUpLevelSkillRequest {
    pub RoleId: i32,
    pub SkillId: i32,
    pub UseBox: bool,
}

impl PbUpLevelSkillRequest {
    pub const MSG_ID: u16 = 25868;
}

impl PbMessage for PbUpLevelSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.UseBox {
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
            2 => self.RoleId = r.read_varint_i32()?,
            12 => self.SkillId = r.read_varint_i32()?,
            9 => self.UseBox = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbUpLevelSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub SkillInfo: Option<ArrayIntInt>,
}

impl PbUpLevelSkillResponse {
    pub const MSG_ID: u16 = 26023;
}

impl PbMessage for PbUpLevelSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.SkillInfo {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.RoleId = r.read_varint_i32()?,
            1 => {
                let bytes = r.read_len_delimited()?;
                self.SkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbRolePropsNotify {
    pub RoleId: i32,
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
}

impl PbRolePropsNotify {
    pub const MSG_ID: u16 = 17579;
}

impl PbMessage for PbRolePropsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.BaseProp {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.AddProp {
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
            14 => self.RoleId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.BaseProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.AddProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArrayIntInt {
    pub Key: i32,
    pub Value: i32,
}

impl PbMessage for ArrayIntInt {
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
pub struct ArrayIntDouble {
    pub Key: i32,
    pub Value: f64,
}

impl PbMessage for ArrayIntDouble {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Key != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Key);
        }
        if self.Value != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.Value).to_bits().to_le_bytes());
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
            2 => self.Value = r.read_f64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArraySkillNode {
    pub SkillNodeId: i32,
    pub IsActive: bool,
    pub SkillId: i32,
}

impl PbMessage for ArraySkillNode {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillNodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillNodeId);
        }
        if self.IsActive {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => self.SkillNodeId = r.read_varint_i32()?,
            2 => self.IsActive = r.read_bool()?,
            3 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResonInfo {
    pub ResonId: i32,
    pub IsOpen: bool,
    pub Increase: i32,
}

impl PbMessage for ResonInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResonId);
        }
        if self.IsOpen {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Increase != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Increase);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ResonId = r.read_varint_i32()?,
            2 => self.IsOpen = r.read_bool()?,
            3 => self.Increase = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleInfo {
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

impl PbMessage for RoleInfo {
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
pub struct PbRoleSkillLevelNotify {
    pub RoleId: i32,
    pub SkillInfo: Option<ArrayIntInt>,
}

impl PbRoleSkillLevelNotify {
    pub const MSG_ID: u16 = 16361;
}

impl PbMessage for PbRoleSkillLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.SkillInfo {
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
            12 => self.RoleId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillEffect {
    pub Level: i32,
    pub EffectDescList: Vec<OneSkillEffect>,
}

impl PbMessage for SkillEffect {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.EffectDescList {
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
            1 => self.Level = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.EffectDescList
                    .push(<OneSkillEffect as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneSkillEffect {
    pub Id: i32,
    pub Desc: Vec<Vec<u8>>,
}

impl PbMessage for OneSkillEffect {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Desc {
            crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
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
            1 => self.Id = r.read_varint_i32()?,
            3 => self.Desc.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleLevelUpViewRequest {
    pub RoleId: i32,
    pub MaxItemId: i32,
    pub ItemList: Vec<ArrayIntInt>,
}

impl RoleLevelUpViewRequest {
    pub const MSG_ID: u16 = 21169;
}

impl PbMessage for RoleLevelUpViewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.MaxItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxItemId);
        }
        for item in &self.ItemList {
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
            2 => self.RoleId = r.read_varint_i32()?,
            3 => self.MaxItemId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleLevelUpViewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Level: i32,
    pub LevelExpInfo: Vec<ArrayIntInt>,
    pub Exp: i32,
    pub AddExp: i32,
    pub FinalProp: Vec<ArrayIntDouble>,
    pub CostList: Vec<ArrayIntInt>,
    pub OverflowList: Vec<ArrayIntInt>,
    pub ItemList: Vec<ArrayIntInt>,
}

impl RoleLevelUpViewResponse {
    pub const MSG_ID: u16 = 17361;
}

impl PbMessage for RoleLevelUpViewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.LevelExpInfo {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        if self.AddExp != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddExp);
        }
        for item in &self.FinalProp {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.CostList {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.OverflowList {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.ItemList {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.Level = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.LevelExpInfo
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            14 => self.Exp = r.read_varint_i32()?,
            8 => self.AddExp = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.FinalProp
                    .push(<ArrayIntDouble as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CostList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.OverflowList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleBreakThroughViewRequest {
    pub RoleId: i32,
}

impl RoleBreakThroughViewRequest {
    pub const MSG_ID: u16 = 22171;
}

impl PbMessage for RoleBreakThroughViewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleBreakThroughViewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LevelLimit: i32,
    pub UnLockSkillId: i32,
    pub CostList: Vec<ArrayIntInt>,
    pub RewardList: Vec<ArrayIntInt>,
    pub FinalProp: Vec<ArrayIntDouble>,
    pub IsConditionFinish: bool,
}

impl RoleBreakThroughViewResponse {
    pub const MSG_ID: u16 = 29247;
}

impl PbMessage for RoleBreakThroughViewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LevelLimit != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelLimit);
        }
        if self.UnLockSkillId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockSkillId);
        }
        for item in &self.CostList {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.RewardList {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.FinalProp {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.IsConditionFinish {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.LevelLimit = r.read_varint_i32()?,
            9 => self.UnLockSkillId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CostList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.RewardList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FinalProp
                    .push(<ArrayIntDouble as PbMessage>::decode(bytes)?);
            }
            15 => self.IsConditionFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillLevelUpViewRequest {
    pub RoleId: i32,
    pub SkillId: i32,
}

impl RoleSkillLevelUpViewRequest {
    pub const MSG_ID: u16 = 28071;
}

impl PbMessage for RoleSkillLevelUpViewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
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
            12 => self.RoleId = r.read_varint_i32()?,
            8 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillLevelUpViewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkillEffectList: Vec<SkillEffect>,
    pub CostList: Vec<ArrayIntInt>,
}

impl RoleSkillLevelUpViewResponse {
    pub const MSG_ID: u16 = 23907;
}

impl PbMessage for RoleSkillLevelUpViewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SkillEffectList {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.CostList {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SkillEffectList
                    .push(<SkillEffect as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CostList
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillViewRequest {
    pub RoleId: i32,
    pub SkillId: i32,
}

impl RoleSkillViewRequest {
    pub const MSG_ID: u16 = 22013;
}

impl PbMessage for RoleSkillViewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            7 => self.RoleId = r.read_varint_i32()?,
            3 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillViewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkillEffectList: Vec<SkillEffect>,
    pub PreSkillEffectList: Vec<SkillEffect>,
    pub IsConditionFinish: bool,
}

impl RoleSkillViewResponse {
    pub const MSG_ID: u16 = 17396;
}

impl PbMessage for RoleSkillViewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SkillEffectList {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.PreSkillEffectList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.IsConditionFinish {
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
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SkillEffectList
                    .push(<SkillEffect as PbMessage>::decode(bytes)?);
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.PreSkillEffectList
                    .push(<SkillEffect as PbMessage>::decode(bytes)?);
            }
            3 => self.IsConditionFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleActivateSkillRequest {
    pub RoleId: i32,
    pub SkillNodeId: i32,
}

impl RoleActivateSkillRequest {
    pub const MSG_ID: u16 = 17616;
}

impl PbMessage for RoleActivateSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillNodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillNodeId);
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
            1 => self.SkillNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleActivateSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub SkillInfo: Option<ArrayIntInt>,
}

impl RoleActivateSkillResponse {
    pub const MSG_ID: u16 = 17055;
}

impl PbMessage for RoleActivateSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.SkillInfo {
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.RoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillNodeNotify {
    pub RoleId: i32,
    pub SkillNodeState: Vec<ArraySkillNode>,
}

impl RoleSkillNodeNotify {
    pub const MSG_ID: u16 = 24848;
}

impl PbMessage for RoleSkillNodeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.SkillNodeState {
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
            11 => self.RoleId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkillNodeState
                    .push(<ArraySkillNode as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResonantChainUnlockRequest {
    pub RoleId: i32,
}

impl ResonantChainUnlockRequest {
    pub const MSG_ID: u16 = 17339;
}

impl PbMessage for ResonantChainUnlockRequest {
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
pub struct ResonantChainUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub ResonantChainGroupIndex: i32,
}

impl ResonantChainUnlockResponse {
    pub const MSG_ID: u16 = 21735;
}

impl PbMessage for ResonantChainUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.ResonantChainGroupIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResonantChainGroupIndex);
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
            10 => self.RoleId = r.read_varint_i32()?,
            11 => self.ResonantChainGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSexChangeRequest {
    pub Sex: i32,
}

impl RoleSexChangeRequest {
    pub const MSG_ID: u16 = 17543;
}

impl PbMessage for RoleSexChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Sex != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.Sex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSexChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Sex: i32,
}

impl RoleSexChangeResponse {
    pub const MSG_ID: u16 = 23208;
}

impl PbMessage for RoleSexChangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Sex != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sex);
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
            15 => self.Sex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleElementChangeRequest {
    pub ElementType: i32,
}

impl RoleElementChangeRequest {
    pub const MSG_ID: u16 = 29639;
}

impl PbMessage for RoleElementChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ElementType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ElementType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleElementChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleElementChangeResponse {
    pub const MSG_ID: u16 = 17531;
}

impl PbMessage for RoleElementChangeResponse {
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
pub struct RoleChangeNotify {
    pub SourceRoleId: i32,
    pub RoleInfo: Option<RoleInfo>,
}

impl RoleChangeNotify {
    pub const MSG_ID: u16 = 18718;
}

impl PbMessage for RoleChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceRoleId);
        }
        if let Some(v) = &self.RoleInfo {
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
            5 => self.SourceRoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleChangeUnlockNotify {
    pub UnlockRoleIds: Vec<i32>,
    pub NextAllowChangeTime: i64,
}

impl RoleChangeUnlockNotify {
    pub const MSG_ID: u16 = 20322;
}

impl PbMessage for RoleChangeUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockRoleIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.NextAllowChangeTime != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextAllowChangeTime);
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockRoleIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => self.NextAllowChangeTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinChangeRequest {
    pub RoleId: i32,
    pub SkinId: i32,
    pub IsWearWeaponSkin: bool,
}

impl RoleSkinChangeRequest {
    pub const MSG_ID: u16 = 26398;
}

impl PbMessage for RoleSkinChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.IsWearWeaponSkin {
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
            9 => self.RoleId = r.read_varint_i32()?,
            8 => self.SkinId = r.read_varint_i32()?,
            4 => self.IsWearWeaponSkin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleSkinChangeResponse {
    pub const MSG_ID: u16 = 27229;
}

impl PbMessage for RoleSkinChangeResponse {
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
pub struct UnlockRoleSkinListRequest {}

impl UnlockRoleSkinListRequest {
    pub const MSG_ID: u16 = 26601;
}

impl PbMessage for UnlockRoleSkinListRequest {
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
pub struct UnlockRoleSkinListResponse {
    pub RoleSkinList: Vec<i32>,
}

impl UnlockRoleSkinListResponse {
    pub const MSG_ID: u16 = 15433;
}

impl PbMessage for UnlockRoleSkinListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleSkinList {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleSkinList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockRoleSkinListNofity {
    pub RoleSkinList: Vec<i32>,
}

impl UnlockRoleSkinListNofity {
    pub const MSG_ID: u16 = 20442;
}

impl PbMessage for UnlockRoleSkinListNofity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleSkinList {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleSkinList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleOperateSelfBgmRequest {
    pub RoleId: i32,
    pub IsOpen: bool,
}

impl RoleOperateSelfBgmRequest {
    pub const MSG_ID: u16 = 19564;
}

impl PbMessage for RoleOperateSelfBgmRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.IsOpen {
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
            15 => self.RoleId = r.read_varint_i32()?,
            3 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleOperateSelfBgmResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
    pub IsOpen: bool,
}

impl RoleOperateSelfBgmResponse {
    pub const MSG_ID: u16 = 25411;
}

impl PbMessage for RoleOperateSelfBgmResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.IsOpen {
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.RoleId = r.read_varint_i32()?,
            4 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleDevelopConfigRequestAVersionOneof {
    Version(Vec<u8>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleDevelopConfigRequest {
    pub aVersion: Option<RoleDevelopConfigRequestAVersionOneof>,
}

impl RoleDevelopConfigRequest {
    pub const MSG_ID: u16 = 29877;
}

impl PbMessage for RoleDevelopConfigRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.aVersion {
            match oneof {
                RoleDevelopConfigRequestAVersionOneof::Version(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
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
                self.aVersion = Some(RoleDevelopConfigRequestAVersionOneof::Version(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleDevelopConfigResponse {
    pub Configs: Option<RoleDevelopConfigs>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleDevelopConfigResponse {
    pub const MSG_ID: u16 = 23021;
}

impl PbMessage for RoleDevelopConfigResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Configs {
            crate::wire::write_submessage(out, 2, v);
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
                self.Configs
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
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
pub struct RoleDevelopConfigs {
    pub DevPropsList: Vec<RoleDevPropsConfig>,
    pub DevTargetRole: i32,
    pub DevPropsProjectList: Vec<RoleDevPropsProjectConfig>,
    pub Version: Vec<u8>,
    pub DevTargetPlanId: i32,
    pub DevTargetFirstPhantomId: i32,
}

impl PbMessage for RoleDevelopConfigs {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DevPropsList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.DevTargetRole != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DevTargetRole);
        }
        for item in &self.DevPropsProjectList {
            crate::wire::write_submessage(out, 3, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
        if self.DevTargetPlanId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DevTargetPlanId);
        }
        if self.DevTargetFirstPhantomId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DevTargetFirstPhantomId);
        }
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
                self.DevPropsList
                    .push(<RoleDevPropsConfig as PbMessage>::decode(bytes)?);
            }
            2 => self.DevTargetRole = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DevPropsProjectList
                    .push(<RoleDevPropsProjectConfig as PbMessage>::decode(bytes)?);
            }
            4 => self.Version = r.read_len_delimited()?.to_vec(),
            5 => self.DevTargetPlanId = r.read_varint_i32()?,
            6 => self.DevTargetFirstPhantomId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleDevPropsProjectConfig {
    pub Id: i32,
    pub ElementId: i32,
    pub RoleName: Vec<u8>,
    pub RoleExperience: i32,
    pub RoleGoalLevel: i32,
    pub WeaponGoalLevel: i32,
    pub WeaponExperience: i32,
    pub RoleItemGroup: Vec<i32>,
    pub WeaponBreachItemGroup: Vec<i32>,
    pub WeaponType: i32,
    pub SkillItemGroup: Vec<i32>,
    pub PrefectSkillLevel: Vec<i32>,
    pub RoleHeadIcon: Vec<u8>,
    pub RoleHeadIconSmall: Vec<u8>,
    pub FormationRoleCard: Vec<u8>,
}

impl PbMessage for RoleDevPropsProjectConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ElementId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementId);
        }
        if !self.RoleName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.RoleName);
        }
        if self.RoleExperience != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleExperience);
        }
        if self.RoleGoalLevel != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleGoalLevel);
        }
        if self.WeaponGoalLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponGoalLevel);
        }
        if self.WeaponExperience != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponExperience);
        }
        for item in &self.RoleItemGroup {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.WeaponBreachItemGroup {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.WeaponType != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponType);
        }
        for item in &self.SkillItemGroup {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.PrefectSkillLevel {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.RoleHeadIcon.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.RoleHeadIcon);
        }
        if !self.RoleHeadIconSmall.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.RoleHeadIconSmall);
        }
        if !self.FormationRoleCard.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FormationRoleCard);
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
            2 => self.ElementId = r.read_varint_i32()?,
            3 => self.RoleName = r.read_len_delimited()?.to_vec(),
            4 => self.RoleExperience = r.read_varint_i32()?,
            5 => self.RoleGoalLevel = r.read_varint_i32()?,
            6 => self.WeaponGoalLevel = r.read_varint_i32()?,
            7 => self.WeaponExperience = r.read_varint_i32()?,
            8 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleItemGroup, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.WeaponBreachItemGroup,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.WeaponType = r.read_varint_i32()?,
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillItemGroup, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PrefectSkillLevel,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            13 => self.RoleHeadIcon = r.read_len_delimited()?.to_vec(),
            14 => self.RoleHeadIconSmall = r.read_len_delimited()?.to_vec(),
            15 => self.FormationRoleCard = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleDevPropsConfig {
    pub Id: i32,
    pub ProspectBeginTime: i64,
    pub ProspectEndTime: i64,
    pub TypeId: i32,
    pub GachaId: i32,
    pub SpecialGachaPair: Vec<SpecialGachaPair>,
    pub SortId: i32,
}

impl PbMessage for RoleDevPropsConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ProspectBeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ProspectBeginTime);
        }
        if self.ProspectEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ProspectEndTime);
        }
        if self.TypeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TypeId);
        }
        if self.GachaId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaId);
        }
        for item in &self.SpecialGachaPair {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.SortId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SortId);
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
            2 => self.ProspectBeginTime = r.read_varint_i64()?,
            3 => self.ProspectEndTime = r.read_varint_i64()?,
            4 => self.TypeId = r.read_varint_i32()?,
            5 => self.GachaId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SpecialGachaPair
                    .push(<SpecialGachaPair as PbMessage>::decode(bytes)?);
            }
            7 => self.SortId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpecialGachaPair {
    pub TypeId: i32,
    pub GachaId: i32,
}

impl PbMessage for SpecialGachaPair {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TypeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TypeId);
        }
        if self.GachaId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TypeId = r.read_varint_i32()?,
            2 => self.GachaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleConfigInfoNotify {
    pub RoleConfigs: Vec<RoleConfigInfo>,
}

impl RoleConfigInfoNotify {
    pub const MSG_ID: u16 = 18911;
}

impl PbMessage for RoleConfigInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleConfigs {
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
                self.RoleConfigs
                    .push(<RoleConfigInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleConfigInfo {
    pub RoleId: i32,
    pub SkillBranch: i32,
}

impl PbMessage for RoleConfigInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillBranch != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranch);
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
            2 => self.SkillBranch = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillBranchModifyRequest {
    pub RoleId: i32,
    pub SkillBranch: i32,
}

impl RoleSkillBranchModifyRequest {
    pub const MSG_ID: u16 = 26316;
}

impl PbMessage for RoleSkillBranchModifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillBranch != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranch);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.RoleId = r.read_varint_i32()?,
            15 => self.SkillBranch = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillBranchModifyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleSkillBranchModifyResponse {
    pub const MSG_ID: u16 = 23107;
}

impl PbMessage for RoleSkillBranchModifyResponse {
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
pub struct RoleConfigInfoUpdateNotify {
    pub RoleConfigs: Vec<RoleConfigInfo>,
}

impl RoleConfigInfoUpdateNotify {
    pub const MSG_ID: u16 = 16265;
}

impl PbMessage for RoleConfigInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleConfigs {
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
                self.RoleConfigs
                    .push(<RoleConfigInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillQuickLevelUpRequest {
    pub RoleId: i32,
    pub SkillId: i32,
    pub TargetLevel: i32,
    pub UseBox: bool,
}

impl RoleSkillQuickLevelUpRequest {
    pub const MSG_ID: u16 = 29438;
}

impl PbMessage for RoleSkillQuickLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.TargetLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetLevel);
        }
        if self.UseBox {
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
            11 => self.RoleId = r.read_varint_i32()?,
            1 => self.SkillId = r.read_varint_i32()?,
            7 => self.TargetLevel = r.read_varint_i32()?,
            5 => self.UseBox = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillQuickLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleInfo: Option<RoleInfo>,
}

impl RoleSkillQuickLevelUpResponse {
    pub const MSG_ID: u16 = 24928;
}

impl PbMessage for RoleSkillQuickLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RoleInfo {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamRecommendRoleInfo {
    pub RoleId: i32,
    pub SkillBranchId: i32,
}

impl PbMessage for TeamRecommendRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamRecommendTeammate {
    pub Main: Option<TeamRecommendRoleInfo>,
    pub Spares: Vec<TeamRecommendRoleInfo>,
}

impl PbMessage for TeamRecommendTeammate {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Main {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.Spares {
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
                self.Main
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Spares
                    .push(<TeamRecommendRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamRecommendInfo {
    pub SkillBranchId: i32,
    pub Teammates: Vec<TeamRecommendTeammate>,
}

impl PbMessage for TeamRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranchId);
        }
        for item in &self.Teammates {
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
            1 => self.SkillBranchId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Teammates
                    .push(<TeamRecommendTeammate as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamRecommendRole {
    pub RoleId: i32,
    pub Teams: Vec<TeamRecommendInfo>,
}

impl PbMessage for TeamRecommendRole {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.Teams {
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
                self.Teams
                    .push(<TeamRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamRecommendRequest {}

impl TeamRecommendRequest {
    pub const MSG_ID: u16 = 19191;
}

impl PbMessage for TeamRecommendRequest {
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
pub struct TeamRecommendResponse {
    pub TeamRecommendList: Vec<TeamRecommendRole>,
}

impl TeamRecommendResponse {
    pub const MSG_ID: u16 = 16900;
}

impl PbMessage for TeamRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TeamRecommendList {
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
                self.TeamRecommendList
                    .push(<TeamRecommendRole as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecGetReportData2FlowRequest {
    pub ReportData: Vec<u8>,
}

impl SecGetReportData2FlowRequest {
    pub const MSG_ID: u16 = 23542;
}

impl PbMessage for SecGetReportData2FlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ReportData.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReportData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.ReportData = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecGetReportData2FlowResponse {
    pub Error: Option<DErrorResult>,
}

impl SecGetReportData2FlowResponse {
    pub const MSG_ID: u16 = 20463;
}

impl PbMessage for SecGetReportData2FlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct Vector {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}

impl PbMessage for Vector {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
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
            3 => self.Z = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rotator {
    pub Pitch: f32,
    pub Yaw: f32,
    pub Roll: f32,
}

impl PbMessage for Rotator {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Pitch != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Pitch).to_bits().to_le_bytes());
        }
        if self.Yaw != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Yaw).to_bits().to_le_bytes());
        }
        if self.Roll != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Roll).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Pitch = r.read_f32()?,
            2 => self.Yaw = r.read_f32()?,
            3 => self.Roll = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Transform {
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
}

impl PbMessage for Transform {
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
pub struct GameplayAttributeData {
    pub BaseValue: i32,
    pub CurrentValue: i32,
    pub AttributeType: Option<EAttributeType>,
}

impl PbMessage for GameplayAttributeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BaseValue != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BaseValue);
        }
        if self.CurrentValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentValue);
        }
        if let Some(v) = &self.AttributeType {
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
            1 => self.BaseValue = r.read_varint_i32()?,
            2 => self.CurrentValue = r.read_varint_i32()?,
            3 => {
                self.AttributeType = Some(
                    <EAttributeType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct AttrData {
    pub AttributeType: Option<EAttributeType>,
    pub WhiteValue: i32,
    pub ValueIncrement: i32,
    pub CurrentValue: i32,
}

impl PbMessage for AttrData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AttributeType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.WhiteValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WhiteValue);
        }
        if self.ValueIncrement != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ValueIncrement);
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
            1 => {
                self.AttributeType = Some(
                    <EAttributeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.WhiteValue = r.read_varint_i32()?,
            3 => self.ValueIncrement = r.read_varint_i32()?,
            4 => self.CurrentValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayTagData {
    pub Id: i32,
    pub TagCount: i32,
}

impl PbMessage for GameplayTagData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.TagCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.TagCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonTagData {
    pub TagId: i32,
    pub RemoveTagIds: bool,
}

impl PbMessage for CommonTagData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TagId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        if self.RemoveTagIds {
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
            1 => self.TagId = r.read_varint_i32()?,
            2 => self.RemoveTagIds = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovementInformation {
    pub LinearVelocity: Option<Vector>,
    pub AngularVelocity: Option<Vector>,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub bSimulatedPhysicSleep: bool,
    pub bRepPhysics: bool,
    pub MovementMode: i32,
    pub TimeStamp: f32,
    pub InputDirection: i32,
    pub ResetMeshOffset: bool,
    pub IsJump: bool,
    pub HorizontalJumpSpeed: f32,
}

impl PbMessage for MovementInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LinearVelocity {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.AngularVelocity {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.bSimulatedPhysicSleep {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.bRepPhysics {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MovementMode != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MovementMode);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        if self.InputDirection != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InputDirection);
        }
        if self.ResetMeshOffset {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsJump {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HorizontalJumpSpeed != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.HorizontalJumpSpeed).to_bits().to_le_bytes());
        }
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
                self.AngularVelocity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.bSimulatedPhysicSleep = r.read_bool()?,
            6 => self.bRepPhysics = r.read_bool()?,
            7 => self.MovementMode = r.read_varint_i32()?,
            8 => self.TimeStamp = r.read_f32()?,
            9 => self.InputDirection = r.read_varint_i32()?,
            10 => self.ResetMeshOffset = r.read_bool()?,
            11 => self.IsJump = r.read_bool()?,
            12 => self.HorizontalJumpSpeed = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RelativeMoveReplaySample {
    pub BaseMovementEntityId: i64,
    pub RelativeLocation: Option<Vector>,
    pub RelativeRotation: Option<Rotator>,
}

impl PbMessage for RelativeMoveReplaySample {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BaseMovementEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BaseMovementEntityId);
        }
        if let Some(v) = &self.RelativeLocation {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.RelativeRotation {
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
            1 => self.BaseMovementEntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RelativeLocation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RelativeRotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveReplaySample {
    pub LinearVelocity: Option<Vector>,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub MovementMode: i32,
    pub TimeStamp: f32,
    pub InputDirection: i32,
    pub Tags: Vec<GameplayTagData>,
    pub RelativeMoveReplaySample: Option<RelativeMoveReplaySample>,
    pub ControllerPitch: f32,
    pub TimeScale: f32,
    pub ServerTimeStamp: i64,
    pub RTT: i32,
    pub SlideForward: Option<Vector>,
    pub MoveState: i32,
    pub SkillId: i64,
    pub ElapsedLogicTickTime: i32,
}

impl PbMessage for MoveReplaySample {
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
        if self.MovementMode != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MovementMode);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        if self.InputDirection != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InputDirection);
        }
        for item in &self.Tags {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.RelativeMoveReplaySample {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.ControllerPitch != 0.0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ControllerPitch).to_bits().to_le_bytes());
        }
        if self.TimeScale != 0.0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeScale).to_bits().to_le_bytes());
        }
        if self.ServerTimeStamp != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerTimeStamp);
        }
        if self.RTT != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RTT);
        }
        if let Some(v) = &self.SlideForward {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.MoveState != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveState);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.ElapsedLogicTickTime != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElapsedLogicTickTime);
        }
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
            4 => self.MovementMode = r.read_varint_i32()?,
            5 => self.TimeStamp = r.read_f32()?,
            6 => self.InputDirection = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Tags
                    .push(<GameplayTagData as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RelativeMoveReplaySample
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ControllerPitch = r.read_f32()?,
            10 => self.TimeScale = r.read_f32()?,
            11 => self.ServerTimeStamp = r.read_varint_i64()?,
            12 => self.RTT = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.SlideForward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.MoveState = r.read_varint_i32()?,
            15 => self.SkillId = r.read_varint_i64()?,
            16 => self.ElapsedLogicTickTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateTag {
    pub PlayerId: i32,
    pub Tags: Vec<Vec<u8>>,
}

impl PbMessage for PrivateTag {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.Tags {
            crate::wire::write_tag(out, 31, crate::wire::WireType::LengthDelimited);
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
            1 => self.PlayerId = r.read_varint_i32()?,
            31 => self.Tags.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatCommon {
    pub PreMessageId: i64,
    pub MessageId: i64,
    pub Originator: i64,
    pub TimeStamp: f32,
    pub EntityId: i64,
    pub IsServerRequest: bool,
}

impl PbMessage for CombatCommon {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PreMessageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PreMessageId);
        }
        if self.MessageId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MessageId);
        }
        if self.Originator != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Originator);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsServerRequest {
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
            1 => self.PreMessageId = r.read_varint_i64()?,
            2 => self.MessageId = r.read_varint_i64()?,
            3 => self.Originator = r.read_varint_i64()?,
            4 => self.TimeStamp = r.read_f32()?,
            5 => self.EntityId = r.read_varint_i64()?,
            6 => self.IsServerRequest = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRemoveInfo {
    pub EntityId: i64,
    pub Type: Option<ERemoveEntityType>,
}

impl PbMessage for EntityRemoveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => {
                self.Type = Some(
                    <ERemoveEntityType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoleShowEntry {
    pub RoleId: i32,
    pub Level: i32,
}

impl PbMessage for RoleShowEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerDetails {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub Level: i32,
    pub OriginWorldLevel: i32,
    pub CurWorldLevel: i32,
    pub HeadId: i32,
    pub HeadFrameId: i32,
    pub Signature: Vec<u8>,
    pub IsOnline: bool,
    pub IsCanLobbyOnline: bool,
    pub LastOfflineTime: i64,
    pub TeamMemberCount: i32,
    pub LevelGap: i32,
    pub Birthday: i32,
    pub RoleShowList: Vec<RoleShowEntry>,
    pub CardShowList: Vec<i32>,
    pub CurCard: i32,
    pub DisplayBirthday: bool,
    pub LastEnterMultiWillTime: i64,
    pub SdkUserId: Vec<u8>,
    pub SdkOnlineId: Vec<u8>,
    pub SdkAccountId: Vec<u8>,
    pub CrossPlayEnabled: bool,
    pub LimitState: i32,
    pub PlayerTitleId: i32,
    pub CurPlayerTitleId: i32,
    pub Sex: i32,
    pub Deactivation: bool,
    pub XboxUserId: Vec<u8>,
    pub XboxOnlineId: Vec<u8>,
    pub XboxAccountId: Vec<u8>,
    pub MatchXboxUser: bool,
    pub XboxSocialState: i32,
}

impl PbMessage for PlayerDetails {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.OriginWorldLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginWorldLevel);
        }
        if self.CurWorldLevel != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWorldLevel);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
        }
        if self.HeadFrameId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadFrameId);
        }
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Signature);
        }
        if self.IsOnline {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsCanLobbyOnline {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LastOfflineTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastOfflineTime);
        }
        if self.TeamMemberCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeamMemberCount);
        }
        if self.LevelGap != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGap);
        }
        if self.Birthday != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Birthday);
        }
        for item in &self.RoleShowList {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.CardShowList {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CurCard != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCard);
        }
        if self.DisplayBirthday {
            crate::wire::write_tag(out, 18, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LastEnterMultiWillTime != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastEnterMultiWillTime);
        }
        if !self.SdkUserId.is_empty() {
            crate::wire::write_tag(out, 20, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkUserId);
        }
        if !self.SdkOnlineId.is_empty() {
            crate::wire::write_tag(out, 21, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkOnlineId);
        }
        if !self.SdkAccountId.is_empty() {
            crate::wire::write_tag(out, 22, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkAccountId);
        }
        if self.CrossPlayEnabled {
            crate::wire::write_tag(out, 23, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LimitState != 0 {
            crate::wire::write_tag(out, 24, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitState);
        }
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 25, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
        }
        if self.CurPlayerTitleId != 0 {
            crate::wire::write_tag(out, 26, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPlayerTitleId);
        }
        if self.Sex != 0 {
            crate::wire::write_tag(out, 27, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sex);
        }
        if self.Deactivation {
            crate::wire::write_tag(out, 28, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.XboxUserId.is_empty() {
            crate::wire::write_tag(out, 29, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxUserId);
        }
        if !self.XboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 30, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxOnlineId);
        }
        if !self.XboxAccountId.is_empty() {
            crate::wire::write_tag(out, 31, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxAccountId);
        }
        if self.MatchXboxUser {
            crate::wire::write_tag(out, 32, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.XboxSocialState != 0 {
            crate::wire::write_tag(out, 33, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.XboxSocialState);
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
            3 => self.Level = r.read_varint_i32()?,
            4 => self.OriginWorldLevel = r.read_varint_i32()?,
            5 => self.CurWorldLevel = r.read_varint_i32()?,
            6 => self.HeadId = r.read_varint_i32()?,
            7 => self.HeadFrameId = r.read_varint_i32()?,
            8 => self.Signature = r.read_len_delimited()?.to_vec(),
            9 => self.IsOnline = r.read_bool()?,
            10 => self.IsCanLobbyOnline = r.read_bool()?,
            11 => self.LastOfflineTime = r.read_varint_i64()?,
            12 => self.TeamMemberCount = r.read_varint_i32()?,
            13 => self.LevelGap = r.read_varint_i32()?,
            14 => self.Birthday = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RoleShowList
                    .push(<RoleShowEntry as PbMessage>::decode(bytes)?);
            }
            16 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardShowList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            17 => self.CurCard = r.read_varint_i32()?,
            18 => self.DisplayBirthday = r.read_bool()?,
            19 => self.LastEnterMultiWillTime = r.read_varint_i64()?,
            20 => self.SdkUserId = r.read_len_delimited()?.to_vec(),
            21 => self.SdkOnlineId = r.read_len_delimited()?.to_vec(),
            22 => self.SdkAccountId = r.read_len_delimited()?.to_vec(),
            23 => self.CrossPlayEnabled = r.read_bool()?,
            24 => self.LimitState = r.read_varint_i32()?,
            25 => self.PlayerTitleId = r.read_varint_i32()?,
            26 => self.CurPlayerTitleId = r.read_varint_i32()?,
            27 => self.Sex = r.read_varint_i32()?,
            28 => self.Deactivation = r.read_bool()?,
            29 => self.XboxUserId = r.read_len_delimited()?.to_vec(),
            30 => self.XboxOnlineId = r.read_len_delimited()?.to_vec(),
            31 => self.XboxAccountId = r.read_len_delimited()?.to_vec(),
            32 => self.MatchXboxUser = r.read_bool()?,
            33 => self.XboxSocialState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntitySimplyMoveInfo {
    pub EntityId: i64,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
}

impl PbMessage for EntitySimplyMoveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Rotation {
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
            1 => self.EntityId = r.read_varint_i64()?,
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementProgress {
    pub CurProgress: i32,
    pub TotalProgress: i32,
}

impl PbMessage for AchievementProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurProgress != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurProgress);
        }
        if self.TotalProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalProgress);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurProgress = r.read_varint_i32()?,
            2 => self.TotalProgress = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementEntry {
    pub Id: i32,
    pub FinishTime: u32,
    pub IsReceive: bool,
    pub Progress: Option<AchievementProgress>,
}

impl PbMessage for AchievementEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.FinishTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.FinishTime));
        }
        if self.IsReceive {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Progress {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.FinishTime = r.read_varint_u32()?,
            3 => self.IsReceive = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Progress
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementGroupEntry {
    pub Id: i32,
    pub FinishTime: u32,
    pub IsReceive: bool,
}

impl PbMessage for AchievementGroupEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.FinishTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.FinishTime));
        }
        if self.IsReceive {
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
            2 => self.FinishTime = r.read_varint_u32()?,
            3 => self.IsReceive = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementGroupInfo {
    pub AchievementGroupEntry: Option<AchievementGroupEntry>,
    pub AchievementEntryList: Vec<AchievementEntry>,
}

impl PbMessage for AchievementGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AchievementGroupEntry {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.AchievementEntryList {
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
                self.AchievementGroupEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AchievementEntryList
                    .push(<AchievementEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementInfoRequest {}

impl AchievementInfoRequest {
    pub const MSG_ID: u16 = 22315;
}

impl PbMessage for AchievementInfoRequest {
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
pub struct AchievementInfoResponse {
    pub AchievementGroupInfoList: Vec<AchievementGroupInfo>,
    pub AchievementFinishedStar: i32,
    pub FinishedAchievementNum: i32,
}

impl AchievementInfoResponse {
    pub const MSG_ID: u16 = 19753;
}

impl PbMessage for AchievementInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AchievementGroupInfoList {
            crate::wire::write_submessage(out, 12, item);
        }
        if self.AchievementFinishedStar != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AchievementFinishedStar);
        }
        if self.FinishedAchievementNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinishedAchievementNum);
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
                self.AchievementGroupInfoList
                    .push(<AchievementGroupInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.AchievementFinishedStar = r.read_varint_i32()?,
            2 => self.FinishedAchievementNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateAchievementInfoRequest {}

impl UpdateAchievementInfoRequest {
    pub const MSG_ID: u16 = 28975;
}

impl PbMessage for UpdateAchievementInfoRequest {
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
pub struct UpdateAchievementInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AchievementEntryList: Vec<AchievementEntry>,
}

impl UpdateAchievementInfoResponse {
    pub const MSG_ID: u16 = 22115;
}

impl PbMessage for UpdateAchievementInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AchievementEntryList {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AchievementEntryList
                    .push(<AchievementEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelEventNotify {
    pub PlayerId: i32,
    pub IncId: i32,
    pub GameCtx: Option<GameCtxPb>,
    pub TotalCount: i32,
    pub StartIndex: i32,
    pub EndIndex: i32,
    pub NeedFinishReq: bool,
}

impl LevelEventNotify {
    pub const MSG_ID: u16 = 27631;
}

impl PbMessage for LevelEventNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
        }
        if self.StartIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartIndex);
        }
        if self.EndIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndIndex);
        }
        if self.NeedFinishReq {
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
            6 => self.PlayerId = r.read_varint_i32()?,
            12 => self.IncId = r.read_varint_i32()?,
            1 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.TotalCount = r.read_varint_i32()?,
            8 => self.StartIndex = r.read_varint_i32()?,
            10 => self.EndIndex = r.read_varint_i32()?,
            11 => self.NeedFinishReq = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameCtxPbCtxInfoOneof {
    BehaviorTree(BehaviorTreeCtxPb),
    Entity(EntityCtxPb),
    NormalInteract(NormalInteractCtxPb),
    DynamicInteract(Box<DynamicInteractCtxPb>),
    RandomInteract(RandomInteractCtxPb),
    StateChangeAction(StateChangeActionCtxPb),
    EntityGroupAction(EntityGroupActionCtxPb),
    EntityTrigger(EntityTriggerCtxPb),
    EntityLeaveTriggerCtx(EntityLeaveTriggerCtxPb),
    EntityDestructible(EntityDestructibleCtxPb),
    EntityTimelineTrack(EntityTimelineTrackCtxPb),
    LevelPlayOpenAction(LevelPlayOpenActionCtxPb),
    LevelPlayRewardAction(LevelPlayRewardActionCtxPb),
    QuestActiveAction(QuestActiveActionCtxPb),
    QuestAcceptAction(QuestAcceptActionCtxPb),
    QuestFinishAction(QuestFinishActionCtxPb),
    ChildQuestNodeEnterAction(ChildQuestNodeEnterActionCtxPb),
    ChildQuestNodeFinishAction(ChildQuestNodeFinishActionCtxPb),
    SuccessNodeAction(SuccessNodeActionCtxPb),
    FailedNodeAction(FailedNodeActionCtxPb),
    CompositionEnterAction(CompositionEnterActionCtxPb),
    EntityConditionListeningAction(EntityConditionListeningActionCtxPb),
    PlayFlowChildQuestNode(PlayFlowChildQuestNodeCtxPb),
    HandInItemChildQuestNode(HandInItemChildQuestNodeCtxPb),
    DoInteractChildQuestNode(DoInteractChildQuestNodeCtxPb),
    ActionGroupNodeAction(ActionGroupNodeActionCtxPb),
    ExploreSkillPullGiantAction(ExploreSkillPullGiantCtxPb),
    LevelPlay(LevelPlayCtxPb),
    GmLevelAction(GmLevelActionCtxPb),
    LifeCycleCreateAction(SceneItemLifeCycleComponentCreateCtxPb),
    LifeCycleDestroyAction(SceneItemLifeCycleComponentDestroyCtxPb),
    FlowAction(FlowActionCtxPb),
    DailyQuestTerminateAction(DailyQuestTerminateActionCtxPb),
    EntityBeamReceiveAction(BeamReceiveAction),
    EntityGroupFailureAction(EntityGroupFailureCtxPb),
    EntityStateChangeConditionAction(SceneItemStateChangeConditionAction),
    FlowStartTeleport(FlowStartTeleportCtxPb),
    LeaveInstEscActionCtx(LeaveInstEscActionCtxPb),
    TrampleActiveAction(TrampleActivateCtxPb),
    TrampleDeActiveAction(TrampleDeActiveCtxPb),
    RenjuCompleteAction(RenjuCompleteActionCtxPb),
    JigsawFoundationMatchedAction(JigsawFoundationMatchedActionCtxPb),
    JigsawFoundationUnMatchedAction(JigsawFoundationUnMatchedActionCtxPb),
    HookLockPointAction(HookLockPointActionCtxPb),
    ClientTriggerAction(ClientTriggerActionCtxPb),
    ExploreSkillCustomAction(ExploreSkillCustomCtxPb),
    JigsawFoundationMatchedConditionAction(JigsawFoundationMatchedConditionActionCtxPb),
    LevelPlayDestroyAction(LevelPlayDestroyActionCtxPb),
    TemplateSpawnerAction(TemplateSpawnerActionCtxPb),
    QuestDestroyAction(QuestDestroyActionCtxPb),
    CompositionConditionEnterAction(CompositionConditionEnterActionCtxPb),
    TargetGearHitPartAction(TargetGearHitPartCtxPb),
    GlobalFix(GlobalFixCtxPb),
    StuckCheckAction(StuckCheckCtxPb),
    AfterConditionAction(EntityAfterConditionActionCtxPb),
    MotorSlider(MotorSliderCtxPb),
    RollBlockGamePlayAction(RollBlockGamePlayActionCtxPb),
    Transfer(TransferCtxPb),
    DynamicEntityReward(DynamicEntityRewardCtxPb),
    BeamCastHitPlayerActionCtxPb(BeamCastHitPlayerActionCtxPb),
    ExploreSkillAction(ExploreSkillActionCtxPb),
    PollutionRangeAction(PlayEnterOrExitPollutionRangeCtxPb),
    EntityQuickHackSkillAction(EntityQuickHackSkillCtxPb),
    RecallQuestActiveAction(RecallQuestActiveActionCtxPb),
    RecallQuestAcceptAction(RecallQuestAcceptActionCtxPb),
    RecallQuestFinishAction(RecallQuestFinishActionCtxPb),
    RecallQuestDestroyAction(RecallQuestDestroyActionCtxPb),
    EdDebugEnterAction(EdDebugEnterActionCtxPb),
    CasketTrigger(CasketTriggerCtxPb),
    FoundationOperateAction(FoundationOperateActionCtxPb),
    TargetGearActionByHitTypeAction(TargetGearActionByHitTypeCtxPb),
    LongPressInteractActionCtxPb(LongPressInteractActionCtxPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameCtxPb {
    pub CtxInfo: Option<GameCtxPbCtxInfoOneof>,
    pub CtxType: Option<GameCtxType>,
}

impl PbMessage for GameCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.CtxInfo {
            match oneof {
                GameCtxPbCtxInfoOneof::BehaviorTree(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                GameCtxPbCtxInfoOneof::Entity(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                GameCtxPbCtxInfoOneof::NormalInteract(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                GameCtxPbCtxInfoOneof::DynamicInteract(v) => {
                    crate::wire::write_submessage(out, 5, &**v);
                }
                GameCtxPbCtxInfoOneof::RandomInteract(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                GameCtxPbCtxInfoOneof::StateChangeAction(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                GameCtxPbCtxInfoOneof::EntityGroupAction(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                GameCtxPbCtxInfoOneof::EntityTrigger(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                GameCtxPbCtxInfoOneof::EntityLeaveTriggerCtx(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                GameCtxPbCtxInfoOneof::EntityDestructible(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                GameCtxPbCtxInfoOneof::EntityTimelineTrack(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                GameCtxPbCtxInfoOneof::LevelPlayOpenAction(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                GameCtxPbCtxInfoOneof::LevelPlayRewardAction(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                GameCtxPbCtxInfoOneof::QuestActiveAction(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                GameCtxPbCtxInfoOneof::QuestAcceptAction(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                GameCtxPbCtxInfoOneof::QuestFinishAction(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                GameCtxPbCtxInfoOneof::ChildQuestNodeEnterAction(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                GameCtxPbCtxInfoOneof::ChildQuestNodeFinishAction(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                GameCtxPbCtxInfoOneof::SuccessNodeAction(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                GameCtxPbCtxInfoOneof::FailedNodeAction(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                GameCtxPbCtxInfoOneof::CompositionEnterAction(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                GameCtxPbCtxInfoOneof::EntityConditionListeningAction(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                GameCtxPbCtxInfoOneof::PlayFlowChildQuestNode(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                GameCtxPbCtxInfoOneof::HandInItemChildQuestNode(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                GameCtxPbCtxInfoOneof::DoInteractChildQuestNode(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                GameCtxPbCtxInfoOneof::ActionGroupNodeAction(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                GameCtxPbCtxInfoOneof::ExploreSkillPullGiantAction(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                GameCtxPbCtxInfoOneof::LevelPlay(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                GameCtxPbCtxInfoOneof::GmLevelAction(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                GameCtxPbCtxInfoOneof::LifeCycleCreateAction(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                GameCtxPbCtxInfoOneof::LifeCycleDestroyAction(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                GameCtxPbCtxInfoOneof::FlowAction(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                GameCtxPbCtxInfoOneof::DailyQuestTerminateAction(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                GameCtxPbCtxInfoOneof::EntityBeamReceiveAction(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                GameCtxPbCtxInfoOneof::EntityGroupFailureAction(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                GameCtxPbCtxInfoOneof::EntityStateChangeConditionAction(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                GameCtxPbCtxInfoOneof::FlowStartTeleport(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                GameCtxPbCtxInfoOneof::LeaveInstEscActionCtx(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                GameCtxPbCtxInfoOneof::TrampleActiveAction(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                GameCtxPbCtxInfoOneof::TrampleDeActiveAction(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                GameCtxPbCtxInfoOneof::RenjuCompleteAction(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                GameCtxPbCtxInfoOneof::JigsawFoundationMatchedAction(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                GameCtxPbCtxInfoOneof::JigsawFoundationUnMatchedAction(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                GameCtxPbCtxInfoOneof::HookLockPointAction(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                GameCtxPbCtxInfoOneof::ClientTriggerAction(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                GameCtxPbCtxInfoOneof::ExploreSkillCustomAction(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                GameCtxPbCtxInfoOneof::JigsawFoundationMatchedConditionAction(v) => {
                    crate::wire::write_submessage(out, 48, v);
                }
                GameCtxPbCtxInfoOneof::LevelPlayDestroyAction(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                GameCtxPbCtxInfoOneof::TemplateSpawnerAction(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                GameCtxPbCtxInfoOneof::QuestDestroyAction(v) => {
                    crate::wire::write_submessage(out, 51, v);
                }
                GameCtxPbCtxInfoOneof::CompositionConditionEnterAction(v) => {
                    crate::wire::write_submessage(out, 52, v);
                }
                GameCtxPbCtxInfoOneof::TargetGearHitPartAction(v) => {
                    crate::wire::write_submessage(out, 53, v);
                }
                GameCtxPbCtxInfoOneof::GlobalFix(v) => {
                    crate::wire::write_submessage(out, 54, v);
                }
                GameCtxPbCtxInfoOneof::StuckCheckAction(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                GameCtxPbCtxInfoOneof::AfterConditionAction(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                GameCtxPbCtxInfoOneof::MotorSlider(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                GameCtxPbCtxInfoOneof::RollBlockGamePlayAction(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                GameCtxPbCtxInfoOneof::Transfer(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                GameCtxPbCtxInfoOneof::DynamicEntityReward(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                GameCtxPbCtxInfoOneof::BeamCastHitPlayerActionCtxPb(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                GameCtxPbCtxInfoOneof::ExploreSkillAction(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                GameCtxPbCtxInfoOneof::PollutionRangeAction(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                GameCtxPbCtxInfoOneof::EntityQuickHackSkillAction(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                GameCtxPbCtxInfoOneof::RecallQuestActiveAction(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                GameCtxPbCtxInfoOneof::RecallQuestAcceptAction(v) => {
                    crate::wire::write_submessage(out, 67, v);
                }
                GameCtxPbCtxInfoOneof::RecallQuestFinishAction(v) => {
                    crate::wire::write_submessage(out, 68, v);
                }
                GameCtxPbCtxInfoOneof::RecallQuestDestroyAction(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                GameCtxPbCtxInfoOneof::EdDebugEnterAction(v) => {
                    crate::wire::write_submessage(out, 3401, v);
                }
                GameCtxPbCtxInfoOneof::CasketTrigger(v) => {
                    crate::wire::write_submessage(out, 3501, v);
                }
                GameCtxPbCtxInfoOneof::FoundationOperateAction(v) => {
                    crate::wire::write_submessage(out, 3502, v);
                }
                GameCtxPbCtxInfoOneof::TargetGearActionByHitTypeAction(v) => {
                    crate::wire::write_submessage(out, 3701, v);
                }
                GameCtxPbCtxInfoOneof::LongPressInteractActionCtxPb(v) => {
                    crate::wire::write_submessage(out, 3702, v);
                }
            }
        }
        if let Some(v) = &self.CtxType {
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
                let val = <BehaviorTreeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::BehaviorTree(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::Entity(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <NormalInteractCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::NormalInteract(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <DynamicInteractCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::DynamicInteract(Box::new(val)));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <RandomInteractCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RandomInteract(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <StateChangeActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::StateChangeAction(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityGroupActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityGroupAction(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityTriggerCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityTrigger(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityLeaveTriggerCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityLeaveTriggerCtx(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityDestructibleCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityDestructible(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityTimelineTrackCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityTimelineTrack(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <LevelPlayOpenActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LevelPlayOpenAction(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <LevelPlayRewardActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LevelPlayRewardAction(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestActiveActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::QuestActiveAction(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestAcceptActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::QuestAcceptAction(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestFinishActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::QuestFinishAction(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChildQuestNodeEnterActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ChildQuestNodeEnterAction(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChildQuestNodeFinishActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ChildQuestNodeFinishAction(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <SuccessNodeActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::SuccessNodeAction(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <FailedNodeActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::FailedNodeAction(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <CompositionEnterActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::CompositionEnterAction(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityConditionListeningActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityConditionListeningAction(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlayFlowChildQuestNodeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::PlayFlowChildQuestNode(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <HandInItemChildQuestNodeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::HandInItemChildQuestNode(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <DoInteractChildQuestNodeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::DoInteractChildQuestNode(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActionGroupNodeActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ActionGroupNodeAction(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExploreSkillPullGiantCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ExploreSkillPullGiantAction(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <LevelPlayCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LevelPlay(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <GmLevelActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::GmLevelAction(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemLifeCycleComponentCreateCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LifeCycleCreateAction(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemLifeCycleComponentDestroyCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LifeCycleDestroyAction(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <FlowActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::FlowAction(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <DailyQuestTerminateActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::DailyQuestTerminateAction(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <BeamReceiveAction as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityBeamReceiveAction(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityGroupFailureCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityGroupFailureAction(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemStateChangeConditionAction as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityStateChangeConditionAction(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <FlowStartTeleportCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::FlowStartTeleport(val));
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <LeaveInstEscActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LeaveInstEscActionCtx(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrampleActivateCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::TrampleActiveAction(val));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrampleDeActiveCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::TrampleDeActiveAction(val));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <RenjuCompleteActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RenjuCompleteAction(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <JigsawFoundationMatchedActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::JigsawFoundationMatchedAction(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <JigsawFoundationUnMatchedActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::JigsawFoundationUnMatchedAction(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <HookLockPointActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::HookLockPointAction(val));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientTriggerActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ClientTriggerAction(val));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExploreSkillCustomCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ExploreSkillCustomAction(val));
            }
            48 => {
                let bytes = r.read_len_delimited()?;
                let val =
                    <JigsawFoundationMatchedConditionActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo =
                    Some(GameCtxPbCtxInfoOneof::JigsawFoundationMatchedConditionAction(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <LevelPlayDestroyActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LevelPlayDestroyAction(val));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <TemplateSpawnerActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::TemplateSpawnerAction(val));
            }
            51 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestDestroyActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::QuestDestroyAction(val));
            }
            52 => {
                let bytes = r.read_len_delimited()?;
                let val = <CompositionConditionEnterActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::CompositionConditionEnterAction(val));
            }
            53 => {
                let bytes = r.read_len_delimited()?;
                let val = <TargetGearHitPartCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::TargetGearHitPartAction(val));
            }
            54 => {
                let bytes = r.read_len_delimited()?;
                let val = <GlobalFixCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::GlobalFix(val));
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <StuckCheckCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::StuckCheckAction(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityAfterConditionActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::AfterConditionAction(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorSliderCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::MotorSlider(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <RollBlockGamePlayActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RollBlockGamePlayAction(val));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransferCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::Transfer(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <DynamicEntityRewardCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::DynamicEntityReward(val));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <BeamCastHitPlayerActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::BeamCastHitPlayerActionCtxPb(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExploreSkillActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::ExploreSkillAction(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlayEnterOrExitPollutionRangeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::PollutionRangeAction(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityQuickHackSkillCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EntityQuickHackSkillAction(val));
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <RecallQuestActiveActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RecallQuestActiveAction(val));
            }
            67 => {
                let bytes = r.read_len_delimited()?;
                let val = <RecallQuestAcceptActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RecallQuestAcceptAction(val));
            }
            68 => {
                let bytes = r.read_len_delimited()?;
                let val = <RecallQuestFinishActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RecallQuestFinishAction(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <RecallQuestDestroyActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::RecallQuestDestroyAction(val));
            }
            3401 => {
                let bytes = r.read_len_delimited()?;
                let val = <EdDebugEnterActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::EdDebugEnterAction(val));
            }
            3501 => {
                let bytes = r.read_len_delimited()?;
                let val = <CasketTriggerCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::CasketTrigger(val));
            }
            3502 => {
                let bytes = r.read_len_delimited()?;
                let val = <FoundationOperateActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::FoundationOperateAction(val));
            }
            3701 => {
                let bytes = r.read_len_delimited()?;
                let val = <TargetGearActionByHitTypeCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::TargetGearActionByHitTypeAction(val));
            }
            3702 => {
                let bytes = r.read_len_delimited()?;
                let val = <LongPressInteractActionCtxPb as PbMessage>::decode(bytes)?;
                self.CtxInfo = Some(GameCtxPbCtxInfoOneof::LongPressInteractActionCtxPb(val));
            }
            1 => {
                self.CtxType = Some(<GameCtxType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityCtxPb {
    pub ConfigId: i32,
    pub IncId: i64,
}

impl PbMessage for EntityCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalInteractCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub OptionIndex: i32,
}

impl PbMessage for NormalInteractCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.OptionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DynamicInteractCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub OptionGuid: Vec<u8>,
    pub finalOptionCtx: Option<GameCtxPb>,
}

impl PbMessage for DynamicInteractCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OptionGuid);
        }
        if let Some(v) = &self.finalOptionCtx {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.finalOptionCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorSliderCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub IsEnter: bool,
}

impl PbMessage for MotorSliderCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.IsEnter {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.IsEnter = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RandomInteractCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub OptionIndex: i32,
}

impl PbMessage for RandomInteractCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.OptionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateChangeActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub StateIndex: i32,
}

impl PbMessage for StateChangeActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.StateIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateIndex);
        }
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemStateChangeConditionAction {
    pub EntityCtx: Option<EntityCtxPb>,
    pub StateIndex: i32,
    pub ConditionIndex: i32,
}

impl PbMessage for SceneItemStateChangeConditionAction {
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
pub struct EntityGroupActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub TriggerIndex: i32,
    pub IsMatch: bool,
}

impl PbMessage for EntityGroupActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TriggerIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TriggerIndex);
        }
        if self.IsMatch {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.TriggerIndex = r.read_varint_i32()?,
            3 => self.IsMatch = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityGroupFailureCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for EntityGroupFailureCtxPb {
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
pub struct EntityTriggerCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub TriggerEntityIncId: i64,
}

impl PbMessage for EntityTriggerCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TriggerEntityIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TriggerEntityIncId);
        }
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
            2 => self.TriggerEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientTriggerActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub IsEnter: bool,
}

impl PbMessage for ClientTriggerActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.IsEnter {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.IsEnter = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityLeaveTriggerCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub TriggerEntityIncId: i64,
}

impl PbMessage for EntityLeaveTriggerCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TriggerEntityIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TriggerEntityIncId);
        }
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
            2 => self.TriggerEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityDestructibleCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for EntityDestructibleCtxPb {
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
pub struct EntityTimelineTrackCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub GroupIndex: i32,
    pub ControlPoint: i32,
    pub EventType: Option<EntityTimelineEventType>,
}

impl PbMessage for EntityTimelineTrackCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.GroupIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupIndex);
        }
        if self.ControlPoint != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ControlPoint);
        }
        if let Some(v) = &self.EventType {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.GroupIndex = r.read_varint_i32()?,
            3 => self.ControlPoint = r.read_varint_i32()?,
            4 => {
                self.EventType = Some(
                    <EntityTimelineEventType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EntityConditionListeningActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub EntityConditionListeningIndex: i32,
}

impl PbMessage for EntityConditionListeningActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.EntityConditionListeningIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConditionListeningIndex);
        }
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
            2 => self.EntityConditionListeningIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAfterConditionActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub PreCondtionListeningIndex: i32,
    pub AfterCondtionListeningIndex: i32,
}

impl PbMessage for EntityAfterConditionActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PreCondtionListeningIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreCondtionListeningIndex);
        }
        if self.AfterCondtionListeningIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AfterCondtionListeningIndex);
        }
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
            2 => self.PreCondtionListeningIndex = r.read_varint_i32()?,
            3 => self.AfterCondtionListeningIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeamCastHitPlayerActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for BeamCastHitPlayerActionCtxPb {
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
pub struct ExploreSkillPullGiantCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for ExploreSkillPullGiantCtxPb {
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
pub struct ExploreSkillCustomCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for ExploreSkillCustomCtxPb {
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
pub struct GmLevelActionCtxPb {
    pub JsonStr: Vec<u8>,
}

impl PbMessage for GmLevelActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.JsonStr.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
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
            1 => self.JsonStr = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemLifeCycleComponentCreateCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for SceneItemLifeCycleComponentCreateCtxPb {
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
pub struct SceneItemLifeCycleComponentDestroyCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for SceneItemLifeCycleComponentDestroyCtxPb {
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
pub struct TrampleActivateCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for TrampleActivateCtxPb {
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
pub struct TrampleDeActiveCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for TrampleDeActiveCtxPb {
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
pub struct RenjuCompleteActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub Controller: i32,
}

impl PbMessage for RenjuCompleteActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Controller != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Controller);
        }
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
            2 => self.Controller = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JigsawFoundationMatchedActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub MatchedIndex: i32,
}

impl PbMessage for JigsawFoundationMatchedActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MatchedIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchedIndex);
        }
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
            2 => self.MatchedIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JigsawFoundationUnMatchedActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub MatchedIndex: i32,
}

impl PbMessage for JigsawFoundationUnMatchedActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MatchedIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchedIndex);
        }
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
            2 => self.MatchedIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JigsawFoundationMatchedConditionActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub MatchedIndex: i32,
    pub ConditionIndex: i32,
}

impl PbMessage for JigsawFoundationMatchedConditionActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MatchedIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchedIndex);
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
            2 => self.MatchedIndex = r.read_varint_i32()?,
            3 => self.ConditionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DynamicEntityRewardCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for DynamicEntityRewardCtxPb {
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
pub struct ExploreSkillActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for ExploreSkillActionCtxPb {
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
pub struct BehaviorTreeCtxPb {
    pub IncId: i64,
    pub BtType: Option<BtType>,
    pub BtId: i32,
    pub NodeId: i32,
}

impl PbMessage for BehaviorTreeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.IncId);
        }
        if let Some(v) = &self.BtType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BtId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
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
            2 => {
                self.BtType = Some(<BtType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.BtId = r.read_varint_i32()?,
            4 => self.NodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayCtxPb {
    pub LevelPlayId: i32,
}

impl PbMessage for LevelPlayCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayOpenActionCtxPb {
    pub LevelPlayId: i32,
}

impl PbMessage for LevelPlayOpenActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayDestroyActionCtxPb {
    pub LevelPlayId: i32,
}

impl PbMessage for LevelPlayDestroyActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardActionCtxPb {
    pub LevelPlayId: i32,
}

impl PbMessage for LevelPlayRewardActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestActiveActionCtxPb {
    pub QuestId: i32,
}

impl PbMessage for QuestActiveActionCtxPb {
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
pub struct QuestAcceptActionCtxPb {
    pub QuestId: i32,
}

impl PbMessage for QuestAcceptActionCtxPb {
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
pub struct QuestFinishActionCtxPb {
    pub QuestId: i32,
}

impl PbMessage for QuestFinishActionCtxPb {
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
pub struct DailyQuestTerminateActionCtxPb {
    pub QuestId: i32,
}

impl PbMessage for DailyQuestTerminateActionCtxPb {
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
pub struct QuestDestroyActionCtxPb {
    pub QuestId: i32,
}

impl PbMessage for QuestDestroyActionCtxPb {
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
pub struct LeaveInstEscActionCtxPb {
    pub InstanceId: i32,
}

impl PbMessage for LeaveInstEscActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChildQuestNodeEnterActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for ChildQuestNodeEnterActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChildQuestNodeFinishActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for ChildQuestNodeFinishActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SuccessNodeActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for SuccessNodeActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FailedNodeActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for FailedNodeActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompositionEnterActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for CompositionEnterActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompositionConditionEnterActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
    pub ConditionIndex: i32,
}

impl PbMessage for CompositionConditionEnterActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.ConditionIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.ConditionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetGearHitPartCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub HitPartIndex: i32,
}

impl PbMessage for TargetGearHitPartCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.HitPartIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HitPartIndex);
        }
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
            2 => self.HitPartIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetGearActionByHitTypeCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub Index: i32,
}

impl PbMessage for TargetGearActionByHitTypeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayFlowChildQuestNodeCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for PlayFlowChildQuestNodeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInItemChildQuestNodeCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for HandInItemChildQuestNodeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DoInteractChildQuestNodeCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for DoInteractChildQuestNodeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionGroupNodeActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for ActionGroupNodeActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowActionCtxPb {
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
    pub ActionId: i32,
}

impl PbMessage for FlowActionCtxPb {
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
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.FlowListName = r.read_len_delimited()?.to_vec(),
            2 => self.FlowId = r.read_varint_i32()?,
            3 => self.StateId = r.read_varint_i32()?,
            4 => self.ActionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeamReceiveAction {
    pub ReceiveType: Option<BeamReceiveActionType>,
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for BeamReceiveAction {
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
                    <BeamReceiveActionType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PlayEnterOrExitPollutionRangeCtxPb {
    pub RangeType: Option<PlayEnterOrExitPollutionRangeType>,
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for PlayEnterOrExitPollutionRangeCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RangeType {
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
                self.RangeType = Some(
                    <PlayEnterOrExitPollutionRangeType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FlowStartTeleportCtxPb {
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
}

impl PbMessage for FlowStartTeleportCtxPb {
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
pub struct HookLockPointActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub InteractionType: Option<HookInteractActionType>,
}

impl PbMessage for HookLockPointActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.InteractionType {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                self.InteractionType = Some(
                    <HookInteractActionType as crate::wire::PbEnum>::from_i32_or_default(
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
pub enum TemplateSpawnerActionCtxPbTypeOneof {
    DestroyType(DestroyType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateSpawnerActionCtxPb {
    pub Type: Option<TemplateSpawnerActionCtxPbTypeOneof>,
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for TemplateSpawnerActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Type {
            match oneof {
                TemplateSpawnerActionCtxPbTypeOneof::DestroyType(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
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
            2 => {
                self.Type = Some(TemplateSpawnerActionCtxPbTypeOneof::DestroyType(
                    <DestroyType as crate::wire::PbEnum>::from_i32_or_default(r.read_varint_i32()?),
                ))
            }
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
pub struct GlobalFixCtxPb {
    pub FixId: i32,
}

impl PbMessage for GlobalFixCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FixId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FixId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FixId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StuckCheckCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
    pub Index: i32,
}

impl PbMessage for StuckCheckCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockGamePlayActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
    pub ParamType: Option<RollBlockGameActionType>,
}

impl PbMessage for RollBlockGamePlayActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ParamType {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                self.ParamType = Some(
                    <RollBlockGameActionType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TransferCtxPb {
    pub TeleportId: i32,
}

impl PbMessage for TransferCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TeleportId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.TeleportId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityQuickHackSkillCtxPb {
    pub SkillId: i32,
    pub EntityState: i32,
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for EntityQuickHackSkillCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.EntityState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityState);
        }
        if let Some(v) = &self.EntityCtx {
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
            2 => self.EntityState = r.read_varint_i32()?,
            3 => {
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
pub struct RecallQuestActiveActionCtxPb {
    pub RecallQuestId: i32,
}

impl PbMessage for RecallQuestActiveActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdDebugEnterActionCtxPb {
    pub BehaviorTreeCtx: Option<BehaviorTreeCtxPb>,
}

impl PbMessage for EdDebugEnterActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BehaviorTreeCtx {
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
                self.BehaviorTreeCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestAcceptActionCtxPb {
    pub RecallQuestId: i32,
}

impl PbMessage for RecallQuestAcceptActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishActionCtxPb {
    pub RecallQuestId: i32,
}

impl PbMessage for RecallQuestFinishActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestDestroyActionCtxPb {
    pub RecallQuestId: i32,
}

impl PbMessage for RecallQuestDestroyActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CasketTriggerCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub Index: i32,
    pub IsEnter: bool,
}

impl PbMessage for CasketTriggerCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.IsEnter {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Index = r.read_varint_i32()?,
            3 => self.IsEnter = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoundationOperateActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub PlayerId: i32,
    pub IsTakeOut: bool,
}

impl PbMessage for FoundationOperateActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IsTakeOut {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.PlayerId = r.read_varint_i32()?,
            3 => self.IsTakeOut = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongPressInteractActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub OptionIndex: i32,
}

impl PbMessage for LongPressInteractActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.OptionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportUpdateNotify {
    pub Ids: Vec<i32>,
}

impl TeleportUpdateNotify {
    pub const MSG_ID: u16 = 25294;
}

impl PbMessage for TeleportUpdateNotify {
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
pub struct TeleportDataRequest {}

impl TeleportDataRequest {
    pub const MSG_ID: u16 = 18464;
}

impl PbMessage for TeleportDataRequest {
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
pub struct TeleportDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Ids: Vec<i32>,
}

impl TeleportDataResponse {
    pub const MSG_ID: u16 = 24608;
}

impl PbMessage for TeleportDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportTransferRequest {
    pub Id: i32,
}

impl TeleportTransferRequest {
    pub const MSG_ID: u16 = 24606;
}

impl PbMessage for TeleportTransferRequest {
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
pub struct TeleportTransferResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MapId: i32,
    pub PosX: f32,
    pub PosY: f32,
    pub PosZ: f32,
    pub Pitch: f32,
    pub Yaw: f32,
    pub Roll: f32,
}

impl TeleportTransferResponse {
    pub const MSG_ID: u16 = 29947;
}

impl PbMessage for TeleportTransferResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.PosZ != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosZ).to_bits().to_le_bytes());
        }
        if self.Pitch != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Pitch).to_bits().to_le_bytes());
        }
        if self.Yaw != 0.0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Yaw).to_bits().to_le_bytes());
        }
        if self.Roll != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Roll).to_bits().to_le_bytes());
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
            14 => self.MapId = r.read_varint_i32()?,
            9 => self.PosX = r.read_f32()?,
            3 => self.PosY = r.read_f32()?,
            2 => self.PosZ = r.read_f32()?,
            7 => self.Pitch = r.read_f32()?,
            15 => self.Yaw = r.read_f32()?,
            12 => self.Roll = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TeleportNotifyV70Oneof {
    TransferEffectId(i32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportNotify {
    pub v70: Option<TeleportNotifyV70Oneof>,
    pub MapId: i32,
    pub Pos: Option<Vector>,
    pub Rot: Option<Vector>,
    pub Gravity: Option<Vector>,
    pub Reason: Option<TeleportReason>,
    pub GameCtx: Option<GameCtxPb>,
    pub TransitionOption: Option<TransitionOptionPb>,
    pub DisableAutoFade: bool,
}

impl TeleportNotify {
    pub const MSG_ID: u16 = 25575;
}

impl PbMessage for TeleportNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.v70 {
            match oneof {
                TeleportNotifyV70Oneof::TransferEffectId(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.Gravity {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.TransitionOption {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.DisableAutoFade {
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
            1 => {
                self.v70 = Some(TeleportNotifyV70Oneof::TransferEffectId(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.MapId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Gravity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.Reason = Some(
                    <TeleportReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TransitionOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.DisableAutoFade = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionOptionPbOptionOneof {
    TransitionMp4(TransitionMp4Pb),
    TransitionFlow(TransitionFlowPb),
    TransitionInSeamless(TransitionInSeamlessPb),
    FadeInScreenShowTime(i32),
    TransitionWithCharacterDisplay(TransitionWithCharacterDisplayPb),
    TransitionWithCustomLoading(TransitionWithCustomLoadingPb),
    TransitionWithSpineLoadingPb(TransitionWithSpineLoadingPb),
    TransitionWithSpecialCustomLoadingPb(TransitionWithSpecialCustomLoadingPb),
    PlayFlowPb(TransitionPlayFlowPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionOptionPb {
    pub Option: Option<TransitionOptionPbOptionOneof>,
    pub TransitionType: Option<TransitionType>,
}

impl PbMessage for TransitionOptionPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Option {
            match oneof {
                TransitionOptionPbOptionOneof::TransitionMp4(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                TransitionOptionPbOptionOneof::TransitionFlow(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                TransitionOptionPbOptionOneof::TransitionInSeamless(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                TransitionOptionPbOptionOneof::FadeInScreenShowTime(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                TransitionOptionPbOptionOneof::TransitionWithCharacterDisplay(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                TransitionOptionPbOptionOneof::TransitionWithCustomLoading(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                TransitionOptionPbOptionOneof::TransitionWithSpineLoadingPb(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                TransitionOptionPbOptionOneof::TransitionWithSpecialCustomLoadingPb(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                TransitionOptionPbOptionOneof::PlayFlowPb(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
            }
        }
        if let Some(v) = &self.TransitionType {
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
                let val = <TransitionMp4Pb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::TransitionMp4(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionFlowPb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::TransitionFlow(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionInSeamlessPb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::TransitionInSeamless(val));
            }
            5 => {
                self.Option = Some(TransitionOptionPbOptionOneof::FadeInScreenShowTime(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionWithCharacterDisplayPb as PbMessage>::decode(bytes)?;
                self.Option =
                    Some(TransitionOptionPbOptionOneof::TransitionWithCharacterDisplay(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionWithCustomLoadingPb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::TransitionWithCustomLoading(
                    val,
                ));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionWithSpineLoadingPb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::TransitionWithSpineLoadingPb(
                    val,
                ));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionWithSpecialCustomLoadingPb as PbMessage>::decode(bytes)?;
                self.Option =
                    Some(TransitionOptionPbOptionOneof::TransitionWithSpecialCustomLoadingPb(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransitionPlayFlowPb as PbMessage>::decode(bytes)?;
                self.Option = Some(TransitionOptionPbOptionOneof::PlayFlowPb(val));
            }
            1 => {
                self.TransitionType = Some(
                    <TransitionType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct Mp4BackgroundColorPb {
    pub FadeIn: Option<Mp4BackgroundColor>,
    pub FadeOut: Option<Mp4BackgroundColor>,
}

impl PbMessage for Mp4BackgroundColorPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FadeIn {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.FadeOut {
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
                self.FadeIn = Some(
                    <Mp4BackgroundColor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                self.FadeOut = Some(
                    <Mp4BackgroundColor as crate::wire::PbEnum>::from_i32_or_default(
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
pub enum TransitionMp4PbScreenColorOneof {
    AfterTeleportScreenColor(AfterTeleportScreenColor),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionMp4Pb {
    pub ScreenColor: Option<TransitionMp4PbScreenColorOneof>,
    pub ResourePath: Vec<u8>,
    pub ReplayWhenReLogin: bool,
    pub IsFadeInScreenAfterTeleport: bool,
    pub Mp4BackgroundColor: Option<Mp4BackgroundColorPb>,
}

impl PbMessage for TransitionMp4Pb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ScreenColor {
            match oneof {
                TransitionMp4PbScreenColorOneof::AfterTeleportScreenColor(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if !self.ResourePath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourePath);
        }
        if self.ReplayWhenReLogin {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsFadeInScreenAfterTeleport {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Mp4BackgroundColor {
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
            5 => {
                self.ScreenColor = Some(TransitionMp4PbScreenColorOneof::AfterTeleportScreenColor(
                    <AfterTeleportScreenColor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            1 => self.ResourePath = r.read_len_delimited()?.to_vec(),
            2 => self.ReplayWhenReLogin = r.read_bool()?,
            3 => self.IsFadeInScreenAfterTeleport = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Mp4BackgroundColor
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionFlowPb {
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
}

impl PbMessage for TransitionFlowPb {
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

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionInSeamlessPbWeatherDaPathOneof {
    TransitionWeatherDaPath(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionInSeamlessPbEffectDaPathOneof {
    SceneEffectDaPath(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionInSeamlessPbConfigOneof {
    SeamlessTeleportFinishConfig(SeamlessTeleportFinishConfigPb),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionInSeamlessPbDOLLARiyOneof {
    NormalStreamingLoad(bool),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionInSeamlessPb {
    pub WeatherDaPath: Option<TransitionInSeamlessPbWeatherDaPathOneof>,
    pub EffectDaPath: Option<TransitionInSeamlessPbEffectDaPathOneof>,
    pub Config: Option<TransitionInSeamlessPbConfigOneof>,
    pub DOLLARiy: Option<TransitionInSeamlessPbDOLLARiyOneof>,
    pub EffectPath: Vec<u8>,
    pub LeastTime: f32,
    pub EffectExpandTime: f32,
    pub EffectCollapseTime: f32,
    pub HasFloorParams: bool,
    pub FloorParams: Option<FloorParams>,
    pub IsTeleportInPlace: bool,
    pub KeepStates: Vec<KeepMovementState>,
}

impl PbMessage for TransitionInSeamlessPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.WeatherDaPath {
            match oneof {
                TransitionInSeamlessPbWeatherDaPathOneof::TransitionWeatherDaPath(v) => {
                    crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
            }
        }
        if let Some(oneof) = &self.EffectDaPath {
            match oneof {
                TransitionInSeamlessPbEffectDaPathOneof::SceneEffectDaPath(v) => {
                    crate::wire::write_tag(out, 10, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
            }
        }
        if let Some(oneof) = &self.Config {
            match oneof {
                TransitionInSeamlessPbConfigOneof::SeamlessTeleportFinishConfig(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
            }
        }
        if let Some(oneof) = &self.DOLLARiy {
            match oneof {
                TransitionInSeamlessPbDOLLARiyOneof::NormalStreamingLoad(v) => {
                    crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
            }
        }
        if !self.EffectPath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.EffectPath);
        }
        if self.LeastTime != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.LeastTime).to_bits().to_le_bytes());
        }
        if self.EffectExpandTime != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.EffectExpandTime).to_bits().to_le_bytes());
        }
        if self.EffectCollapseTime != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.EffectCollapseTime).to_bits().to_le_bytes());
        }
        if self.HasFloorParams {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.FloorParams {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.IsTeleportInPlace {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.KeepStates {
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
            8 => {
                self.WeatherDaPath = Some(
                    TransitionInSeamlessPbWeatherDaPathOneof::TransitionWeatherDaPath(
                        r.read_len_delimited()?.to_vec(),
                    ),
                )
            }
            10 => {
                self.EffectDaPath =
                    Some(TransitionInSeamlessPbEffectDaPathOneof::SceneEffectDaPath(
                        r.read_len_delimited()?.to_vec(),
                    ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <SeamlessTeleportFinishConfigPb as PbMessage>::decode(bytes)?;
                self.Config =
                    Some(TransitionInSeamlessPbConfigOneof::SeamlessTeleportFinishConfig(val));
            }
            12 => {
                self.DOLLARiy = Some(TransitionInSeamlessPbDOLLARiyOneof::NormalStreamingLoad(
                    r.read_bool()?,
                ))
            }
            1 => self.EffectPath = r.read_len_delimited()?.to_vec(),
            2 => self.LeastTime = r.read_f32()?,
            3 => self.EffectExpandTime = r.read_f32()?,
            4 => self.EffectCollapseTime = r.read_f32()?,
            5 => self.HasFloorParams = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FloorParams
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.IsTeleportInPlace = r.read_bool()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.KeepStates, |sr| {
                Ok(
                    <KeepMovementState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TransitionWithCharacterDisplayPb {
    pub StyllId: i32,
}

impl PbMessage for TransitionWithCharacterDisplayPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StyllId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StyllId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StyllId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionWithCustomLoadingPb {
    pub ConfigId: i32,
}

impl PbMessage for TransitionWithCustomLoadingPb {
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
pub struct FloorParams {
    pub FloorMeshPath: Vec<u8>,
    pub FloorMaterialPath: Vec<u8>,
    pub PosX: f32,
    pub PosY: f32,
    pub FloorAppearTime: f32,
    pub FloorDisappearTime: f32,
}

impl PbMessage for FloorParams {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FloorMeshPath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FloorMeshPath);
        }
        if !self.FloorMaterialPath.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FloorMaterialPath);
        }
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.FloorAppearTime != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.FloorAppearTime).to_bits().to_le_bytes());
        }
        if self.FloorDisappearTime != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.FloorDisappearTime).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FloorMeshPath = r.read_len_delimited()?.to_vec(),
            2 => self.FloorMaterialPath = r.read_len_delimited()?.to_vec(),
            3 => self.PosX = r.read_f32()?,
            4 => self.PosY = r.read_f32()?,
            5 => self.FloorAppearTime = r.read_f32()?,
            6 => self.FloorDisappearTime = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeamlessTeleportFinishConfigPb {
    pub IsnotStopScreenEffect: bool,
    pub EffectExtraState: i32,
}

impl PbMessage for SeamlessTeleportFinishConfigPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsnotStopScreenEffect {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EffectExtraState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectExtraState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsnotStopScreenEffect = r.read_bool()?,
            2 => self.EffectExtraState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportFinishRequest {}

impl TeleportFinishRequest {
    pub const MSG_ID: u16 = 28614;
}

impl PbMessage for TeleportFinishRequest {
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
pub struct TeleportFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TeleportFinishResponse {
    pub const MSG_ID: u16 = 24577;
}

impl PbMessage for TeleportFinishResponse {
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
pub enum TransitionWithSpineLoadingPbBackgroundFadeInEffectPbOneof {
    FadeBackgroundFadeInEffectPb(FadeBackgroundFadeInEffectPb),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionWithSpineLoadingPbBackgroundFadeOutEffectPbOneof {
    FadeBackgroundFadeOutEffectPb(FadeBackgroundFadeOutEffectPb),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionWithSpineLoadingPbTimeOneof {
    KeepTime(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionWithSpineLoadingPbCustomShowUiPbOneof {
    ICustomShowUiPb(ICustomShowUiPb),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionWithSpineLoadingPbAkEventOneof {
    StartAkEvent(Vec<u8>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionWithSpineLoadingPb {
    pub BackgroundFadeInEffectPb: Option<TransitionWithSpineLoadingPbBackgroundFadeInEffectPbOneof>,
    pub BackgroundFadeOutEffectPb:
        Option<TransitionWithSpineLoadingPbBackgroundFadeOutEffectPbOneof>,
    pub Time: Option<TransitionWithSpineLoadingPbTimeOneof>,
    pub CustomShowUiPb: Option<TransitionWithSpineLoadingPbCustomShowUiPbOneof>,
    pub AkEvent: Option<TransitionWithSpineLoadingPbAkEventOneof>,
    pub ICustomScreenTypeBasePb: Option<ICustomScreenTypeBasePb>,
}

impl PbMessage for TransitionWithSpineLoadingPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.BackgroundFadeInEffectPb {
            match oneof {
                TransitionWithSpineLoadingPbBackgroundFadeInEffectPbOneof::FadeBackgroundFadeInEffectPb(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
            }
        }
        if let Some(oneof) = &self.BackgroundFadeOutEffectPb {
            match oneof {
                TransitionWithSpineLoadingPbBackgroundFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectPb(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if let Some(oneof) = &self.Time {
            match oneof {
                TransitionWithSpineLoadingPbTimeOneof::KeepTime(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.CustomShowUiPb {
            match oneof {
                TransitionWithSpineLoadingPbCustomShowUiPbOneof::ICustomShowUiPb(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
            }
        }
        if let Some(oneof) = &self.AkEvent {
            match oneof {
                TransitionWithSpineLoadingPbAkEventOneof::StartAkEvent(v) => {
                    crate::wire::write_tag(out, 6, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
            }
        }
        if let Some(v) = &self.ICustomScreenTypeBasePb {
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
                let val = <FadeBackgroundFadeInEffectPb as PbMessage>::decode(bytes)?;
                self.BackgroundFadeInEffectPb = Some(TransitionWithSpineLoadingPbBackgroundFadeInEffectPbOneof::FadeBackgroundFadeInEffectPb(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <FadeBackgroundFadeOutEffectPb as PbMessage>::decode(bytes)?;
                self.BackgroundFadeOutEffectPb = Some(TransitionWithSpineLoadingPbBackgroundFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectPb(val));
            }
            4 => {
                self.Time = Some(TransitionWithSpineLoadingPbTimeOneof::KeepTime(
                    r.read_f32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <ICustomShowUiPb as PbMessage>::decode(bytes)?;
                self.CustomShowUiPb =
                    Some(TransitionWithSpineLoadingPbCustomShowUiPbOneof::ICustomShowUiPb(val));
            }
            6 => {
                self.AkEvent = Some(TransitionWithSpineLoadingPbAkEventOneof::StartAkEvent(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ICustomScreenTypeBasePb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionPlayFlowPb {
    pub ActionParamPb: Option<TransitionPlayFlowActionParamPb>,
    pub FadeBackgroundFadeInEffectPb: Option<FadeBackgroundFadeInEffectPb>,
    pub FadeBackgroundFadeOutEffectPb: Option<FadeBackgroundFadeOutEffectPb>,
}

impl PbMessage for TransitionPlayFlowPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActionParamPb {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.FadeBackgroundFadeInEffectPb {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.FadeBackgroundFadeOutEffectPb {
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
                self.ActionParamPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FadeBackgroundFadeInEffectPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FadeBackgroundFadeOutEffectPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionPlayFlowActionParamPb {
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
    pub FlowGuid: Vec<u8>,
}

impl PbMessage for TransitionPlayFlowActionParamPb {
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
        if !self.FlowGuid.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowGuid);
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
            4 => self.FlowGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionWithSpecialCustomLoadingPbLoadingTypeOneof {
    HonamiStoryCustomLoadingPb(HonamiStoryCustomLoadingPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransitionWithSpecialCustomLoadingPb {
    pub LoadingType: Option<TransitionWithSpecialCustomLoadingPbLoadingTypeOneof>,
}

impl PbMessage for TransitionWithSpecialCustomLoadingPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.LoadingType {
            match oneof {
                TransitionWithSpecialCustomLoadingPbLoadingTypeOneof::HonamiStoryCustomLoadingPb(v) => {
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
                let val = <HonamiStoryCustomLoadingPb as PbMessage>::decode(bytes)?;
                self.LoadingType = Some(TransitionWithSpecialCustomLoadingPbLoadingTypeOneof::HonamiStoryCustomLoadingPb(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryCustomLoadingPb {
    pub LoadingId: i32,
}

impl PbMessage for HonamiStoryCustomLoadingPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadingId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadingId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LoadingId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomScreenTypeBasePbScreenPbOneof {
    ICustomScreenSpinePb(ICustomScreenSpinePb),
    ICustomScreenBackgroundImagePb(ICustomScreenBackgroundImagePb),
    ICustomScreenLoadingPb(ICustomScreenLoadingPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenTypeBasePb {
    pub ScreenPb: Option<ICustomScreenTypeBasePbScreenPbOneof>,
}

impl PbMessage for ICustomScreenTypeBasePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ScreenPb {
            match oneof {
                ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenSpinePb(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenBackgroundImagePb(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenLoadingPb(v) => {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                let val = <ICustomScreenSpinePb as PbMessage>::decode(bytes)?;
                self.ScreenPb = Some(ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenSpinePb(
                    val,
                ));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <ICustomScreenBackgroundImagePb as PbMessage>::decode(bytes)?;
                self.ScreenPb =
                    Some(ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenBackgroundImagePb(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <ICustomScreenLoadingPb as PbMessage>::decode(bytes)?;
                self.ScreenPb =
                    Some(ICustomScreenTypeBasePbScreenPbOneof::ICustomScreenLoadingPb(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenSpinePb {
    pub SpineId: i32,
}

impl PbMessage for ICustomScreenSpinePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SpineId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpineId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SpineId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenBackgroundImagePb {
    pub BgPath: Vec<u8>,
}

impl PbMessage for ICustomScreenBackgroundImagePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BgPath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.BgPath);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BgPath = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomScreenLoadingPbLoadingTypeOneof {
    ICustomScreenLoadingCyberpunkPb(ICustomScreenLoadingCyberpunkPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenLoadingPb {
    pub LoadingType: Option<ICustomScreenLoadingPbLoadingTypeOneof>,
}

impl PbMessage for ICustomScreenLoadingPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.LoadingType {
            match oneof {
                ICustomScreenLoadingPbLoadingTypeOneof::ICustomScreenLoadingCyberpunkPb(v) => {
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
                let val = <ICustomScreenLoadingCyberpunkPb as PbMessage>::decode(bytes)?;
                self.LoadingType = Some(
                    ICustomScreenLoadingPbLoadingTypeOneof::ICustomScreenLoadingCyberpunkPb(val),
                );
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenLoadingCyberpunkPb {}

impl PbMessage for ICustomScreenLoadingCyberpunkPb {
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
pub enum ICustomShowUiPbCustomScreenTextSettingPbOneof {
    ICustomScreenTextSettingPb(ICustomScreenTextSettingPb),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomShowUiPbHideCircleOneof {
    IsHideCircle(bool),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomShowUiPb {
    pub CustomScreenTextSettingPb: Option<ICustomShowUiPbCustomScreenTextSettingPbOneof>,
    pub HideCircle: Option<ICustomShowUiPbHideCircleOneof>,
}

impl PbMessage for ICustomShowUiPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.CustomScreenTextSettingPb {
            match oneof {
                ICustomShowUiPbCustomScreenTextSettingPbOneof::ICustomScreenTextSettingPb(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
            }
        }
        if let Some(oneof) = &self.HideCircle {
            match oneof {
                ICustomShowUiPbHideCircleOneof::IsHideCircle(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
                let bytes = r.read_len_delimited()?;
                let val = <ICustomScreenTextSettingPb as PbMessage>::decode(bytes)?;
                self.CustomScreenTextSettingPb = Some(
                    ICustomShowUiPbCustomScreenTextSettingPbOneof::ICustomScreenTextSettingPb(val),
                );
            }
            2 => {
                self.HideCircle = Some(ICustomShowUiPbHideCircleOneof::IsHideCircle(r.read_bool()?))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomScreenTextSettingPbShowTextInfoOneof {
    IsShowTextInfo(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomScreenTextSettingPbTextContentOneof {
    TidTextContent(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ICustomScreenTextSettingPbEdTextContentOneof {
    EdTidTextContent(Vec<u8>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ICustomScreenTextSettingPb {
    pub ShowTextInfo: Option<ICustomScreenTextSettingPbShowTextInfoOneof>,
    pub TextContent: Option<ICustomScreenTextSettingPbTextContentOneof>,
    pub EdTextContent: Option<ICustomScreenTextSettingPbEdTextContentOneof>,
}

impl PbMessage for ICustomScreenTextSettingPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ShowTextInfo {
            match oneof {
                ICustomScreenTextSettingPbShowTextInfoOneof::IsShowTextInfo(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
            }
        }
        if let Some(oneof) = &self.TextContent {
            match oneof {
                ICustomScreenTextSettingPbTextContentOneof::TidTextContent(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
            }
        }
        if let Some(oneof) = &self.EdTextContent {
            match oneof {
                ICustomScreenTextSettingPbEdTextContentOneof::EdTidTextContent(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
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
                self.ShowTextInfo = Some(
                    ICustomScreenTextSettingPbShowTextInfoOneof::IsShowTextInfo(r.read_bool()?),
                )
            }
            2 => {
                self.TextContent = Some(ICustomScreenTextSettingPbTextContentOneof::TidTextContent(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            3 => {
                self.EdTextContent = Some(
                    ICustomScreenTextSettingPbEdTextContentOneof::EdTidTextContent(
                        r.read_len_delimited()?.to_vec(),
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeInEffectPbFadeInEffectPbOneof {
    FadeBackgroundFadeInEffectBlackPb(FadeBackgroundFadeInEffectBlackPb),
    FadeBackgroundFadeInEffectScreenPb(FadeBackgroundFadeInEffectScreenPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeInEffectPb {
    pub FadeInEffectPb: Option<FadeBackgroundFadeInEffectPbFadeInEffectPbOneof>,
}

impl PbMessage for FadeBackgroundFadeInEffectPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.FadeInEffectPb {
            match oneof {
                FadeBackgroundFadeInEffectPbFadeInEffectPbOneof::FadeBackgroundFadeInEffectBlackPb(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                FadeBackgroundFadeInEffectPbFadeInEffectPbOneof::FadeBackgroundFadeInEffectScreenPb(v) => {
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
                let val = <FadeBackgroundFadeInEffectBlackPb as PbMessage>::decode(bytes)?;
                self.FadeInEffectPb = Some(FadeBackgroundFadeInEffectPbFadeInEffectPbOneof::FadeBackgroundFadeInEffectBlackPb(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <FadeBackgroundFadeInEffectScreenPb as PbMessage>::decode(bytes)?;
                self.FadeInEffectPb = Some(FadeBackgroundFadeInEffectPbFadeInEffectPbOneof::FadeBackgroundFadeInEffectScreenPb(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeInEffectBlackPbFadeInOneof {
    FadeInTime(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeInEffectBlackPbFadeOutOneof {
    FadeOutTime(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeInEffectBlackPb {
    pub FadeIn: Option<FadeBackgroundFadeInEffectBlackPbFadeInOneof>,
    pub FadeOut: Option<FadeBackgroundFadeInEffectBlackPbFadeOutOneof>,
    pub FadeColor: i32,
}

impl PbMessage for FadeBackgroundFadeInEffectBlackPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.FadeIn {
            match oneof {
                FadeBackgroundFadeInEffectBlackPbFadeInOneof::FadeInTime(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.FadeOut {
            match oneof {
                FadeBackgroundFadeInEffectBlackPbFadeOutOneof::FadeOutTime(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.FadeColor != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FadeColor);
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
                self.FadeIn = Some(FadeBackgroundFadeInEffectBlackPbFadeInOneof::FadeInTime(
                    r.read_f32()?,
                ))
            }
            3 => {
                self.FadeOut = Some(FadeBackgroundFadeInEffectBlackPbFadeOutOneof::FadeOutTime(
                    r.read_f32()?,
                ))
            }
            1 => self.FadeColor = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeInEffectScreenPb {
    pub ScreenEffect: Vec<u8>,
}

impl PbMessage for FadeBackgroundFadeInEffectScreenPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ScreenEffect.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ScreenEffect);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ScreenEffect = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof {
    FadeBackgroundFadeOutEffectBlackPb(FadeBackgroundFadeOutEffectBlackPb),
    FadeBackgroundFadeOutEffectSceenPb(FadeBackgroundFadeOutEffectSceenPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeOutEffectPb {
    pub FadeOutEffectPb: Option<FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof>,
}

impl PbMessage for FadeBackgroundFadeOutEffectPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.FadeOutEffectPb {
            match oneof {
                FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectBlackPb(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectSceenPb(v) => {
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
                let val = <FadeBackgroundFadeOutEffectBlackPb as PbMessage>::decode(bytes)?;
                self.FadeOutEffectPb = Some(FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectBlackPb(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <FadeBackgroundFadeOutEffectSceenPb as PbMessage>::decode(bytes)?;
                self.FadeOutEffectPb = Some(FadeBackgroundFadeOutEffectPbFadeOutEffectPbOneof::FadeBackgroundFadeOutEffectSceenPb(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeOutEffectBlackPbFadeInOneof {
    FadeInTime(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FadeBackgroundFadeOutEffectBlackPbFadeOutOneof {
    FadeOutTime(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeOutEffectBlackPb {
    pub FadeIn: Option<FadeBackgroundFadeOutEffectBlackPbFadeInOneof>,
    pub FadeOut: Option<FadeBackgroundFadeOutEffectBlackPbFadeOutOneof>,
    pub FadeColor: i32,
}

impl PbMessage for FadeBackgroundFadeOutEffectBlackPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.FadeIn {
            match oneof {
                FadeBackgroundFadeOutEffectBlackPbFadeInOneof::FadeInTime(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.FadeOut {
            match oneof {
                FadeBackgroundFadeOutEffectBlackPbFadeOutOneof::FadeOutTime(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.FadeColor != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FadeColor);
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
                self.FadeIn = Some(FadeBackgroundFadeOutEffectBlackPbFadeInOneof::FadeInTime(
                    r.read_f32()?,
                ))
            }
            3 => {
                self.FadeOut = Some(FadeBackgroundFadeOutEffectBlackPbFadeOutOneof::FadeOutTime(
                    r.read_f32()?,
                ))
            }
            1 => self.FadeColor = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FadeBackgroundFadeOutEffectSceenPb {
    pub ScreenEffect: Vec<u8>,
}

impl PbMessage for FadeBackgroundFadeOutEffectSceenPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ScreenEffect.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ScreenEffect);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ScreenEffect = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CircumFluenceTaskData {
    pub ActivityTasks: Vec<ActivityTask>,
    pub ClaimedReward: Vec<ActivityTask>,
    pub TaskScoreRewardId: Vec<i32>,
    pub NowOpen: bool,
    pub EndTime: i64,
    pub NextRefreshTime: i64,
}

impl PbMessage for CircumFluenceTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ClaimedReward {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.TaskScoreRewardId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.NowOpen {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ClaimedReward
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TaskScoreRewardId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.NowOpen = r.read_bool()?,
            6 => self.EndTime = r.read_varint_i64()?,
            7 => self.NextRefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourActivityChallenge {
    pub ChallengeId: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for ParkourActivityChallenge {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.BeginTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourActivity {
    pub Challenges: Vec<ParkourActivityChallenge>,
}

impl PbMessage for ParkourActivity {
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
                    .push(<ParkourActivityChallenge as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DoubleInstActivityReward {
    pub GetDoubleInstRwdCount: i32,
}

impl PbMessage for DoubleInstActivityReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GetDoubleInstRwdCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GetDoubleInstRwdCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GetDoubleInstRwdCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatherTaskDoneInfo {
    pub TaskId: i32,
    pub State: Option<GatherActivityTaskState>,
}

impl PbMessage for GatherTaskDoneInfo {
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
                    <GatherActivityTaskState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GatherActivityInfo {
    pub GatherTaskDoneInfo: Vec<GatherTaskDoneInfo>,
}

impl PbMessage for GatherActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GatherTaskDoneInfo {
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
                self.GatherTaskDoneInfo
                    .push(<GatherTaskDoneInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignActivity {
    pub SignStateList: Vec<SignState>,
    pub RewardFree: bool,
}

impl PbMessage for SignActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SignStateList {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        if self.RewardFree {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SignStateList, |sr| {
                    Ok(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                })?
            }
            2 => self.RewardFree = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestPointReward {
    pub Id: i32,
    pub State: Option<SignState>,
}

impl PbMessage for HarvestPointReward {
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
                self.State = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestLevelReward {
    pub Id: i32,
    pub StartTime: i32,
    pub IsOpen: bool,
    pub Points: i32,
    pub Diff: i32,
    pub State: Option<SignState>,
}

impl PbMessage for HarvestLevelReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartTime);
        }
        if self.IsOpen {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Points != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Points);
        }
        if self.Diff != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Diff);
        }
        if let Some(v) = &self.State {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.StartTime = r.read_varint_i32()?,
            3 => self.IsOpen = r.read_bool()?,
            4 => self.Points = r.read_varint_i32()?,
            5 => self.Diff = r.read_varint_i32()?,
            6 => {
                self.State = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivity {
    pub HarvestPointRewards: Vec<HarvestPointReward>,
    pub HarvestLevelRewards: Vec<HarvestLevelReward>,
}

impl PbMessage for HarvestActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HarvestPointRewards {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.HarvestLevelRewards {
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
                self.HarvestPointRewards
                    .push(<HarvestPointReward as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HarvestLevelRewards
                    .push(<HarvestLevelReward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewBieCourseActivity {
    pub HadTakeReward: Vec<i32>,
}

impl PbMessage for NewBieCourseActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HadTakeReward {
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
pub struct RoleTrialTask {
    pub RoleId: i32,
    pub ChallengeState: Option<ChallengeState>,
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
}

impl PbMessage for RoleTrialTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.ChallengeState {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
        if self.EndOpenTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                self.ChallengeState = Some(
                    <ChallengeState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.BeginOpenTime = r.read_varint_i64()?,
            6 => self.EndOpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrialInfoActivity {
    pub RoleTrialTask: Vec<RoleTrialTask>,
}

impl PbMessage for RoleTrialInfoActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleTrialTask {
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
                self.RoleTrialTask
                    .push(<RoleTrialTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhantomCollectRewardDataOneof {
    Progress(PhantomCollectProgress),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomCollectReward {
    pub Data: Option<PhantomCollectRewardDataOneof>,
    pub Type: Option<PhantomCollectRewardType>,
    pub State: Option<SignState>,
}

impl PbMessage for PhantomCollectReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                PhantomCollectRewardDataOneof::Progress(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomCollectProgress as PbMessage>::decode(bytes)?;
                self.Data = Some(PhantomCollectRewardDataOneof::Progress(val));
            }
            1 => {
                self.Type = Some(
                    <PhantomCollectRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                self.State = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomCollectProgress {
    pub Phantoms: Vec<i32>,
}

impl PbMessage for PhantomCollectProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Phantoms {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Phantoms, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomCollectActivity {
    pub PhantomCollectRewards: Vec<PhantomCollectReward>,
}

impl PbMessage for PhantomCollectActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomCollectRewards {
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
                self.PhantomCollectRewards
                    .push(<PhantomCollectReward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityRogueData {
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
    pub RoguelikeSeason: Option<RoguelikeSeason>,
}

impl PbMessage for ActivityRogueData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
        if self.EndOpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndOpenTime);
        }
        if let Some(v) = &self.RoguelikeSeason {
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
                self.RoguelikeSeason
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTokenList {
    pub Id: i32,
    pub IsReceive: bool,
}

impl PbMessage for RoguelikeTokenList {
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
pub struct RogueSeasonReward {
    pub Id: i32,
    pub IsReceive: bool,
}

impl PbMessage for RogueSeasonReward {
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
pub struct RoguelikeSeason {
    pub SeasonId: i32,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RoguelikeTokenList: Vec<RoguelikeTokenList>,
    pub SeasonRewardList: Vec<RogueSeasonReward>,
    pub TokenItemCount: i32,
    pub BlackFlowerUseCount: i32,
    pub BlackFlowerMaxCount: i32,
    pub TowerTrialBestClearCount: i32,
}

impl PbMessage for RoguelikeSeason {
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
                    .push(<RoguelikeTokenList as PbMessage>::decode(bytes)?);
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
pub struct ActivityRoleGiveData {
    pub IsGetReward: bool,
}

impl PbMessage for ActivityRoleGiveData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsGetReward {
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
            1 => self.IsGetReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityCorniceMeetingData {
    pub UnlockTime: i64,
    pub LevelEntryData: BTreeMap<i32, ActivityCorniceMeetingLevelEntryData>,
}

impl PbMessage for ActivityCorniceMeetingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if !self.LevelEntryData.is_empty() {
            crate::wire::write_map(out, 2, &self.LevelEntryData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.UnlockTime = r.read_varint_i64()?,
            2 => crate::wire::decode_map_entry(r, &mut self.LevelEntryData)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdvertisingPageData {
    pub Show: bool,
    pub PointTime: i64,
}

impl PbMessage for AdvertisingPageData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Show {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PointTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PointTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Show = r.read_bool()?,
            2 => self.PointTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityDataDataOneof {
    ParkourActivity(ParkourActivity),
    SignActivity(SignActivity),
    NewBieCourseActivity(NewBieCourseActivity),
    DoubleInstActivityReward(DoubleInstActivityReward),
    HarvestActivity(HarvestActivity),
    RoleTrialInfoActivity(RoleTrialInfoActivity),
    PhantomCollectActivity(PhantomCollectActivity),
    GatherActivityInfo(GatherActivityInfo),
    DailyAdventureActivityData(DailyAdventureActivityData),
    ActivityRogueData(ActivityRogueData),
    ActivityLongShanMain(ActivityLongShanMain),
    ActivityTurnTableData(ActivityTurnTableData),
    BossRushActivityData(BossRushActivityData),
    TrackMoonActivityTaskData(ActivityTaskData),
    ActivityTimePointRewarData(ActivityTimePointRewarData),
    TowerDefenseActivityInfo(TowerDefenseActivityInfo),
    CircumFluenceTaskData(CircumFluenceTaskData),
    ActivityRoleGiveData(ActivityRoleGiveData),
    ActivityCorniceMeetingData(ActivityCorniceMeetingData),
    RiskHarvestActivityData(RiskHarvestActivityData),
    ActivityBlackCoastData(ActivityBlackCoastData),
    DreamLinkActivityData(DreamLinkActivityData),
    ActivityScratchTicketData(ActivityScratchTicketData),
    PreheatSignActivityData(PreheatSignActivityData),
    MowTowerActivityData(MowTowerActivityData),
    FarmGoldData(FarmGoldData),
    SpringSignData(SpringSignData),
    MapTravelActivityData(MapTravelActivityData),
    RoleSkinTrialActivity(RoleSkinTrialActivity),
    ActivityFishingData(ActivityFishingData),
    ActivityWeeklyRogueData(ActivityWeeklyRogueData),
    SolarisSpeedActivity(SolarisSpeedActivity),
    BabelTowerActivity(BabelTowerActivity),
    ActivityBetHorsesData(ActivityBetHorsesData),
    ActivityMapExploreData(ActivityMapExploreData),
    ActivityPermanentRogueData(ActivityPermanentRogueData),
    ActivityAvignon(ActivityAvignon),
    DangoAbyssActivityData(DangoAbyssActivityData),
    ActivityInviteNewbie(ActivityInviteNewbie),
    ActivityDangoMonopolyData(ActivityDangoMonopolyData),
    ActivityCiacconaGalData(ActivityCiacconaGalData),
    ActivityLinkageData(ActivityLinkageData),
    ActivityRegressData(ActivityRegressData),
    CumulativeShopData(CumulativeShopData),
    PhantomArenaActivityData(PhantomArenaActivityData),
    BeginnerCarnivalData(BeginnerCarnivalData),
    ActivityMoraleData(ActivityMoraleData),
    FloroRanchActivityData(FloroRanchActivityData),
    LifePointDrawActivityData(LifePointDrawActivityData),
    ActivityTrapDefenseData(ActivityTrapDefenseData),
    ActivityFunPlayData(ActivityFunPlayData),
    ActivitySoarData(ActivitySoarData),
    ActivityLineCrossData(ActivityLineCrossData),
    ActivityMoonSignInData(ActivityMoonSignInData),
    HonamiStoryActivityData(HonamiStoryActivityData),
    FightPhotoActivityData(FightPhotoActivityData),
    SurvivorsActivityData(SurvivorsActivityData),
    ActivityPrizeDrawingData(ActivityPrizeDrawingData),
    AdvertisingPageData(AdvertisingPageData),
    AdvertisingPageInfo(AdvertisingPageInfo),
    ArtemisActivityInfo(ArtemisActivityInfo),
    RoleCoopActivityData(RoleCoopActivityData),
    MotorCycleIpActivityData(MotorCycleIpActivityData),
    PhantomBattleRecordActivityInfo(PhantomArenaActivityData),
    InfrThemeActivityPb(InfrThemeActivityPb),
    RoadBookActivityInfo(RoadBookActivityInfo),
    PhantomBattleGuideActivity(PhantomBattleGuideActivity),
    NewTowerClimbingActivityData(NewTowerClimbingActivityData),
    NewPlayerSupportActivityData(NewPlayerSupportActivityData),
    MotorParkourActivityInfo(MotorParkourActivityInfo),
    SpringFestivalActivityInfo(SpringFestivalActivityInfo),
    TotalTopUpActivityInfo(TotalTopUpActivityInfo),
    MotorFightActivityPb(MotorFightActivityPb),
    H5ViewActivityData(H5ViewActivityData),
    SkinRewardActivityData(SkinRewardActivityData),
    EncircleActivityPb(EncircleActivityPb),
    MotorDevelopActivityData(MotorDevelopActivityData),
    FlagChallengeActivityInfo(FlagChallengeActivityInfo),
    FeiXuePreheatActivityInfo(FeiXuePreheatActivityInfo),
    RhythmActivityPb(RhythmActivityPb),
    DropCatchActivityInfo(DropCatchActivityInfo),
    TetrisActivityInfo(TetrisActivityInfo),
    PinballActivityData(PinballActivityData),
    BossPilingActivityInfo(BossPilingActivityInfo),
    ThemeCelebration(ThemeCelebration),
    WuWuWeekActivity(WuWuWeekActivity),
    MotorDecalActivityData(MotorDecalActivityData),
    LinkageCheckInActivityData(LinkageCheckInActivityData),
    RoleGiftActivityData(RoleGiftActivityData),
    KurotatoActivityData(KurotatoActivityData),
    GolemCrackActivityInfo(GolemCrackActivityInfo),
    EdgeRunnerActivityInfo(EdgeRunnerActivityInfo),
    OnlineMotorActivityData(OnlineMotorActivityData),
    RealmBetweenActivityInfo(RealmBetweenActivityInfo),
    ThroughTrainSummaryActivityData(ThroughTrainSummaryActivityData),
    NewbieMainActivityPb(NewbieMainActivityPb),
    NewbieCourseV2ActivityPb(NewbieCourseV2ActivityPb),
    NewbieAdventureV2Pb(NewbieAdventureV2Pb),
    NewPlayerSupportActivityV2Pb(NewPlayerSupportActivityV2Pb),
    ThroughTrainActivityData(ThroughTrainActivityData),
    RoverRogueActivityData(RoverRogueActivityData),
    DangoBrawlActivityInfo(DangoBrawlActivityInfo),
    QingXiaoActivityInfo(QingXiaoActivityInfo),
    MatchThreeActivityInfo(MatchThreeActivityInfo),
    GiantActivityInfo(GiantActivityInfo),
    SolarSchoolActivityPb(SolarSchoolActivityPb),
    RoleTowerActivityData(RoleTowerActivityData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityData {
    pub Data: Option<ActivityDataDataOneof>,
    pub Id: i32,
    pub Type: Option<ActivityType>,
    pub BeginShowTime: i64,
    pub EndShowTime: i64,
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
    pub IsUnlock: bool,
    pub CompletePreQuests: Vec<i32>,
    pub IsFirstOpen: bool,
    pub FinishConditions: Vec<i32>,
    pub ActivityOpenType: Option<ActivityOpenType>,
    pub IsPreOpen: bool,
    pub StartTime: i64,
    pub EndTime: i64,
    pub BeginRewardTimeInternal: i64,
    pub EndRewardTimeInternal: i64,
}

impl PbMessage for ActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                ActivityDataDataOneof::ParkourActivity(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                ActivityDataDataOneof::SignActivity(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                ActivityDataDataOneof::NewBieCourseActivity(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                ActivityDataDataOneof::DoubleInstActivityReward(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                ActivityDataDataOneof::HarvestActivity(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                ActivityDataDataOneof::RoleTrialInfoActivity(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                ActivityDataDataOneof::PhantomCollectActivity(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                ActivityDataDataOneof::GatherActivityInfo(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                ActivityDataDataOneof::DailyAdventureActivityData(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                ActivityDataDataOneof::ActivityRogueData(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                ActivityDataDataOneof::ActivityLongShanMain(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                ActivityDataDataOneof::ActivityTurnTableData(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                ActivityDataDataOneof::BossRushActivityData(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                ActivityDataDataOneof::TrackMoonActivityTaskData(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                ActivityDataDataOneof::ActivityTimePointRewarData(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                ActivityDataDataOneof::TowerDefenseActivityInfo(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                ActivityDataDataOneof::CircumFluenceTaskData(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                ActivityDataDataOneof::ActivityRoleGiveData(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                ActivityDataDataOneof::ActivityCorniceMeetingData(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                ActivityDataDataOneof::RiskHarvestActivityData(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                ActivityDataDataOneof::ActivityBlackCoastData(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                ActivityDataDataOneof::DreamLinkActivityData(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                ActivityDataDataOneof::ActivityScratchTicketData(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                ActivityDataDataOneof::PreheatSignActivityData(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                ActivityDataDataOneof::MowTowerActivityData(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                ActivityDataDataOneof::FarmGoldData(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                ActivityDataDataOneof::SpringSignData(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                ActivityDataDataOneof::MapTravelActivityData(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                ActivityDataDataOneof::RoleSkinTrialActivity(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                ActivityDataDataOneof::ActivityFishingData(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                ActivityDataDataOneof::ActivityWeeklyRogueData(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                ActivityDataDataOneof::SolarisSpeedActivity(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                ActivityDataDataOneof::BabelTowerActivity(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                ActivityDataDataOneof::ActivityBetHorsesData(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                ActivityDataDataOneof::ActivityMapExploreData(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                ActivityDataDataOneof::ActivityPermanentRogueData(v) => {
                    crate::wire::write_submessage(out, 48, v);
                }
                ActivityDataDataOneof::ActivityAvignon(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                ActivityDataDataOneof::DangoAbyssActivityData(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                ActivityDataDataOneof::ActivityInviteNewbie(v) => {
                    crate::wire::write_submessage(out, 51, v);
                }
                ActivityDataDataOneof::ActivityDangoMonopolyData(v) => {
                    crate::wire::write_submessage(out, 52, v);
                }
                ActivityDataDataOneof::ActivityCiacconaGalData(v) => {
                    crate::wire::write_submessage(out, 53, v);
                }
                ActivityDataDataOneof::ActivityLinkageData(v) => {
                    crate::wire::write_submessage(out, 54, v);
                }
                ActivityDataDataOneof::ActivityRegressData(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                ActivityDataDataOneof::CumulativeShopData(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                ActivityDataDataOneof::PhantomArenaActivityData(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                ActivityDataDataOneof::BeginnerCarnivalData(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                ActivityDataDataOneof::ActivityMoraleData(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                ActivityDataDataOneof::FloroRanchActivityData(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                ActivityDataDataOneof::LifePointDrawActivityData(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                ActivityDataDataOneof::ActivityTrapDefenseData(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                ActivityDataDataOneof::ActivityFunPlayData(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                ActivityDataDataOneof::ActivitySoarData(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                ActivityDataDataOneof::ActivityLineCrossData(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                ActivityDataDataOneof::ActivityMoonSignInData(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                ActivityDataDataOneof::HonamiStoryActivityData(v) => {
                    crate::wire::write_submessage(out, 71, v);
                }
                ActivityDataDataOneof::FightPhotoActivityData(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
                ActivityDataDataOneof::SurvivorsActivityData(v) => {
                    crate::wire::write_submessage(out, 73, v);
                }
                ActivityDataDataOneof::ActivityPrizeDrawingData(v) => {
                    crate::wire::write_submessage(out, 74, v);
                }
                ActivityDataDataOneof::AdvertisingPageInfo(v) => {
                    crate::wire::write_submessage(out, 75, v);
                }
                ActivityDataDataOneof::ArtemisActivityInfo(v) => {
                    crate::wire::write_submessage(out, 76, v);
                }
                ActivityDataDataOneof::RoleCoopActivityData(v) => {
                    crate::wire::write_submessage(out, 77, v);
                }
                ActivityDataDataOneof::MotorCycleIpActivityData(v) => {
                    crate::wire::write_submessage(out, 78, v);
                }
                ActivityDataDataOneof::PhantomBattleRecordActivityInfo(v) => {
                    crate::wire::write_submessage(out, 79, v);
                }
                ActivityDataDataOneof::InfrThemeActivityPb(v) => {
                    crate::wire::write_submessage(out, 80, v);
                }
                ActivityDataDataOneof::RoadBookActivityInfo(v) => {
                    crate::wire::write_submessage(out, 81, v);
                }
                ActivityDataDataOneof::PhantomBattleGuideActivity(v) => {
                    crate::wire::write_submessage(out, 82, v);
                }
                ActivityDataDataOneof::NewTowerClimbingActivityData(v) => {
                    crate::wire::write_submessage(out, 83, v);
                }
                ActivityDataDataOneof::NewPlayerSupportActivityData(v) => {
                    crate::wire::write_submessage(out, 84, v);
                }
                ActivityDataDataOneof::MotorParkourActivityInfo(v) => {
                    crate::wire::write_submessage(out, 85, v);
                }
                ActivityDataDataOneof::SpringFestivalActivityInfo(v) => {
                    crate::wire::write_submessage(out, 86, v);
                }
                ActivityDataDataOneof::TotalTopUpActivityInfo(v) => {
                    crate::wire::write_submessage(out, 87, v);
                }
                ActivityDataDataOneof::MotorFightActivityPb(v) => {
                    crate::wire::write_submessage(out, 88, v);
                }
                ActivityDataDataOneof::H5ViewActivityData(v) => {
                    crate::wire::write_submessage(out, 89, v);
                }
                ActivityDataDataOneof::SkinRewardActivityData(v) => {
                    crate::wire::write_submessage(out, 90, v);
                }
                ActivityDataDataOneof::EncircleActivityPb(v) => {
                    crate::wire::write_submessage(out, 91, v);
                }
                ActivityDataDataOneof::MotorDevelopActivityData(v) => {
                    crate::wire::write_submessage(out, 92, v);
                }
                ActivityDataDataOneof::FlagChallengeActivityInfo(v) => {
                    crate::wire::write_submessage(out, 93, v);
                }
                ActivityDataDataOneof::FeiXuePreheatActivityInfo(v) => {
                    crate::wire::write_submessage(out, 94, v);
                }
                ActivityDataDataOneof::RhythmActivityPb(v) => {
                    crate::wire::write_submessage(out, 95, v);
                }
                ActivityDataDataOneof::DropCatchActivityInfo(v) => {
                    crate::wire::write_submessage(out, 96, v);
                }
                ActivityDataDataOneof::TetrisActivityInfo(v) => {
                    crate::wire::write_submessage(out, 97, v);
                }
                ActivityDataDataOneof::PinballActivityData(v) => {
                    crate::wire::write_submessage(out, 98, v);
                }
                ActivityDataDataOneof::BossPilingActivityInfo(v) => {
                    crate::wire::write_submessage(out, 99, v);
                }
                ActivityDataDataOneof::ThemeCelebration(v) => {
                    crate::wire::write_submessage(out, 100, v);
                }
                ActivityDataDataOneof::WuWuWeekActivity(v) => {
                    crate::wire::write_submessage(out, 101, v);
                }
                ActivityDataDataOneof::MotorDecalActivityData(v) => {
                    crate::wire::write_submessage(out, 102, v);
                }
                ActivityDataDataOneof::LinkageCheckInActivityData(v) => {
                    crate::wire::write_submessage(out, 103, v);
                }
                ActivityDataDataOneof::RoleGiftActivityData(v) => {
                    crate::wire::write_submessage(out, 104, v);
                }
                ActivityDataDataOneof::KurotatoActivityData(v) => {
                    crate::wire::write_submessage(out, 105, v);
                }
                ActivityDataDataOneof::GolemCrackActivityInfo(v) => {
                    crate::wire::write_submessage(out, 106, v);
                }
                ActivityDataDataOneof::EdgeRunnerActivityInfo(v) => {
                    crate::wire::write_submessage(out, 107, v);
                }
                ActivityDataDataOneof::OnlineMotorActivityData(v) => {
                    crate::wire::write_submessage(out, 108, v);
                }
                ActivityDataDataOneof::RealmBetweenActivityInfo(v) => {
                    crate::wire::write_submessage(out, 109, v);
                }
                ActivityDataDataOneof::ThroughTrainSummaryActivityData(v) => {
                    crate::wire::write_submessage(out, 110, v);
                }
                ActivityDataDataOneof::NewbieMainActivityPb(v) => {
                    crate::wire::write_submessage(out, 111, v);
                }
                ActivityDataDataOneof::NewbieCourseV2ActivityPb(v) => {
                    crate::wire::write_submessage(out, 112, v);
                }
                ActivityDataDataOneof::NewbieAdventureV2Pb(v) => {
                    crate::wire::write_submessage(out, 113, v);
                }
                ActivityDataDataOneof::NewPlayerSupportActivityV2Pb(v) => {
                    crate::wire::write_submessage(out, 114, v);
                }
                ActivityDataDataOneof::ThroughTrainActivityData(v) => {
                    crate::wire::write_submessage(out, 115, v);
                }
                ActivityDataDataOneof::RoverRogueActivityData(v) => {
                    crate::wire::write_submessage(out, 116, v);
                }
                ActivityDataDataOneof::DangoBrawlActivityInfo(v) => {
                    crate::wire::write_submessage(out, 117, v);
                }
                ActivityDataDataOneof::QingXiaoActivityInfo(v) => {
                    crate::wire::write_submessage(out, 118, v);
                }
                ActivityDataDataOneof::MatchThreeActivityInfo(v) => {
                    crate::wire::write_submessage(out, 120, v);
                }
                ActivityDataDataOneof::GiantActivityInfo(v) => {
                    crate::wire::write_submessage(out, 121, v);
                }
                ActivityDataDataOneof::SolarSchoolActivityPb(v) => {
                    crate::wire::write_submessage(out, 122, v);
                }
                ActivityDataDataOneof::RoleTowerActivityData(v) => {
                    crate::wire::write_submessage(out, 123, v);
                }
                _ => {}
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BeginShowTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginShowTime);
        }
        if self.EndShowTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndShowTime);
        }
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
        if self.EndOpenTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndOpenTime);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.CompletePreQuests {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsFirstOpen {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.FinishConditions {
            crate::wire::write_tag(out, 30, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ActivityOpenType {
            crate::wire::write_tag(out, 33, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsPreOpen {
            crate::wire::write_tag(out, 42, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 67, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 68, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.BeginRewardTimeInternal != 0 {
            crate::wire::write_tag(out, 69, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginRewardTimeInternal);
        }
        if self.EndRewardTimeInternal != 0 {
            crate::wire::write_tag(out, 70, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndRewardTimeInternal);
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
                let val = <ParkourActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ParkourActivity(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <SignActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SignActivity(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewBieCourseActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewBieCourseActivity(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <DoubleInstActivityReward as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DoubleInstActivityReward(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <HarvestActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::HarvestActivity(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTrialInfoActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoleTrialInfoActivity(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomCollectActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PhantomCollectActivity(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <GatherActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::GatherActivityInfo(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <DailyAdventureActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DailyAdventureActivityData(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityRogueData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityRogueData(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityLongShanMain as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityLongShanMain(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityTurnTableData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityTurnTableData(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <BossRushActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::BossRushActivityData(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityTaskData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::TrackMoonActivityTaskData(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityTimePointRewarData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityTimePointRewarData(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <TowerDefenseActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::TowerDefenseActivityInfo(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <CircumFluenceTaskData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::CircumFluenceTaskData(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityRoleGiveData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityRoleGiveData(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityCorniceMeetingData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityCorniceMeetingData(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <RiskHarvestActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RiskHarvestActivityData(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityBlackCoastData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityBlackCoastData(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <DreamLinkActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DreamLinkActivityData(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityScratchTicketData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityScratchTicketData(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <PreheatSignActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PreheatSignActivityData(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <MowTowerActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MowTowerActivityData(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <FarmGoldData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::FarmGoldData(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <SpringSignData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SpringSignData(val));
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <MapTravelActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MapTravelActivityData(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleSkinTrialActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoleSkinTrialActivity(val));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityFishingData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityFishingData(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityWeeklyRogueData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityWeeklyRogueData(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <SolarisSpeedActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SolarisSpeedActivity(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <BabelTowerActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::BabelTowerActivity(val));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityBetHorsesData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityBetHorsesData(val));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityMapExploreData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityMapExploreData(val));
            }
            48 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityPermanentRogueData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityPermanentRogueData(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityAvignon as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityAvignon(val));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <DangoAbyssActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DangoAbyssActivityData(val));
            }
            51 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityInviteNewbie as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityInviteNewbie(val));
            }
            52 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityDangoMonopolyData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityDangoMonopolyData(val));
            }
            53 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityCiacconaGalData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityCiacconaGalData(val));
            }
            54 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityLinkageData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityLinkageData(val));
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityRegressData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityRegressData(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <CumulativeShopData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::CumulativeShopData(val));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomArenaActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PhantomArenaActivityData(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <BeginnerCarnivalData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::BeginnerCarnivalData(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityMoraleData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityMoraleData(val));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <FloroRanchActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::FloroRanchActivityData(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <LifePointDrawActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::LifePointDrawActivityData(val));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityTrapDefenseData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityTrapDefenseData(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityFunPlayData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityFunPlayData(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivitySoarData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivitySoarData(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityLineCrossData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityLineCrossData(val));
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityMoonSignInData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityMoonSignInData(val));
            }
            71 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::HonamiStoryActivityData(val));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <FightPhotoActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::FightPhotoActivityData(val));
            }
            73 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SurvivorsActivityData(val));
            }
            74 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityPrizeDrawingData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ActivityPrizeDrawingData(val));
            }
            75 => {
                let bytes = r.read_len_delimited()?;
                let val = <AdvertisingPageInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::AdvertisingPageInfo(val));
            }
            76 => {
                let bytes = r.read_len_delimited()?;
                let val = <ArtemisActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ArtemisActivityInfo(val));
            }
            77 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleCoopActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoleCoopActivityData(val));
            }
            78 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorCycleIpActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MotorCycleIpActivityData(val));
            }
            79 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomArenaActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PhantomBattleRecordActivityInfo(val));
            }
            80 => {
                let bytes = r.read_len_delimited()?;
                let val = <InfrThemeActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::InfrThemeActivityPb(val));
            }
            81 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoadBookActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoadBookActivityInfo(val));
            }
            82 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomBattleGuideActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PhantomBattleGuideActivity(val));
            }
            83 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewTowerClimbingActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewTowerClimbingActivityData(val));
            }
            84 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewPlayerSupportActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewPlayerSupportActivityData(val));
            }
            85 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorParkourActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MotorParkourActivityInfo(val));
            }
            86 => {
                let bytes = r.read_len_delimited()?;
                let val = <SpringFestivalActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SpringFestivalActivityInfo(val));
            }
            87 => {
                let bytes = r.read_len_delimited()?;
                let val = <TotalTopUpActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::TotalTopUpActivityInfo(val));
            }
            88 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorFightActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MotorFightActivityPb(val));
            }
            89 => {
                let bytes = r.read_len_delimited()?;
                let val = <H5ViewActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::H5ViewActivityData(val));
            }
            90 => {
                let bytes = r.read_len_delimited()?;
                let val = <SkinRewardActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SkinRewardActivityData(val));
            }
            91 => {
                let bytes = r.read_len_delimited()?;
                let val = <EncircleActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::EncircleActivityPb(val));
            }
            92 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorDevelopActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MotorDevelopActivityData(val));
            }
            93 => {
                let bytes = r.read_len_delimited()?;
                let val = <FlagChallengeActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::FlagChallengeActivityInfo(val));
            }
            94 => {
                let bytes = r.read_len_delimited()?;
                let val = <FeiXuePreheatActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::FeiXuePreheatActivityInfo(val));
            }
            95 => {
                let bytes = r.read_len_delimited()?;
                let val = <RhythmActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RhythmActivityPb(val));
            }
            96 => {
                let bytes = r.read_len_delimited()?;
                let val = <DropCatchActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DropCatchActivityInfo(val));
            }
            97 => {
                let bytes = r.read_len_delimited()?;
                let val = <TetrisActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::TetrisActivityInfo(val));
            }
            98 => {
                let bytes = r.read_len_delimited()?;
                let val = <PinballActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::PinballActivityData(val));
            }
            99 => {
                let bytes = r.read_len_delimited()?;
                let val = <BossPilingActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::BossPilingActivityInfo(val));
            }
            100 => {
                let bytes = r.read_len_delimited()?;
                let val = <ThemeCelebration as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ThemeCelebration(val));
            }
            101 => {
                let bytes = r.read_len_delimited()?;
                let val = <WuWuWeekActivity as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::WuWuWeekActivity(val));
            }
            102 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorDecalActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MotorDecalActivityData(val));
            }
            103 => {
                let bytes = r.read_len_delimited()?;
                let val = <LinkageCheckInActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::LinkageCheckInActivityData(val));
            }
            104 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleGiftActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoleGiftActivityData(val));
            }
            105 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::KurotatoActivityData(val));
            }
            106 => {
                let bytes = r.read_len_delimited()?;
                let val = <GolemCrackActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::GolemCrackActivityInfo(val));
            }
            107 => {
                let bytes = r.read_len_delimited()?;
                let val = <EdgeRunnerActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::EdgeRunnerActivityInfo(val));
            }
            108 => {
                let bytes = r.read_len_delimited()?;
                let val = <OnlineMotorActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::OnlineMotorActivityData(val));
            }
            109 => {
                let bytes = r.read_len_delimited()?;
                let val = <RealmBetweenActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RealmBetweenActivityInfo(val));
            }
            110 => {
                let bytes = r.read_len_delimited()?;
                let val = <ThroughTrainSummaryActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ThroughTrainSummaryActivityData(val));
            }
            111 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewbieMainActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewbieMainActivityPb(val));
            }
            112 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewbieCourseV2ActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewbieCourseV2ActivityPb(val));
            }
            113 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewbieAdventureV2Pb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewbieAdventureV2Pb(val));
            }
            114 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewPlayerSupportActivityV2Pb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::NewPlayerSupportActivityV2Pb(val));
            }
            115 => {
                let bytes = r.read_len_delimited()?;
                let val = <ThroughTrainActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::ThroughTrainActivityData(val));
            }
            116 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoverRogueActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoverRogueActivityData(val));
            }
            117 => {
                let bytes = r.read_len_delimited()?;
                let val = <DangoBrawlActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::DangoBrawlActivityInfo(val));
            }
            118 => {
                let bytes = r.read_len_delimited()?;
                let val = <QingXiaoActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::QingXiaoActivityInfo(val));
            }
            120 => {
                let bytes = r.read_len_delimited()?;
                let val = <MatchThreeActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::MatchThreeActivityInfo(val));
            }
            121 => {
                let bytes = r.read_len_delimited()?;
                let val = <GiantActivityInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::GiantActivityInfo(val));
            }
            122 => {
                let bytes = r.read_len_delimited()?;
                let val = <SolarSchoolActivityPb as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::SolarSchoolActivityPb(val));
            }
            123 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTowerActivityData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityDataDataOneof::RoleTowerActivityData(val));
            }
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Type = Some(<ActivityType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.BeginShowTime = r.read_varint_i64()?,
            4 => self.EndShowTime = r.read_varint_i64()?,
            5 => self.BeginOpenTime = r.read_varint_i64()?,
            6 => self.EndOpenTime = r.read_varint_i64()?,
            7 => self.IsUnlock = r.read_bool()?,
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompletePreQuests,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => self.IsFirstOpen = r.read_bool()?,
            30 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishConditions,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            33 => {
                self.ActivityOpenType = Some(
                    <ActivityOpenType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            42 => self.IsPreOpen = r.read_bool()?,
            67 => self.StartTime = r.read_varint_i64()?,
            68 => self.EndTime = r.read_varint_i64()?,
            69 => self.BeginRewardTimeInternal = r.read_varint_i64()?,
            70 => self.EndRewardTimeInternal = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityRequest {}

impl ActivityRequest {
    pub const MSG_ID: u16 = 16076;
}

impl PbMessage for ActivityRequest {
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
pub struct ActivityResponse {
    pub Activities: Vec<ActivityData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ActivityResponse {
    pub const MSG_ID: u16 = 15999;
}

impl PbMessage for ActivityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Activities {
            crate::wire::write_submessage(out, 6, item);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Activities
                    .push(<ActivityData as PbMessage>::decode(bytes)?);
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
pub struct ActivityTurnTableData {
    pub IsAllFinish: bool,
    pub GroupId: i32,
    pub Rewards: Vec<i32>,
    pub TurntableTasks: Vec<ActivityTask>,
}

impl PbMessage for ActivityTurnTableData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAllFinish {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        for item in &self.Rewards {
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
            2 => self.GroupId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Rewards, |sr| {
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
pub struct ActivityCorniceMeetingLevelEntryData {
    pub MaxScore: i32,
    pub RemainTime: i32,
    pub UnlockTime: i64,
    pub RewardedMap: Vec<i32>,
}

impl PbMessage for ActivityCorniceMeetingLevelEntryData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.RemainTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainTime);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        for item in &self.RewardedMap {
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
            1 => self.MaxScore = r.read_varint_i32()?,
            2 => self.RemainTime = r.read_varint_i32()?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedMap, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeStageInfo {
    pub Id: i32,
    pub Tasks: Vec<ActivityTask>,
}

impl PbMessage for BlackCoastThemeStageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Tasks {
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
                self.Tasks.push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityBlackCoastData {
    pub StageData: Vec<BlackCoastThemeStageInfo>,
    pub RewardIds: Vec<i32>,
}

impl PbMessage for ActivityBlackCoastData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StageData {
            crate::wire::write_submessage(out, 1, item);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.StageData
                    .push(<BlackCoastThemeStageInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DreamLinkActivityData {
    pub MaxEnergy: i32,
    pub SignStateList: Vec<SignState>,
    pub RoleInstanceList: Vec<RoleInstanceList>,
    pub LevelPlayList: Vec<LevelPlayList>,
    pub BossRewardIds: Vec<SignState>,
    pub AllLimitTimeReward: Vec<AllLimitTimeReward>,
    pub ScoreMap: BTreeMap<i32, i32>,
    pub LimitTimeReward: i64,
    pub LimitTimeEnd: i64,
    pub RogueBossInstData: Vec<RogueBossInstData>,
    pub PlayTime: i32,
    pub UnlockButtons: Vec<i32>,
}

impl PbMessage for DreamLinkActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxEnergy != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxEnergy);
        }
        for item in &self.SignStateList {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        for item in &self.RoleInstanceList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.LevelPlayList {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.BossRewardIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        for item in &self.AllLimitTimeReward {
            crate::wire::write_submessage(out, 8, item);
        }
        if !self.ScoreMap.is_empty() {
            crate::wire::write_map(out, 9, &self.ScoreMap);
        }
        if self.LimitTimeReward != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitTimeReward);
        }
        if self.LimitTimeEnd != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitTimeEnd);
        }
        for item in &self.RogueBossInstData {
            crate::wire::write_submessage(out, 12, item);
        }
        if self.PlayTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayTime);
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
                self.RoleInstanceList
                    .push(<RoleInstanceList as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.LevelPlayList
                    .push(<LevelPlayList as PbMessage>::decode(bytes)?);
            }
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BossRewardIds, |sr| {
                    Ok(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                })?
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AllLimitTimeReward
                    .push(<AllLimitTimeReward as PbMessage>::decode(bytes)?);
            }
            9 => crate::wire::decode_map_entry(r, &mut self.ScoreMap)?,
            10 => self.LimitTimeReward = r.read_varint_i64()?,
            11 => self.LimitTimeEnd = r.read_varint_i64()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RogueBossInstData
                    .push(<RogueBossInstData as PbMessage>::decode(bytes)?);
            }
            13 => self.PlayTime = r.read_varint_i32()?,
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
pub struct AllLimitTimeReward {
    pub SignState: Option<SignState>,
    pub CurProgress: i32,
    pub Target: i32,
    pub ConfigId: i32,
}

impl PbMessage for AllLimitTimeReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SignState {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurProgress);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Target);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
                self.SignState = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.CurProgress = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleInstanceList {
    pub InstId: i32,
    pub IsUnlock: bool,
    pub CanUnlock: bool,
}

impl PbMessage for RoleInstanceList {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.CanUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueBossInstData {
    pub InstId: i32,
    pub IsUnlock: bool,
    pub CanUnlock: bool,
    pub UnlockTime: i64,
}

impl PbMessage for RogueBossInstData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanUnlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.CanUnlock = r.read_bool()?,
            4 => self.UnlockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayList {
    pub Index: i32,
    pub LevelPlayId: i32,
    pub State: Option<SignState>,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub PlayTime: i32,
}

impl PbMessage for LevelPlayList {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.PlayTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayTime);
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
            2 => self.LevelPlayId = r.read_varint_i32()?,
            3 => {
                self.State = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.IsUnlock = r.read_bool()?,
            5 => self.UnlockTime = r.read_varint_i64()?,
            6 => self.PlayTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchTicketRoundData {
    pub RoundId: i32,
    pub UnlockTime: i64,
    pub AreaStageRewardDataList: BTreeMap<i32, ScratchCardRewardData>,
    pub LeftRewardItem: BTreeMap<i32, i32>,
}

impl PbMessage for ScratchTicketRoundData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if !self.AreaStageRewardDataList.is_empty() {
            crate::wire::write_map(out, 3, &self.AreaStageRewardDataList);
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
            3 => crate::wire::decode_map_entry(r, &mut self.AreaStageRewardDataList)?,
            4 => crate::wire::decode_map_entry(r, &mut self.LeftRewardItem)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardRewardData {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for ScratchCardRewardData {
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
pub struct ScratchTicketConditionData {
    pub Id: i32,
    pub Progress: i32,
    pub FinishedAchievementNum: i32,
}

impl PbMessage for ScratchTicketConditionData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.FinishedAchievementNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinishedAchievementNum);
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
            3 => self.FinishedAchievementNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityScratchTicketData {
    pub RoundData: Vec<ScratchTicketRoundData>,
    pub ConditionData: Vec<ScratchTicketConditionData>,
}

impl PbMessage for ActivityScratchTicketData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoundData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ConditionData {
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
                self.RoundData
                    .push(<ScratchTicketRoundData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionData
                    .push(<ScratchTicketConditionData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignNodeInfo {
    pub PreheatNodeId: i32,
    pub UnlockTime: i64,
    pub Rewarded: bool,
}

impl PbMessage for PreheatSignNodeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PreheatNodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreheatNodeId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
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
            1 => self.PreheatNodeId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.Rewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignActivityData {
    pub PreheatSignNodeInfos: Vec<PreheatSignNodeInfo>,
}

impl PbMessage for PreheatSignActivityData {
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
pub struct SpringSignData {
    pub SpringSignActivityTasks: Vec<ActivityTask>,
    pub CanInvite: bool,
    pub DrawRoles: Vec<i32>,
    pub SkinReward: bool,
}

impl PbMessage for SpringSignData {
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
pub struct MowTowerLevelsInfo {
    pub BabelTowerLevelId: i32,
    pub UnlockTime: i64,
    pub IsUnlock: bool,
    pub FirstScore: i32,
    pub SecondScore: i32,
    pub LevelRewardStatus: BTreeMap<i32, MowTowerRewardStatus>,
    pub HardLevelBuffs: Vec<i32>,
    pub FirstRoleSelection: Vec<i32>,
    pub SecondRoleSelection: Vec<i32>,
}

impl PbMessage for MowTowerLevelsInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BabelTowerLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BabelTowerLevelId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FirstScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstScore);
        }
        if self.SecondScore != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondScore);
        }
        if !self.LevelRewardStatus.is_empty() {
            crate::wire::write_map(out, 5, &self.LevelRewardStatus);
        }
        for item in &self.HardLevelBuffs {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FirstRoleSelection {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SecondRoleSelection {
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
            1 => self.BabelTowerLevelId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.IsUnlock = r.read_bool()?,
            4 => self.FirstScore = r.read_varint_i32()?,
            9 => self.SecondScore = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.LevelRewardStatus)?,
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HardLevelBuffs, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FirstRoleSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SecondRoleSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerActivityData {
    pub MowTowerLevelsInfo: Vec<MowTowerLevelsInfo>,
}

impl PbMessage for MowTowerActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MowTowerLevelsInfo {
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
                self.MowTowerLevelsInfo
                    .push(<MowTowerLevelsInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThroughTrainActivityData {
    pub IsFinish: bool,
}

impl PbMessage for ThroughTrainActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFinish {
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
            1 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThroughTrainSummaryActivityData {
    pub ActivityIds: Vec<i32>,
    pub CompletedActivityIds: Vec<i32>,
}

impl PbMessage for ThroughTrainSummaryActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.CompletedActivityIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActivityIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompletedActivityIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinTrialContentData {
    pub Id: i32,
    pub ChallengeState: Option<ChallengeState>,
}

impl PbMessage for RoleSkinTrialContentData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            1 => self.Id = r.read_varint_i32()?,
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
pub struct RoleSkinTrialActivity {
    pub RoleSkinTrialContentData: Vec<RoleSkinTrialContentData>,
}

impl PbMessage for RoleSkinTrialActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleSkinTrialContentData {
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
                self.RoleSkinTrialContentData
                    .push(<RoleSkinTrialContentData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityWeeklyRogueDataDataOneof {
    RogueWeeklyLastInfo(RogueWeeklyLastInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityWeeklyRogueData {
    pub Data: Option<ActivityWeeklyRogueDataDataOneof>,
    pub CycleId: i32,
    pub Score: i32,
    pub RogueWeeklyAward: Vec<RogueWeeklyAward>,
    pub MaxScore: i32,
    pub CurWorldLevel: i32,
    pub UseFreeCount: i32,
    pub MaxFreeCount: i32,
}

impl PbMessage for ActivityWeeklyRogueData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                ActivityWeeklyRogueDataDataOneof::RogueWeeklyLastInfo(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        for item in &self.RogueWeeklyAward {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <RogueWeeklyLastInfo as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityWeeklyRogueDataDataOneof::RogueWeeklyLastInfo(val));
            }
            1 => self.CycleId = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyAward
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
pub struct RogueWeeklyAward {
    pub SignState: Option<SignState>,
    pub CurProgress: i32,
    pub MaxProgress: i32,
    pub ConfigId: i32,
}

impl PbMessage for RogueWeeklyAward {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SignState {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurProgress);
        }
        if self.MaxProgress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxProgress);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
                self.SignState = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.CurProgress = r.read_varint_i32()?,
            3 => self.MaxProgress = r.read_varint_i32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyLastInfo {
    pub InstId: i32,
    pub CurLayer: i32,
    pub MaxLayer: i32,
    pub WorldLevel: i32,
}

impl PbMessage for RogueWeeklyLastInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.WorldLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.CurLayer = r.read_varint_i32()?,
            3 => self.MaxLayer = r.read_varint_i32()?,
            4 => self.WorldLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityPermanentRogueData {
    pub PermanentSeasonData: Vec<PermanentSeasonData>,
    pub RogueResTaskData: Option<RogueResTaskData>,
}

impl PbMessage for ActivityPermanentRogueData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PermanentSeasonData {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.RogueResTaskData {
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
                self.PermanentSeasonData
                    .push(<PermanentSeasonData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PermanentSeasonData {
    pub PermanentSeasonDataId: i32,
    pub SkillDict: BTreeMap<i32, i32>,
    pub RogueResEndId: Vec<i32>,
    pub RogueResEndAward: Vec<ActivityTask>,
    pub TrialRoleIds: Vec<i32>,
    pub RoleIds: Vec<i32>,
    pub EndTime: i64,
    pub ShopItemCount: i32,
}

impl PbMessage for PermanentSeasonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PermanentSeasonDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PermanentSeasonDataId);
        }
        if !self.SkillDict.is_empty() {
            crate::wire::write_map(out, 2, &self.SkillDict);
        }
        for item in &self.RogueResEndId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RogueResEndAward {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.TrialRoleIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.ShopItemCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopItemCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PermanentSeasonDataId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.SkillDict)?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RogueResEndId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResEndAward
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TrialRoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => self.EndTime = r.read_varint_i64()?,
            8 => self.ShopItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTaskData {
    pub PermanentRogueData: Option<PermanentRogueData>,
    pub RogueResCollectionState: BTreeMap<i32, SignState>,
}

impl PbMessage for RogueResTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PermanentRogueData {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.RogueResCollectionState.is_empty() {
            crate::wire::write_map(out, 2, &self.RogueResCollectionState);
        }
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
                self.PermanentRogueData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_map_entry(r, &mut self.RogueResCollectionState)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermanentRogueDataDataOneof {
    RogueResTaskThemeData(RogueResTaskThemeData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PermanentRogueData {
    pub Data: Option<PermanentRogueDataDataOneof>,
}

impl PermanentRogueData {
    pub const MSG_ID: u16 = 24660;
}

impl PbMessage for PermanentRogueData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                PermanentRogueDataDataOneof::RogueResTaskThemeData(v) => {
                    crate::wire::write_submessage(out, 13, v);
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
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <RogueResTaskThemeData as PbMessage>::decode(bytes)?;
                self.Data = Some(PermanentRogueDataDataOneof::RogueResTaskThemeData(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTaskThemeData {
    pub RogueSignReward: Vec<ActivityTask>,
    pub RogueResThemeId: i32,
    pub EndTime: i64,
}

impl PbMessage for RogueResTaskThemeData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueSignReward {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RogueResThemeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RogueResThemeId);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueSignReward
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => self.RogueResThemeId = r.read_varint_i32()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityFishingData {
    pub ActivityTaskData: Vec<ActivityTask>,
    pub MilestoneReward: BTreeMap<i32, SignState>,
    pub LimitTimeReward: i64,
    pub LimitTimeEnd: i64,
    pub MilestoneRewardItemAccumulate: i32,
}

impl PbMessage for ActivityFishingData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTaskData {
            crate::wire::write_submessage(out, 1, item);
        }
        if !self.MilestoneReward.is_empty() {
            crate::wire::write_map(out, 2, &self.MilestoneReward);
        }
        if self.LimitTimeReward != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitTimeReward);
        }
        if self.LimitTimeEnd != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitTimeEnd);
        }
        if self.MilestoneRewardItemAccumulate != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MilestoneRewardItemAccumulate);
        }
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
                self.ActivityTaskData
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_map_entry(r, &mut self.MilestoneReward)?,
            3 => self.LimitTimeReward = r.read_varint_i64()?,
            4 => self.LimitTimeEnd = r.read_varint_i64()?,
            5 => self.MilestoneRewardItemAccumulate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerData {
    pub BabelTowerLevelId: i32,
    pub UnlockTime: i64,
    pub NormalLevelBuffs: Vec<i32>,
    pub RoleIds: Vec<i32>,
    pub HardLevelBuffs: Vec<i32>,
    pub HardLevelItems: Vec<i32>,
    pub HardLevelStar: i32,
    pub HasPassed: bool,
    pub MaxPassRoleSelection: Vec<i32>,
    pub MaxPassBuffSelection: Vec<i32>,
    pub MaxPassStar: i32,
    pub SkillBranchId: Vec<i32>,
    pub MaxPassUseTime: i32,
}

impl PbMessage for BabelTowerData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BabelTowerLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BabelTowerLevelId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        for item in &self.NormalLevelBuffs {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.HardLevelBuffs {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.HardLevelItems {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.HardLevelStar != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HardLevelStar);
        }
        if self.HasPassed {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.MaxPassRoleSelection {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MaxPassBuffSelection {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MaxPassStar != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxPassStar);
        }
        for item in &self.SkillBranchId {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MaxPassUseTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxPassUseTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BabelTowerLevelId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.NormalLevelBuffs,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HardLevelBuffs, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HardLevelItems, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            7 => self.HardLevelStar = r.read_varint_i32()?,
            8 => self.HasPassed = r.read_bool()?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MaxPassRoleSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MaxPassBuffSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => self.MaxPassStar = r.read_varint_i32()?,
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => self.MaxPassUseTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelDebuff {
    pub BuffId: i32,
    pub Unlocked: bool,
}

impl PbMessage for BabelDebuff {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
        }
        if self.Unlocked {
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
            1 => self.BuffId = r.read_varint_i32()?,
            2 => self.Unlocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelBuff {
    pub BuffId: i32,
    pub Unlocked: bool,
}

impl PbMessage for BabelBuff {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
        }
        if self.Unlocked {
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
            1 => self.BuffId = r.read_varint_i32()?,
            2 => self.Unlocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerActivity {
    pub BabelTowerDataList: Vec<BabelTowerData>,
    pub BabelDebuffUnlocks: Vec<BabelDebuff>,
    pub BabelBuffUnlocks: Vec<BabelBuff>,
    pub NormalQuest: Vec<ActivityTask>,
    pub DailyQuest: Vec<ActivityTask>,
    pub CurrentItemCount: i32,
    pub ShowName: bool,
}

impl PbMessage for BabelTowerActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BabelTowerDataList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BabelDebuffUnlocks {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.BabelBuffUnlocks {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.NormalQuest {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.DailyQuest {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.CurrentItemCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentItemCount);
        }
        if self.ShowName {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.BabelTowerDataList
                    .push(<BabelTowerData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BabelDebuffUnlocks
                    .push(<BabelDebuff as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BabelBuffUnlocks
                    .push(<BabelBuff as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.NormalQuest
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DailyQuest
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            6 => self.CurrentItemCount = r.read_varint_i32()?,
            7 => self.ShowName = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityMapExploreData {
    pub ActivityTasks: BTreeMap<i32, ActivityTaskState>,
}

impl PbMessage for ActivityMapExploreData {
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
pub struct ActivityInviteNewbie {
    pub InviteCode: Vec<u8>,
    pub Score: i32,
    pub RedDot: bool,
    pub AllRewardClaimed: bool,
}

impl PbMessage for ActivityInviteNewbie {
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
pub struct ActivityDangoMonopolyData {
    pub CurrentBoardId: i32,
    pub CurrentGridId: i32,
    pub RewardGridId: i32,
    pub BoardRewards: Vec<i32>,
    pub DangoTaskConfig: Vec<DangoMonopolyConfig>,
    pub TaskEndTimeMap: BTreeMap<i32, i64>,
    pub UnlockTime: i64,
    pub BoardMap: BTreeMap<i32, DangoMonopolyBoardData>,
}

impl PbMessage for ActivityDangoMonopolyData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrentBoardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentBoardId);
        }
        if self.CurrentGridId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentGridId);
        }
        if self.RewardGridId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardGridId);
        }
        for item in &self.BoardRewards {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DangoTaskConfig {
            crate::wire::write_submessage(out, 5, item);
        }
        if !self.TaskEndTimeMap.is_empty() {
            crate::wire::write_map(out, 6, &self.TaskEndTimeMap);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if !self.BoardMap.is_empty() {
            crate::wire::write_map(out, 8, &self.BoardMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurrentBoardId = r.read_varint_i32()?,
            2 => self.CurrentGridId = r.read_varint_i32()?,
            3 => self.RewardGridId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BoardRewards, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DangoTaskConfig
                    .push(<DangoMonopolyConfig as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_map_entry(r, &mut self.TaskEndTimeMap)?,
            7 => self.UnlockTime = r.read_varint_i64()?,
            8 => crate::wire::decode_map_entry(r, &mut self.BoardMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyBoardData {
    pub PropertyIds: Vec<i32>,
    pub RecordDiceRollTimes: i32,
    pub RecordTriggerMap: BTreeMap<i32, i32>,
}

impl PbMessage for DangoMonopolyBoardData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PropertyIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RecordDiceRollTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecordDiceRollTimes);
        }
        if !self.RecordTriggerMap.is_empty() {
            crate::wire::write_map(out, 3, &self.RecordTriggerMap);
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
            2 => self.RecordDiceRollTimes = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.RecordTriggerMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyConfig {
    pub TaskId: i32,
    pub ActivityTaskState: Option<DangoMonopolyTaskState>,
    pub Progress: i32,
    pub TargetProgress: i32,
}

impl PbMessage for DangoMonopolyConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if let Some(v) = &self.ActivityTaskState {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetProgress);
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
                self.ActivityTaskState = Some(
                    <DangoMonopolyTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Progress = r.read_varint_i32()?,
            4 => self.TargetProgress = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityCiacconaGalData {
    pub ChapterData: Vec<CiacconaGalChapterData>,
    pub ProgressRewardData: Vec<CiacconaGalRewardData>,
    pub EndingData: Vec<CiacconaGalEndingData>,
    pub CiacconaGalInspirationData: Option<CiacconaGalInspirationData>,
    pub State2Unlock: bool,
    pub State3Unlock: bool,
    pub RewardStartTime: i64,
    pub RewardEndTime: i64,
}

impl PbMessage for ActivityCiacconaGalData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChapterData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ProgressRewardData {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.EndingData {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.CiacconaGalInspirationData {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.State2Unlock {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.State3Unlock {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardStartTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RewardStartTime);
        }
        if self.RewardEndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RewardEndTime);
        }
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
                self.ChapterData
                    .push(<CiacconaGalChapterData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ProgressRewardData
                    .push(<CiacconaGalRewardData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.EndingData
                    .push(<CiacconaGalEndingData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CiacconaGalInspirationData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.State2Unlock = r.read_bool()?,
            6 => self.State3Unlock = r.read_bool()?,
            7 => self.RewardStartTime = r.read_varint_i64()?,
            8 => self.RewardEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalChapterData {
    pub ChapterDataId: i32,
    pub CanUnlock: bool,
    pub CiacconaGalSubEndingData: Vec<CiacconaGalSubEndingData>,
    pub CiacconaGalChoiceData: Vec<CiacconaGalChoiceData>,
}

impl PbMessage for CiacconaGalChapterData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChapterDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterDataId);
        }
        if self.CanUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.CiacconaGalSubEndingData {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.CiacconaGalChoiceData {
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
            1 => self.ChapterDataId = r.read_varint_i32()?,
            2 => self.CanUnlock = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CiacconaGalSubEndingData
                    .push(<CiacconaGalSubEndingData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CiacconaGalChoiceData
                    .push(<CiacconaGalChoiceData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalChoiceData {
    pub ChoiceDataId: i32,
    pub SecondState: bool,
    pub FirstState: bool,
}

impl PbMessage for CiacconaGalChoiceData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChoiceDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChoiceDataId);
        }
        if self.SecondState {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FirstState {
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
            1 => self.ChoiceDataId = r.read_varint_i32()?,
            2 => self.SecondState = r.read_bool()?,
            3 => self.FirstState = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalSubEndingData {
    pub SubEndingDataId: i32,
    pub IsFinished: bool,
    pub IsRewarded: bool,
}

impl PbMessage for CiacconaGalSubEndingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubEndingDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubEndingDataId);
        }
        if self.IsFinished {
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
            1 => self.SubEndingDataId = r.read_varint_i32()?,
            2 => self.IsFinished = r.read_bool()?,
            3 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalRewardData {
    pub RewardDataId: i32,
    pub CanReceive: bool,
    pub IsRewarded: bool,
}

impl PbMessage for CiacconaGalRewardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardDataId);
        }
        if self.CanReceive {
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
            1 => self.RewardDataId = r.read_varint_i32()?,
            2 => self.CanReceive = r.read_bool()?,
            3 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalInspirationData {
    pub InspirationCount: i32,
    pub RefreshTime: i64,
}

impl PbMessage for CiacconaGalInspirationData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InspirationCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InspirationCount);
        }
        if self.RefreshTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RefreshTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.InspirationCount = r.read_varint_i32()?,
            2 => self.RefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaGalEndingData {
    pub SubEndingDataId: i32,
    pub IsRewarded: bool,
}

impl PbMessage for CiacconaGalEndingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubEndingDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubEndingDataId);
        }
        if self.IsRewarded {
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
            1 => self.SubEndingDataId = r.read_varint_i32()?,
            2 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaActivityData {
    pub PhantomArenaChallengeInfoList: Vec<PhantomArenaChallengeInfo>,
    pub ActivityTasks: Vec<ActivityTask>,
    pub PhantomArenaMasterInfo: Option<PhantomArenaMasterInfo>,
    pub BadgeInfo: Vec<PhantomArenaBadge>,
    pub BadgeReward: Vec<PhantomArenaBadgeReward>,
    pub CardList: Vec<PhantomArenaCardInfo>,
    pub CardReward: Vec<PhantomArenaCardReward>,
    pub RoleInfo: Vec<PhantomArenaRoleInfo>,
    pub DeckInfo: Vec<PhantomArenaDeckInfo>,
    pub TimeLimitShopEndTime: i64,
}

impl PbMessage for PhantomArenaActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomArenaChallengeInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.PhantomArenaMasterInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.BadgeInfo {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.BadgeReward {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.CardList {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.CardReward {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.RoleInfo {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.DeckInfo {
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
                self.PhantomArenaChallengeInfoList
                    .push(<PhantomArenaChallengeInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomArenaMasterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BadgeInfo
                    .push(<PhantomArenaBadge as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.BadgeReward
                    .push(<PhantomArenaBadgeReward as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CardList
                    .push(<PhantomArenaCardInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.CardReward
                    .push(<PhantomArenaCardReward as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .push(<PhantomArenaRoleInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.DeckInfo
                    .push(<PhantomArenaDeckInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.TimeLimitShopEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaChallengeInfo {
    pub ChallengeInfoId: i32,
    pub IsUnlock: bool,
    pub CanReChallenge: bool,
    pub LastCardRoleId: i32,
    pub LastCardGroupIndex: i32,
    pub FinishConditions: Vec<i32>,
    pub IsUncover: bool,
    pub IsShow: bool,
}

impl PbMessage for PhantomArenaChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeInfoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeInfoId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanReChallenge {
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
            1 => self.ChallengeInfoId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.CanReChallenge = r.read_bool()?,
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
pub struct PhantomArenaMasterInfo {
    pub MasterLevel: i32,
    pub MasterExp: i32,
    pub RewardTaken: Vec<i32>,
    pub MasterWeeklyExp: i32,
    pub LastUsedDeckServerId: i32,
    pub LastUsedCardRoleId: i32,
}

impl PbMessage for PhantomArenaMasterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MasterLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MasterLevel);
        }
        if self.MasterExp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MasterExp);
        }
        for item in &self.RewardTaken {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MasterWeeklyExp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MasterWeeklyExp);
        }
        if self.LastUsedDeckServerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastUsedDeckServerId);
        }
        if self.LastUsedCardRoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastUsedCardRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MasterLevel = r.read_varint_i32()?,
            2 => self.MasterExp = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardTaken, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.MasterWeeklyExp = r.read_varint_i32()?,
            5 => self.LastUsedDeckServerId = r.read_varint_i32()?,
            6 => self.LastUsedCardRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaBadge {
    pub BadgeId: i32,
    pub IsUnlock: bool,
}

impl PbMessage for PhantomArenaBadge {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BadgeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BadgeId);
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
            1 => self.BadgeId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaBadgeReward {
    pub BadgeRewardId: i32,
    pub NeedCount: i32,
    pub IsTaken: bool,
}

impl PbMessage for PhantomArenaBadgeReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BadgeRewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BadgeRewardId);
        }
        if self.NeedCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NeedCount);
        }
        if self.IsTaken {
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
            1 => self.BadgeRewardId = r.read_varint_i32()?,
            2 => self.NeedCount = r.read_varint_i32()?,
            3 => self.IsTaken = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaCardInfo {
    pub CardId: i32,
    pub IsUnlock: bool,
    pub IsCardOutlookUnlock: bool,
}

impl PbMessage for PhantomArenaCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsCardOutlookUnlock {
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
            1 => self.CardId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsCardOutlookUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaCardReward {
    pub CardId: i32,
    pub NeedCount: i32,
    pub IsTaken: bool,
}

impl PbMessage for PhantomArenaCardReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
        }
        if self.NeedCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NeedCount);
        }
        if self.IsTaken {
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
            1 => self.CardId = r.read_varint_i32()?,
            2 => self.NeedCount = r.read_varint_i32()?,
            3 => self.IsTaken = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaRoleInfo {
    pub RoleInfoId: i32,
    pub IsUnlock: bool,
    pub IsTaken: bool,
}

impl PbMessage for PhantomArenaRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleInfoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleInfoId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsTaken {
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
            1 => self.RoleInfoId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsTaken = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomArenaDeckInfo {
    pub Name: Vec<u8>,
    pub BattleCardIds: Vec<i32>,
    pub CanUse: bool,
    pub LastUseChallengeId: Vec<i32>,
    pub Index: i32,
    pub SkillUnlockInfos: Vec<PhantomBattleCardSkillUnlockInfo>,
}

impl PbMessage for PhantomArenaDeckInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        for item in &self.BattleCardIds {
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
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BattleCardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
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
pub struct PhantomBattleGuideActivity {
    pub QuestId: i32,
    pub DropId: i32,
    pub RewardTotalNum: i32,
    pub SendReward: bool,
    pub RecordActId: i32,
}

impl PbMessage for PhantomBattleGuideActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
        if self.DropId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropId);
        }
        if self.RewardTotalNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardTotalNum);
        }
        if self.SendReward {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RecordActId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecordActId);
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
            2 => self.DropId = r.read_varint_i32()?,
            3 => self.RewardTotalNum = r.read_varint_i32()?,
            4 => self.SendReward = r.read_bool()?,
            5 => self.RecordActId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeginnerCarnivalData {
    pub RoleId: i32,
    pub ActivityTaskData: Option<ActivityTaskData>,
    pub JumpTaskIds: Vec<i32>,
    pub JumpTaskCondInfos: Vec<JumpTaskCondInfo>,
}

impl PbMessage for BeginnerCarnivalData {
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
pub struct JumpTaskCondInfo {
    pub JumpId: i32,
    pub ConditionGroupIds: Vec<i32>,
}

impl PbMessage for JumpTaskCondInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.JumpId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JumpId);
        }
        for item in &self.ConditionGroupIds {
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
            1 => self.JumpId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ConditionGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityFunPlayChallengeData {
    pub ChallengeId: i32,
    pub UnlockTime: i64,
    pub RewardStatus: Option<FunPlayChallengeRewardStatus>,
    pub FunPlaySharpComment: Vec<i32>,
    pub FinishTime: i64,
}

impl PbMessage for ActivityFunPlayChallengeData {
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
        for item in &self.FunPlaySharpComment {
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
                &mut self.FunPlaySharpComment,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.FinishTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityFunPlayData {
    pub ActivityFunPlayChallengeData: Vec<ActivityFunPlayChallengeData>,
}

impl PbMessage for ActivityFunPlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityFunPlayChallengeData {
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
                self.ActivityFunPlayChallengeData
                    .push(<ActivityFunPlayChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivitySoarData {
    pub QuestId: i32,
}

impl PbMessage for ActivitySoarData {
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
pub struct InitHonamiActivityRequest {
    pub ActivityId: i32,
}

impl InitHonamiActivityRequest {
    pub const MSG_ID: u16 = 20790;
}

impl PbMessage for InitHonamiActivityRequest {
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
pub struct InitHonamiActivityResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub HonamiStoryActivityData: Option<HonamiStoryActivityData>,
}

impl InitHonamiActivityResponse {
    pub const MSG_ID: u16 = 24959;
}

impl PbMessage for InitHonamiActivityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.HonamiStoryActivityData {
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryActivityData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryActivityData {
    pub PlayerBagInfo: Option<HonamiStoryPlayerBagInfo>,
    pub ActivatedTalentId: Vec<i32>,
    pub ItemCollectionList: Vec<HonamiStoryItemCollectionConfig>,
    pub MascotConfigList: Vec<HonamiStoryMascotConfig>,
    pub AreaConfigList: Vec<HonamiStoryAreaConfig>,
    pub PermanentTaskData: Vec<ConditionTask>,
    pub LimitTaskData: Vec<ConditionTask>,
    pub ScoreRewardInfo: Vec<HonamiStoryScoreRewardInfo>,
    pub LifeSupportLevel: i32,
    pub LimitShopConsumeItemNum: i32,
    pub PbTowerInfos: Vec<TowerInfoData>,
    pub TalentInfos: Vec<TalentInfoData>,
    pub ItemCollectionInfos: Vec<ConditionTask>,
    pub TotalRevenue: i32,
}

impl PbMessage for HonamiStoryActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlayerBagInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ActivatedTalentId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ItemCollectionList {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.MascotConfigList {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.AreaConfigList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.PermanentTaskData {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.LimitTaskData {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.ScoreRewardInfo {
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
        for item in &self.PbTowerInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.TalentInfos {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.ItemCollectionInfos {
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
                self.PlayerBagInfo
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
                self.ItemCollectionList.push(
                    <HonamiStoryItemCollectionConfig as PbMessage>::decode(bytes)?,
                );
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MascotConfigList
                    .push(<HonamiStoryMascotConfig as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AreaConfigList
                    .push(<HonamiStoryAreaConfig as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PermanentTaskData
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.LimitTaskData
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ScoreRewardInfo
                    .push(<HonamiStoryScoreRewardInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.LifeSupportLevel = r.read_varint_i32()?,
            11 => self.LimitShopConsumeItemNum = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PbTowerInfos
                    .push(<TowerInfoData as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.TalentInfos
                    .push(<TalentInfoData as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.ItemCollectionInfos
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            15 => self.TotalRevenue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrThemeActivityPb {
    pub ActivityTaskData: Option<ActivityTaskData>,
}

impl PbMessage for InfrThemeActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTaskData {
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
pub struct FlagChallengeActivityInfo {
    pub ConditionTasks: Vec<ConditionTask>,
    pub FlagChallengeLevelInfos: Vec<FlagChallengeLevelInfo>,
    pub FlagStrongholdInfos: Vec<FlagStrongholdInfo>,
    pub FlagChallengeRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
    pub UnlockTeleporterId: Vec<i32>,
}

impl PbMessage for FlagChallengeActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.FlagChallengeLevelInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.FlagStrongholdInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.FlagChallengeRoleLevelInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.UnlockTeleporterId {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengeLevelInfos
                    .push(<FlagChallengeLevelInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FlagStrongholdInfos
                    .push(<FlagStrongholdInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengeRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockTeleporterId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingActivityInfo {
    pub ConditionTasks: Vec<ConditionTask>,
    pub BossPilingLevelInfos: Vec<BossPilingLevelInfo>,
}

impl PbMessage for BossPilingActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BossPilingLevelInfos {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BossPilingLevelInfos
                    .push(<BossPilingLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantActivityInfo {
    pub TotalStableValue: i32,
    pub GotRewardIds: Vec<i32>,
    pub ActiveIds: Vec<i32>,
}

impl PbMessage for GiantActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalStableValue != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalStableValue);
        }
        for item in &self.GotRewardIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ActiveIds {
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
            1 => self.TotalStableValue = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GotRewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActiveIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyAdventureActivityTask {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub Status: Option<DailyAdventureTaskState>,
}

impl PbMessage for DailyAdventureActivityTask {
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
                    <DailyAdventureTaskState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct DailyAdventureActivityData {
    pub DailyAdventureActivityTasks: Vec<DailyAdventureActivityTask>,
    pub PtRewardTaken: Vec<i32>,
}

impl PbMessage for DailyAdventureActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DailyAdventureActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.PtRewardTaken {
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
                self.DailyAdventureActivityTasks
                    .push(<DailyAdventureActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PtRewardTaken, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanMainData {
    pub Id: i32,
    pub Tasks: Vec<LongShanMainTaskData>,
    pub CanUnlock: bool,
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
}

impl PbMessage for LongShanMainData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CanUnlock {
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
                    .push(<LongShanMainTaskData as PbMessage>::decode(bytes)?);
            }
            3 => self.CanUnlock = r.read_bool()?,
            4 => self.BeginOpenTime = r.read_varint_i64()?,
            5 => self.EndOpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanMainTaskData {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub IsFinished: bool,
    pub IsTaken: bool,
    pub Unlock: bool,
    pub FinishConditions: Vec<i32>,
    pub ConditionId: i32,
    pub ConditionGroupId: i32,
    pub UnlockConditionFinish: bool,
}

impl PbMessage for LongShanMainTaskData {
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
        if self.ConditionGroupId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionGroupId);
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
            9 => self.ConditionGroupId = r.read_varint_i32()?,
            10 => self.UnlockConditionFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLongShanMain {
    pub StageData: Vec<LongShanMainData>,
    pub ScoreRewardedId: Vec<i32>,
}

impl PbMessage for ActivityLongShanMain {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StageData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ScoreRewardedId {
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
                self.StageData
                    .push(<LongShanMainData as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ScoreRewardedId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushActivityData {
    pub LevelDetailInfo: Vec<LevelInfo>,
    pub RewardInfo: Vec<BossRushScoreRewardData>,
    pub UnlockedBuffIndices: Vec<i32>,
    pub TaskProgressReward: Vec<ActivityTask>,
}

impl PbMessage for BossRushActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelDetailInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnlockedBuffIndices {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TaskProgressReward {
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
                self.LevelDetailInfo
                    .push(<LevelInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RewardInfo
                    .push(<BossRushScoreRewardData as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedBuffIndices,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TaskProgressReward
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelInfo {
    pub InstId: i32,
    pub StartTime: i32,
    pub IsOpen: bool,
    pub Score: i32,
    pub RoleInfo: Vec<i32>,
    pub BuffInfo: Vec<HardLevelBuffs>,
    pub LevelRewardClaimStatus: Option<BossRushRewardClaimStatus>,
    pub SelectScoreBuffs: Vec<i32>,
    pub LevelScoreRewardStatus: Vec<BossRushRewardClaimStatus>,
}

impl PbMessage for LevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartTime);
        }
        if self.IsOpen {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        for item in &self.RoleInfo {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BuffInfo {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.LevelRewardClaimStatus {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SelectScoreBuffs {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LevelScoreRewardStatus {
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.StartTime = r.read_varint_i32()?,
            3 => self.IsOpen = r.read_bool()?,
            4 => self.Score = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleInfo, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.BuffInfo
                    .push(<HardLevelBuffs as PbMessage>::decode(bytes)?);
            }
            7 => {
                self.LevelRewardClaimStatus = Some(
                    <BossRushRewardClaimStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectScoreBuffs,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LevelScoreRewardStatus,
                |sr| {
                    Ok(
                        <BossRushRewardClaimStatus as crate::wire::PbEnum>::from_i32_or_default(
                            sr.read_varint_i32()?,
                        ),
                    )
                },
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushScoreRewardData {
    pub RewardDataId: i32,
    pub State: Option<BossRushRewardClaimStatus>,
}

impl PbMessage for BossRushScoreRewardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardDataId);
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
            1 => self.RewardDataId = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <BossRushRewardClaimStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct HardLevelBuffs {
    pub BuffId: i32,
    pub Slot: i32,
    pub State: Option<BossRushBuffSelectionStatus>,
}

impl PbMessage for HardLevelBuffs {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
        }
        if self.Slot != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Slot);
        }
        if let Some(v) = &self.State {
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
            1 => self.BuffId = r.read_varint_i32()?,
            2 => self.Slot = r.read_varint_i32()?,
            3 => {
                self.State = Some(
                    <BossRushBuffSelectionStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ActivityTask {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub Status: Option<ActivityTaskState>,
    pub PreItemMap: BTreeMap<i32, i32>,
}

impl PbMessage for ActivityTask {
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
            2 => self.Current = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
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
pub struct ActivityTaskData {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl PbMessage for ActivityTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionTask {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub Status: Option<ConditionTaskState>,
}

impl PbMessage for ConditionTask {
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
                    <ConditionTaskState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ActivityTimePointRewarData {
    pub Rewards: Vec<TimePointRewardData>,
}

impl PbMessage for ActivityTimePointRewarData {
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
                    .push(<TimePointRewardData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimePointRewardData {
    pub Id: i32,
    pub RewardTime: i64,
    pub Rewarded: bool,
    pub CanGetReward: bool,
    pub Task: Option<ConditionTask>,
}

impl PbMessage for TimePointRewardData {
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
pub struct TowerDefenseActivityInfo {
    pub InstanceInfos: Vec<TowerDefenceInstanceInfo>,
    pub RewardedScoreIds: Vec<i32>,
    pub TotalScore: i32,
    pub ShowName: bool,
}

impl PbMessage for TowerDefenseActivityInfo {
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
pub struct TowerDefenceInstanceInfo {
    pub Id: i32,
    pub Score: i32,
    pub Rewarded: bool,
    pub IsPassed: bool,
    pub UnlockTime: i64,
    pub MaxScore: i32,
    pub PassTime: i32,
}

impl PbMessage for TowerDefenceInstanceInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Rewarded {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsPassed {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
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
            3 => self.Rewarded = r.read_bool()?,
            4 => self.IsPassed = r.read_bool()?,
            5 => self.UnlockTime = r.read_varint_i64()?,
            6 => self.MaxScore = r.read_varint_i32()?,
            7 => self.PassTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestStarRewardInfo {
    pub TargetScore: i32,
    pub State: Option<StarRewardState>,
}

impl PbMessage for RiskHarvestStarRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetScore);
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
            1 => self.TargetScore = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <StarRewardState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RiskHarvestInstInfo {
    pub Id: i32,
    pub UnlockTime: i64,
    pub IsUnlock: bool,
    pub Score: i32,
    pub Rewarded: bool,
    pub IsFinished: bool,
    pub StarRewardInfos: Vec<RiskHarvestStarRewardInfo>,
}

impl PbMessage for RiskHarvestInstInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Rewarded {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsFinished {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.StarRewardInfos {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.IsUnlock = r.read_bool()?,
            4 => self.Score = r.read_varint_i32()?,
            5 => self.Rewarded = r.read_bool()?,
            6 => self.IsFinished = r.read_bool()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.StarRewardInfos
                    .push(<RiskHarvestStarRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestActivityData {
    pub InstInfos: Vec<RiskHarvestInstInfo>,
    pub RewardedScores: Vec<i32>,
    pub RewardedBuffGroups: Vec<i32>,
    pub UnlockBuffGroups: Vec<i32>,
    pub RewardedBuffTypeIds: Vec<i32>,
}

impl PbMessage for RiskHarvestActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardedScores {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardedBuffGroups {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBuffGroups {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardedBuffTypeIds {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.InstInfos
                    .push(<RiskHarvestInstInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedScores, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedBuffGroups,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBuffGroups,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedBuffTypeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelPlayInfo {
    pub InstId: i32,
    pub StartTime: i32,
    pub Challenges: bool,
    pub Points: i32,
    pub LevelRewardGet: bool,
    pub Difficulty: i32,
}

impl PbMessage for FarmGoldLevelPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartTime);
        }
        if self.Challenges {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Points != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Points);
        }
        if self.LevelRewardGet {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Difficulty != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Difficulty);
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
            2 => self.StartTime = r.read_varint_i32()?,
            3 => self.Challenges = r.read_bool()?,
            4 => self.Points = r.read_varint_i32()?,
            5 => self.LevelRewardGet = r.read_bool()?,
            6 => self.Difficulty = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldData {
    pub PointRewardGet: Vec<i32>,
    pub LevelPlayTasks: Vec<FarmGoldLevelPlayInfo>,
}

impl PbMessage for FarmGoldData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PointRewardGet {
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
            1 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PointRewardGet, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
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
pub struct MapTravelActivityData {
    pub ActivityTasks: Vec<ActivityTask>,
    pub MonsterGain: Vec<i32>,
    pub GetFullReward: bool,
    pub MapTravelLevel: i32,
    pub UnlockAreas: Vec<i32>,
    pub SoarLevels: Vec<SoarLevelPlayInfo>,
}

impl PbMessage for MapTravelActivityData {
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
        for item in &self.UnlockAreas {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockAreas, |sr| {
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
pub struct SoarLevelPlayInfo {
    pub SoarLevelPlatId: i32,
    pub HistorySoarScore: i32,
    pub ReceiveIds: Vec<i32>,
}

impl PbMessage for SoarLevelPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SoarLevelPlatId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SoarLevelPlatId);
        }
        if self.HistorySoarScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistorySoarScore);
        }
        for item in &self.ReceiveIds {
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
            1 => self.SoarLevelPlatId = r.read_varint_i32()?,
            2 => self.HistorySoarScore = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReceiveIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSpeedContext {
    pub LevelId: i32,
    pub Score: i32,
    pub Ranking: i32,
    pub StartTime: i32,
    pub LapRecord: i32,
}

impl PbMessage for SolarSpeedContext {
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
        if self.LapRecord != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LapRecord);
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
            5 => self.LapRecord = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarisSpeedActivity {
    pub SolarSpeedContext: Vec<SolarSpeedContext>,
    pub ActivityTaskDatas: Option<ActivityTaskData>,
}

impl PbMessage for SolarisSpeedActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SolarSpeedContext {
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
                self.SolarSpeedContext
                    .push(<SolarSpeedContext as PbMessage>::decode(bytes)?);
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
pub struct RacingBetsLegMatchData {
    pub LegMatchesId: i32,
    pub DangoId: i32,
    pub BettingGearId: i32,
    pub BettingGearCash: i32,
    pub Odds: i32,
    pub OddsVersion: Vec<u8>,
    pub LeaveCancelNum: i32,
    pub OddsReward: i32,
}

impl PbMessage for RacingBetsLegMatchData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchesId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchesId);
        }
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.BettingGearId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BettingGearId);
        }
        if self.BettingGearCash != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BettingGearCash);
        }
        if self.Odds != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Odds);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if self.LeaveCancelNum != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeaveCancelNum);
        }
        if self.OddsReward != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OddsReward);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LegMatchesId = r.read_varint_i32()?,
            2 => self.DangoId = r.read_varint_i32()?,
            3 => self.BettingGearId = r.read_varint_i32()?,
            4 => self.BettingGearCash = r.read_varint_i32()?,
            5 => self.Odds = r.read_varint_i32()?,
            6 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            7 => self.LeaveCancelNum = r.read_varint_i32()?,
            8 => self.OddsReward = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoActorData {
    pub Id: i32,
    pub Record: i32,
    pub Odds: i32,
}

impl PbMessage for DangoActorData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Record != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Record);
        }
        if self.Odds != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Odds);
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
            2 => self.Record = r.read_varint_i32()?,
            3 => self.Odds = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsOrganInfo {
    pub Id: i32,
    pub Point: i32,
}

impl PbMessage for RacingBetsOrganInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Point = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTimeTuple {
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for RacingBetsTimeTuple {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.BeginTime = r.read_varint_i64()?,
            2 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsLegMatch {
    pub Id: i32,
    pub DangoActorData: Vec<DangoActorData>,
    pub MatchStartEndTime: Option<RacingBetsTimeTuple>,
    pub GearStartEndTime: Option<RacingBetsTimeTuple>,
    pub BetDangoRank: Vec<i32>,
    pub OddsRateRefreshTime: i64,
    pub OddsVersion: Vec<u8>,
    pub MasterLevel: Option<RacingBetsTimeTuple>,
    pub OrganInfo: Vec<RacingBetsOrganInfo>,
}

impl PbMessage for RacingBetsLegMatch {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.DangoActorData {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.MatchStartEndTime {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.GearStartEndTime {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.BetDangoRank {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.OddsRateRefreshTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OddsRateRefreshTime);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if let Some(v) = &self.MasterLevel {
            crate::wire::write_submessage(out, 8, v);
        }
        for item in &self.OrganInfo {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DangoActorData
                    .push(<DangoActorData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MatchStartEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GearStartEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BetDangoRank, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => self.OddsRateRefreshTime = r.read_varint_i64()?,
            7 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MasterLevel
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.OrganInfo
                    .push(<RacingBetsOrganInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGroupMatchInfo {
    pub MatchId: i32,
    pub GroupMatchTime: Option<RacingBetsTimeTuple>,
    pub LegMatch: Vec<RacingBetsLegMatch>,
    pub PromoteDangoList: Vec<i32>,
    pub Dangos: Vec<i32>,
}

impl PbMessage for RacingBetsGroupMatchInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchId);
        }
        if let Some(v) = &self.GroupMatchTime {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.LegMatch {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.PromoteDangoList {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Dangos {
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
            1 => self.MatchId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GroupMatchTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LegMatch
                    .push(<RacingBetsLegMatch as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PromoteDangoList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Dangos, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsSeasonData {
    pub CurCash: i32,
    pub TotalCash: i32,
    pub RacingBetsLegMatchData: Vec<RacingBetsLegMatchData>,
    pub HitNum: i32,
}

impl PbMessage for RacingBetsSeasonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCash != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCash);
        }
        if self.TotalCash != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCash);
        }
        for item in &self.RacingBetsLegMatchData {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.HitNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HitNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurCash = r.read_varint_i32()?,
            2 => self.TotalCash = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RacingBetsLegMatchData
                    .push(<RacingBetsLegMatchData as PbMessage>::decode(bytes)?);
            }
            4 => self.HitNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRewardData {
    pub Id: i32,
    pub Status: Option<ConditionTaskStatus>,
    pub Progress: i32,
    pub TargetProgress: i32,
    pub ConditionFinishState: bool,
}

impl PbMessage for RacingBetsRewardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetProgress);
        }
        if self.ConditionFinishState {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <ConditionTaskStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Progress = r.read_varint_i32()?,
            4 => self.TargetProgress = r.read_varint_i32()?,
            6 => self.ConditionFinishState = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityBetHorsesData {
    pub ActivityId: i32,
    pub StartAndEndTime: Option<RacingBetsTimeTuple>,
    pub MatchInfo: Vec<RacingBetsGroupMatchInfo>,
    pub RacingBetsSeasonData: Option<RacingBetsSeasonData>,
    pub BetsRewardData: Vec<RacingBetsRewardData>,
    pub LegMatchTimeList: Vec<i64>,
    pub CloseSettleMenuLegMatchList: Vec<i32>,
}

impl PbMessage for ActivityBetHorsesData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.StartAndEndTime {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.MatchInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.RacingBetsSeasonData {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.BetsRewardData {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.LegMatchTimeList {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.CloseSettleMenuLegMatchList {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.StartAndEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MatchInfo
                    .push(<RacingBetsGroupMatchInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RacingBetsSeasonData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.BetsRewardData
                    .push(<RacingBetsRewardData as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LegMatchTimeList,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CloseSettleMenuLegMatchList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityAvignon {
    pub RewardData: Option<ActivityTaskData>,
    pub StageId: Vec<i32>,
}

impl PbMessage for ActivityAvignon {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardData {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.StageId {
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
                self.RewardData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StageId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLinkageRewardData {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for ActivityLinkageRewardData {
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
