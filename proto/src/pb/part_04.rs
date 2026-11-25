use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleIllustratedInfoResponse {
    pub Roles: Vec<RoleHandbookInfo>,
    pub Weapons: Vec<WeaponHandbookInfo>,
}

impl RoleIllustratedInfoResponse {
    pub const MSG_ID: u16 = 22483;
}

impl PbMessage for RoleIllustratedInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.Weapons {
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .push(<RoleHandbookInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Weapons
                    .push(<WeaponHandbookInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleHandbookInfo {
    pub RoleId: i32,
    pub UnlockTime: i64,
}

impl PbMessage for RoleHandbookInfo {
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
pub struct WeaponHandbookInfo {
    pub WeaponId: i32,
    pub UnlockTime: i64,
}

impl PbMessage for WeaponHandbookInfo {
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
pub struct InfluenceInfoRequest {}

impl InfluenceInfoRequest {
    pub const MSG_ID: u16 = 29194;
}

impl PbMessage for InfluenceInfoRequest {
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
pub struct InfluenceInfoResponse {
    pub InfluenceInfos: Vec<InfluenceInfo>,
}

impl InfluenceInfoResponse {
    pub const MSG_ID: u16 = 21485;
}

impl PbMessage for InfluenceInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InfluenceInfos {
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
                self.InfluenceInfos
                    .push(<InfluenceInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfluenceInfo {
    pub InfluenceId: i32,
    pub RewardIndex: i32,
    pub Relation: i32,
}

impl PbMessage for InfluenceInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InfluenceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InfluenceId);
        }
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if self.Relation != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Relation);
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
            2 => self.RewardIndex = r.read_varint_i32()?,
            3 => self.Relation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrInfoRequest {}

impl InfrInfoRequest {
    pub const MSG_ID: u16 = 21330;
}

impl PbMessage for InfrInfoRequest {
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
pub struct InfrInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub InfrInfo: Option<InfrPb>,
}

impl InfrInfoResponse {
    pub const MSG_ID: u16 = 29855;
}

impl PbMessage for InfrInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InfrInfo {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => {
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
pub struct InfrRoadUpdateNotify {
    pub RoadInfo: Option<InfrRoadPb>,
}

impl InfrRoadUpdateNotify {
    pub const MSG_ID: u16 = 23036;
}

impl PbMessage for InfrRoadUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoadInfo {
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
                self.RoadInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrPb {
    pub FireInfo: Option<InfrFirePb>,
    pub RoadInfo: Option<InfrRoadPb>,
    pub LibraryInfo: Option<InfrLibraryPb>,
}

impl PbMessage for InfrPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FireInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.RoadInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.LibraryInfo {
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
                self.FireInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoadInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LibraryInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrLibraryPb {
    pub ArchiveTasks: Vec<InfrTaskPb>,
    pub PhoneTasks: Vec<InfrTaskPb>,
    pub UnreadArchives: Vec<i32>,
}

impl PbMessage for InfrLibraryPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ArchiveTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.PhoneTasks {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnreadArchives {
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
                self.ArchiveTasks
                    .push(<InfrTaskPb as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhoneTasks
                    .push(<InfrTaskPb as PbMessage>::decode(bytes)?);
            }
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnreadArchives, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrTaskPb {
    pub TaskId: i32,
    pub Target: i32,
    pub status: Option<InfrTaskStatusPb>,
}

impl PbMessage for InfrTaskPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Target);
        }
        if let Some(v) = &self.status {
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
            1 => self.TaskId = r.read_varint_i32()?,
            2 => self.Target = r.read_varint_i32()?,
            3 => {
                self.status = Some(
                    <InfrTaskStatusPb as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct InfrFirePb {
    pub FireExp: i64,
    pub FireLevel: i32,
    pub FireLevelReachTime: i64,
    pub FireStatus: Option<InfrStatusPb>,
}

impl PbMessage for InfrFirePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FireExp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FireExp);
        }
        if self.FireLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FireLevel);
        }
        if self.FireLevelReachTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FireLevelReachTime);
        }
        if let Some(v) = &self.FireStatus {
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
            1 => self.FireExp = r.read_varint_i64()?,
            2 => self.FireLevel = r.read_varint_i32()?,
            3 => self.FireLevelReachTime = r.read_varint_i64()?,
            4 => {
                self.FireStatus = Some(<InfrStatusPb as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrRoadPb {
    pub Roads: Vec<InfrOneRoad>,
    pub Notices: Vec<InfrNotice>,
    pub ManualTraceRoad: i32,
    pub RecommendRoad: i32,
}

impl PbMessage for InfrRoadPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roads {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.Notices {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.ManualTraceRoad != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ManualTraceRoad);
        }
        if self.RecommendRoad != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommendRoad);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Roads.push(<InfrOneRoad as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Notices.push(<InfrNotice as PbMessage>::decode(bytes)?);
            }
            3 => self.ManualTraceRoad = r.read_varint_i32()?,
            4 => self.RecommendRoad = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrOneRoad {
    pub RoadId: i32,
    pub status: Option<InfrStatusPb>,
    pub CompleteTime: i64,
    pub TotalGiftCount: i64,
    pub LastGiftTime: i64,
}

impl PbMessage for InfrOneRoad {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoadId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoadId);
        }
        if let Some(v) = &self.status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CompleteTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CompleteTime);
        }
        if self.TotalGiftCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalGiftCount);
        }
        if self.LastGiftTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastGiftTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoadId = r.read_varint_i32()?,
            2 => {
                self.status = Some(<InfrStatusPb as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.CompleteTime = r.read_varint_i64()?,
            4 => self.TotalGiftCount = r.read_varint_i64()?,
            5 => self.LastGiftTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrNotice {
    pub RoadId: i32,
    pub PasserId: i32,
    pub GiftCount: i32,
    pub CreateTime: i64,
}

impl PbMessage for InfrNotice {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoadId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoadId);
        }
        if self.PasserId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PasserId);
        }
        if self.GiftCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GiftCount);
        }
        if self.CreateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CreateTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoadId = r.read_varint_i32()?,
            2 => self.PasserId = r.read_varint_i32()?,
            3 => self.GiftCount = r.read_varint_i32()?,
            4 => self.CreateTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2Pb {
    pub FireInfo: Option<InfrV2FirePb>,
    pub TreeInfo: Option<InfrV2TreePb>,
    pub RewardScoreIds: Vec<i32>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub TreeFinishCond: Vec<i32>,
}

impl PbMessage for InfrV2Pb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FireInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.TreeInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.RewardScoreIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.TreeFinishCond {
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
                self.FireInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TreeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardScoreIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TreeFinishCond, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2InfoRequest {}

impl InfrV2InfoRequest {
    pub const MSG_ID: u16 = 20516;
}

impl PbMessage for InfrV2InfoRequest {
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
pub struct InfrV2InfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub InfrInfo: Option<InfrV2Pb>,
}

impl InfrV2InfoResponse {
    pub const MSG_ID: u16 = 26149;
}

impl PbMessage for InfrV2InfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InfrInfo {
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
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
pub struct InfrV2FirePb {
    pub FireExp: i64,
    pub FireLevel: i32,
    pub FireLevelReachTime: i64,
    pub FireStatus: Option<InfrV2StatusPb>,
}

impl PbMessage for InfrV2FirePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FireExp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FireExp);
        }
        if self.FireLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FireLevel);
        }
        if self.FireLevelReachTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FireLevelReachTime);
        }
        if let Some(v) = &self.FireStatus {
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
            1 => self.FireExp = r.read_varint_i64()?,
            2 => self.FireLevel = r.read_varint_i32()?,
            3 => self.FireLevelReachTime = r.read_varint_i64()?,
            4 => {
                self.FireStatus = Some(
                    <InfrV2StatusPb as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct InfrV2TreePb {
    pub Trees: Vec<InfrV2OneTree>,
    pub ManualTraceTree: i32,
}

impl PbMessage for InfrV2TreePb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Trees {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.ManualTraceTree != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ManualTraceTree);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Trees
                    .push(<InfrV2OneTree as PbMessage>::decode(bytes)?);
            }
            3 => self.ManualTraceTree = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2OneTree {
    pub TreeId: i32,
    pub status: Option<InfrV2StatusPb>,
    pub CompleteTime: i64,
    pub TotalGiftCount: i64,
    pub LastGiftTime: i64,
}

impl PbMessage for InfrV2OneTree {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeId);
        }
        if let Some(v) = &self.status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CompleteTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CompleteTime);
        }
        if self.TotalGiftCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalGiftCount);
        }
        if self.LastGiftTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastGiftTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeId = r.read_varint_i32()?,
            2 => {
                self.status = Some(
                    <InfrV2StatusPb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.CompleteTime = r.read_varint_i64()?,
            4 => self.TotalGiftCount = r.read_varint_i64()?,
            5 => self.LastGiftTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstDataNotify {
    pub EnterInfos: Vec<InstEnterInfoPb>,
}

impl InstDataNotify {
    pub const MSG_ID: u16 = 18211;
}

impl PbMessage for InstDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EnterInfos {
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
                self.EnterInfos
                    .push(<InstEnterInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstEnterInfoPb {
    pub Id: i32,
    pub ChallengedTimes: i32,
}

impl PbMessage for InstEnterInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ChallengedTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengedTimes);
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
            2 => self.ChallengedTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateInstanceDungeonNotify {
    pub LevelPlayId: i32,
}

impl CreateInstanceDungeonNotify {
    pub const MSG_ID: u16 = 18768;
}

impl PbMessage for CreateInstanceDungeonNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExchangeRewardRequest {}

impl ExchangeRewardRequest {
    pub const MSG_ID: u16 = 25606;
}

impl PbMessage for ExchangeRewardRequest {
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
pub struct ExchangeRewardResponse {
    pub ExchangeShareData: BTreeMap<i32, i32>,
    pub ExchangeRewardData: BTreeMap<i32, i32>,
}

impl ExchangeRewardResponse {
    pub const MSG_ID: u16 = 23945;
}

impl PbMessage for ExchangeRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ExchangeShareData.is_empty() {
            crate::wire::write_map(out, 13, &self.ExchangeShareData);
        }
        if !self.ExchangeRewardData.is_empty() {
            crate::wire::write_map(out, 12, &self.ExchangeRewardData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.ExchangeShareData)?,
            12 => crate::wire::decode_map_entry(r, &mut self.ExchangeRewardData)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemExchangeInfo {
    pub ItemId: i32,
    pub TodayTimes: i32,
    pub TotalTimes: i32,
    pub DailyLimit: i32,
    pub TotalLimit: i32,
}

impl PbMessage for ItemExchangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.TodayTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TodayTimes);
        }
        if self.TotalTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalTimes);
        }
        if self.DailyLimit != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DailyLimit);
        }
        if self.TotalLimit != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalLimit);
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
            2 => self.TodayTimes = r.read_varint_i32()?,
            3 => self.TotalTimes = r.read_varint_i32()?,
            4 => self.DailyLimit = r.read_varint_i32()?,
            5 => self.TotalLimit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemExchangeInfoRequest {}

impl ItemExchangeInfoRequest {
    pub const MSG_ID: u16 = 16256;
}

impl PbMessage for ItemExchangeInfoRequest {
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
pub struct ItemExchangeInfoResponse {
    pub ItemExchangeInfos: Vec<ItemExchangeInfo>,
}

impl ItemExchangeInfoResponse {
    pub const MSG_ID: u16 = 19721;
}

impl PbMessage for ItemExchangeInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemExchangeInfos {
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
                self.ItemExchangeInfos
                    .push(<ItemExchangeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityAttributePbInfo {
    pub AttributeMap: BTreeMap<i32, i32>,
    pub LockedAttributeMap: BTreeMap<i32, i32>,
}

impl PbMessage for SimpleCombatEntityAttributePbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AttributeMap.is_empty() {
            crate::wire::write_map(out, 1, &self.AttributeMap);
        }
        if !self.LockedAttributeMap.is_empty() {
            crate::wire::write_map(out, 2, &self.LockedAttributeMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.AttributeMap)?,
            2 => crate::wire::decode_map_entry(r, &mut self.LockedAttributeMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayInfo {
    pub Id: i32,
    pub IsFirst: bool,
    pub State: i32,
    pub UpdateTime: i64,
    pub GetRewardCount: i32,
}

impl PbMessage for LevelPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsFirst {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if self.UpdateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpdateTime);
        }
        if self.GetRewardCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GetRewardCount);
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
            2 => self.IsFirst = r.read_bool()?,
            3 => self.State = r.read_varint_i32()?,
            4 => self.UpdateTime = r.read_varint_i64()?,
            5 => self.GetRewardCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayInfoNotify {
    pub LevelPlayInfo: Vec<LevelPlayInfo>,
}

impl LevelPlayInfoNotify {
    pub const MSG_ID: u16 = 28525;
}

impl PbMessage for LevelPlayInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelPlayInfo {
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
                self.LevelPlayInfo
                    .push(<LevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleTrackReportAsyncRequest {}

impl SimpleTrackReportAsyncRequest {
    pub const MSG_ID: u16 = 15365;
}

impl PbMessage for SimpleTrackReportAsyncRequest {
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
pub struct SimpleTrackReportAsyncResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SimpleTrackReportMsgs: Vec<SimpleTrackReportMsg>,
}

impl SimpleTrackReportAsyncResponse {
    pub const MSG_ID: u16 = 23444;
}

impl PbMessage for SimpleTrackReportAsyncResponse {
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
pub struct SimpleTrackReportMsg {
    pub InstId: i32,
    pub LevelPlayId: i32,
    pub GainTreasureCount: i32,
}

impl PbMessage for SimpleTrackReportMsg {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if self.GainTreasureCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GainTreasureCount);
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
            2 => self.LevelPlayId = r.read_varint_i32()?,
            3 => self.GainTreasureCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayVarAsyncRequest {
    pub InstId: i32,
    pub LevelPlayId: i32,
}

impl LevelPlayVarAsyncRequest {
    pub const MSG_ID: u16 = 17974;
}

impl PbMessage for LevelPlayVarAsyncRequest {
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
pub struct LevelPlayVarAsyncResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Vars: BTreeMap<Vec<u8>, VarDefinePb>,
}

impl LevelPlayVarAsyncResponse {
    pub const MSG_ID: u16 = 21256;
}

impl PbMessage for LevelPlayVarAsyncResponse {
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
pub struct PlayPointStateAsyncRequest {
    pub InstId: i32,
    pub ArenaId: i32,
}

impl PlayPointStateAsyncRequest {
    pub const MSG_ID: u16 = 16155;
}

impl PbMessage for PlayPointStateAsyncRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.ArenaId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ArenaId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.InstId = r.read_varint_i32()?,
            3 => self.ArenaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayPointStateAsyncResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LevelPlayStateDict: BTreeMap<i32, LevelPlayStateMsg>,
}

impl PlayPointStateAsyncResponse {
    pub const MSG_ID: u16 = 29313;
}

impl PbMessage for PlayPointStateAsyncResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.LevelPlayStateDict.is_empty() {
            crate::wire::write_map(out, 5, &self.LevelPlayStateDict);
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
            5 => crate::wire::decode_map_entry(r, &mut self.LevelPlayStateDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayStateMsg {
    pub LevelPlayEntityId: i32,
    pub ExploratoryType: i32,
    pub StateType: i32,
    pub CompleteNumber: i32,
    pub IsHide: bool,
    pub HideGroupInfo: Vec<u8>,
    pub IsUnlocked: bool,
    pub LevelPlayMarkUnlock: bool,
}

impl PbMessage for LevelPlayStateMsg {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayEntityId);
        }
        if self.ExploratoryType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploratoryType);
        }
        if self.StateType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateType);
        }
        if self.CompleteNumber != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CompleteNumber);
        }
        if self.IsHide {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.HideGroupInfo.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.HideGroupInfo);
        }
        if self.IsUnlocked {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelPlayMarkUnlock {
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
            1 => self.LevelPlayEntityId = r.read_varint_i32()?,
            2 => self.ExploratoryType = r.read_varint_i32()?,
            3 => self.StateType = r.read_varint_i32()?,
            4 => self.CompleteNumber = r.read_varint_i32()?,
            5 => self.IsHide = r.read_bool()?,
            6 => self.HideGroupInfo = r.read_len_delimited()?.to_vec(),
            7 => self.IsUnlocked = r.read_bool()?,
            8 => self.LevelPlayMarkUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessInfo {
    pub LivenessCount: i32,
    pub RewardedLiveness: Vec<i32>,
    pub Tasks: Vec<LivenessTask>,
    pub DayEnd: i64,
    pub AreaId: i32,
}

impl PbMessage for LivenessInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LivenessCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LivenessCount);
        }
        for item in &self.RewardedLiveness {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.DayEnd != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DayEnd);
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
            1 => self.LivenessCount = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedLiveness,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks.push(<LivenessTask as PbMessage>::decode(bytes)?);
            }
            4 => self.DayEnd = r.read_varint_i64()?,
            5 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessTask {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub IsFinished: bool,
    pub IsTaken: bool,
    pub ConditionFinishState: bool,
}

impl PbMessage for LivenessTask {
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
            2 => self.Current = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
            4 => self.IsFinished = r.read_bool()?,
            5 => self.IsTaken = r.read_bool()?,
            6 => self.ConditionFinishState = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessRequest {}

impl LivenessRequest {
    pub const MSG_ID: u16 = 19066;
}

impl PbMessage for LivenessRequest {
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
pub struct LivenessResponse {
    pub LivenessInfo: Option<LivenessInfo>,
}

impl LivenessResponse {
    pub const MSG_ID: u16 = 26035;
}

impl PbMessage for LivenessResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LivenessInfo {
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
pub struct LivenessTakeRequest {
    pub Ids: Vec<i32>,
}

impl LivenessTakeRequest {
    pub const MSG_ID: u16 = 24082;
}

impl PbMessage for LivenessTakeRequest {
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
pub struct LivenessTakeResponse {
    pub Ids: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl LivenessTakeResponse {
    pub const MSG_ID: u16 = 15626;
}

impl PbMessage for LivenessTakeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct RoleLoadEquipData {
    pub RoleId: i32,
    pub Pos: Option<EquipPos>,
    pub EquipIncId: i32,
}

impl PbMessage for RoleLoadEquipData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EquipIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipIncId);
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
                self.Pos = Some(<EquipPos as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.EquipIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadEquipData {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl PbMessage for LoadEquipData {
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
pub struct WeaponSkinRequest {}

impl WeaponSkinRequest {
    pub const MSG_ID: u16 = 28824;
}

impl PbMessage for WeaponSkinRequest {
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
pub struct WeaponSkinResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EquipList: Vec<LoadEquipData>,
}

impl WeaponSkinResponse {
    pub const MSG_ID: u16 = 29155;
}

impl PbMessage for WeaponSkinResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.EquipList {
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
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.EquipList
                    .push(<LoadEquipData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipTakeOnRequest {
    pub Data: Option<RoleLoadEquipData>,
}

impl EquipTakeOnRequest {
    pub const MSG_ID: u16 = 21385;
}

impl PbMessage for EquipTakeOnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct EquipTakeOnResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub DataList: Vec<RoleLoadEquipData>,
}

impl EquipTakeOnResponse {
    pub const MSG_ID: u16 = 29980;
}

impl PbMessage for EquipTakeOnResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DataList {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DataList
                    .push(<RoleLoadEquipData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipTakeOnNotify {
    pub DataList: Vec<RoleLoadEquipData>,
}

impl EquipTakeOnNotify {
    pub const MSG_ID: u16 = 25950;
}

impl PbMessage for EquipTakeOnNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DataList {
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
                self.DataList
                    .push(<RoleLoadEquipData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockSkinDataNotify {
    pub PhantomSkinList: Vec<i32>,
    pub IsLogin: bool,
}

impl UnlockSkinDataNotify {
    pub const MSG_ID: u16 = 17173;
}

impl PbMessage for UnlockSkinDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomSkinList {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomSkinList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => self.IsLogin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSkinDeleteNotify {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl WeaponSkinDeleteNotify {
    pub const MSG_ID: u16 = 17893;
}

impl PbMessage for WeaponSkinDeleteNotify {
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
pub struct EquipWeaponSkinRequest {
    pub Data: Option<LoadEquipData>,
}

impl EquipWeaponSkinRequest {
    pub const MSG_ID: u16 = 16426;
}

impl PbMessage for EquipWeaponSkinRequest {
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
pub struct EquipWeaponSkinResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub DataList: Vec<LoadEquipData>,
}

impl EquipWeaponSkinResponse {
    pub const MSG_ID: u16 = 26442;
}

impl PbMessage for EquipWeaponSkinResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DataList {
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
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.DataList
                    .push(<LoadEquipData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendEquipSkinRequest {
    pub RoleId: i32,
}

impl SendEquipSkinRequest {
    pub const MSG_ID: u16 = 25017;
}

impl PbMessage for SendEquipSkinRequest {
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
pub struct SendEquipSkinResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SendEquipSkinResponse {
    pub const MSG_ID: u16 = 15525;
}

impl PbMessage for SendEquipSkinResponse {
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
pub struct AceBlackProductAccountInfo {
    pub TdmDeviceId: Vec<u8>,
    pub IsRoot: bool,
    pub IsSimulator: bool,
}

impl PbMessage for AceBlackProductAccountInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TdmDeviceId.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TdmDeviceId);
        }
        if self.IsRoot {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSimulator {
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
            1 => self.TdmDeviceId = r.read_len_delimited()?.to_vec(),
            2 => self.IsRoot = r.read_bool()?,
            3 => self.IsSimulator = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DevLoginCheckData {
    pub ProtoVersion: i32,
    pub ProtoMD5: Vec<u8>,
    pub ConfigVersion: i32,
    pub ConfigMD5: Vec<u8>,
    pub BranchName: Vec<u8>,
    pub ProtoSeedMD5: Vec<u8>,
}

impl PbMessage for DevLoginCheckData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ProtoVersion != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProtoVersion);
        }
        if !self.ProtoMD5.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ProtoMD5);
        }
        if self.ConfigVersion != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigVersion);
        }
        if !self.ConfigMD5.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ConfigMD5);
        }
        if !self.BranchName.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.BranchName);
        }
        if !self.ProtoSeedMD5.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ProtoSeedMD5);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ProtoVersion = r.read_varint_i32()?,
            2 => self.ProtoMD5 = r.read_len_delimited()?.to_vec(),
            3 => self.ConfigVersion = r.read_varint_i32()?,
            4 => self.ConfigMD5 = r.read_len_delimited()?.to_vec(),
            5 => self.BranchName = r.read_len_delimited()?.to_vec(),
            6 => self.ProtoSeedMD5 = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginRequestDevLoginCheckOneof {
    DevLoginCheckData(DevLoginCheckData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoginRequest {
    pub DevLoginCheck: Option<LoginRequestDevLoginCheckOneof>,
    pub Id: i32,
    pub Account: Vec<u8>,
    pub LoginTraceId: Vec<u8>,
    pub Token: Vec<u8>,
    pub AppVersion: Vec<u8>,
    pub LauncherVersion: Vec<u8>,
    pub ResourceVersion: Vec<u8>,
    pub ClientBasicInfo: Option<ClientBasicInfo>,
    pub PublicResourceVersionInfo: Option<PublicResourceVersionInfo>,
    pub AceBlackProductAccountInfo: Option<AceBlackProductAccountInfo>,
    pub PushNotificationsEnabled: bool,
    pub ClientId: Vec<u8>,
    pub SdkUserId: Vec<u8>,
    pub SdkOnlineId: Vec<u8>,
    pub SdkAccountId: Vec<u8>,
    pub PackageClientFightConfig: Vec<u8>,
    pub LimitState: i32,
    pub FsmVersion: i32,
    pub ConfirmQuestResource: bool,
    pub QuestReourceState: i32,
    pub IsLowMemorePlatform: bool,
    pub BlockState: Option<BlockState>,
    pub downloadResourceQuestId: Vec<i32>,
    pub XboxUserId: Vec<u8>,
    pub XboxOnlineId: Vec<u8>,
    pub XboxAccountId: Vec<u8>,
    pub XboxSocialState: i32,
    pub XstsToken: Vec<u8>,
    pub GradedResourceVersion: Vec<u8>,
}

impl LoginRequest {
    pub const MSG_ID: u16 = 103;
}

impl PbMessage for LoginRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.DevLoginCheck {
            match oneof {
                LoginRequestDevLoginCheckOneof::DevLoginCheckData(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.Account.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Account);
        }
        if !self.LoginTraceId.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LoginTraceId);
        }
        if !self.Token.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Token);
        }
        if !self.AppVersion.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AppVersion);
        }
        if !self.LauncherVersion.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LauncherVersion);
        }
        if !self.ResourceVersion.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourceVersion);
        }
        if let Some(v) = &self.ClientBasicInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.PublicResourceVersionInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.AceBlackProductAccountInfo {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.PushNotificationsEnabled {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.ClientId.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientId);
        }
        if !self.SdkUserId.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkUserId);
        }
        if !self.SdkOnlineId.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkOnlineId);
        }
        if !self.SdkAccountId.is_empty() {
            crate::wire::write_tag(out, 16, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SdkAccountId);
        }
        if !self.PackageClientFightConfig.is_empty() {
            crate::wire::write_tag(out, 17, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PackageClientFightConfig);
        }
        if self.LimitState != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitState);
        }
        if self.FsmVersion != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmVersion);
        }
        if self.ConfirmQuestResource {
            crate::wire::write_tag(out, 20, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.QuestReourceState != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestReourceState);
        }
        if self.IsLowMemorePlatform {
            crate::wire::write_tag(out, 22, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.BlockState {
            crate::wire::write_tag(out, 23, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.downloadResourceQuestId {
            crate::wire::write_tag(out, 24, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.XboxUserId.is_empty() {
            crate::wire::write_tag(out, 25, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxUserId);
        }
        if !self.XboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 26, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxOnlineId);
        }
        if !self.XboxAccountId.is_empty() {
            crate::wire::write_tag(out, 27, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxAccountId);
        }
        if self.XboxSocialState != 0 {
            crate::wire::write_tag(out, 28, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.XboxSocialState);
        }
        if !self.XstsToken.is_empty() {
            crate::wire::write_tag(out, 30, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XstsToken);
        }
        if !self.GradedResourceVersion.is_empty() {
            crate::wire::write_tag(out, 31, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GradedResourceVersion);
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
                let val = <DevLoginCheckData as PbMessage>::decode(bytes)?;
                self.DevLoginCheck = Some(LoginRequestDevLoginCheckOneof::DevLoginCheckData(val));
            }
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Account = r.read_len_delimited()?.to_vec(),
            3 => self.LoginTraceId = r.read_len_delimited()?.to_vec(),
            4 => self.Token = r.read_len_delimited()?.to_vec(),
            5 => self.AppVersion = r.read_len_delimited()?.to_vec(),
            6 => self.LauncherVersion = r.read_len_delimited()?.to_vec(),
            7 => self.ResourceVersion = r.read_len_delimited()?.to_vec(),
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ClientBasicInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PublicResourceVersionInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AceBlackProductAccountInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.PushNotificationsEnabled = r.read_bool()?,
            13 => self.ClientId = r.read_len_delimited()?.to_vec(),
            14 => self.SdkUserId = r.read_len_delimited()?.to_vec(),
            15 => self.SdkOnlineId = r.read_len_delimited()?.to_vec(),
            16 => self.SdkAccountId = r.read_len_delimited()?.to_vec(),
            17 => self.PackageClientFightConfig = r.read_len_delimited()?.to_vec(),
            18 => self.LimitState = r.read_varint_i32()?,
            19 => self.FsmVersion = r.read_varint_i32()?,
            20 => self.ConfirmQuestResource = r.read_bool()?,
            21 => self.QuestReourceState = r.read_varint_i32()?,
            22 => self.IsLowMemorePlatform = r.read_bool()?,
            23 => {
                self.BlockState = Some(<BlockState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            24 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.downloadResourceQuestId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            25 => self.XboxUserId = r.read_len_delimited()?.to_vec(),
            26 => self.XboxOnlineId = r.read_len_delimited()?.to_vec(),
            27 => self.XboxAccountId = r.read_len_delimited()?.to_vec(),
            28 => self.XboxSocialState = r.read_varint_i32()?,
            30 => self.XstsToken = r.read_len_delimited()?.to_vec(),
            31 => self.GradedResourceVersion = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoginResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ReconnectToken: Vec<u8>,
    pub Timestamp: i64,
    pub Platform: Vec<u8>,
    pub ClientWaitingMode: i32,
    pub ClientWaitingTime: i32,
    pub ClientAutoInInterval: i32,
    pub ClientDisplayTime: i32,
}

impl LoginResponse {
    pub const MSG_ID: u16 = 104;
}

impl PbMessage for LoginResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ReconnectToken.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReconnectToken);
        }
        if self.Timestamp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Timestamp);
        }
        if !self.Platform.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Platform);
        }
        if self.ClientWaitingMode != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientWaitingMode);
        }
        if self.ClientWaitingTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientWaitingTime);
        }
        if self.ClientAutoInInterval != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientAutoInInterval);
        }
        if self.ClientDisplayTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientDisplayTime);
        }
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
            2 => self.ReconnectToken = r.read_len_delimited()?.to_vec(),
            3 => self.Timestamp = r.read_varint_i64()?,
            4 => self.Platform = r.read_len_delimited()?.to_vec(),
            5 => self.ClientWaitingMode = r.read_varint_i32()?,
            6 => self.ClientWaitingTime = r.read_varint_i32()?,
            7 => self.ClientAutoInInterval = r.read_varint_i32()?,
            8 => self.ClientDisplayTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateCharacterRequest {
    pub Sex: i32,
    pub Name: Vec<u8>,
}

impl CreateCharacterRequest {
    pub const MSG_ID: u16 = 101;
}

impl PbMessage for CreateCharacterRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Sex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sex);
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
            1 => self.Sex = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateCharacterResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Name: Vec<u8>,
    pub PlayerId: i32,
    pub CreateTime: i32,
}

impl CreateCharacterResponse {
    pub const MSG_ID: u16 = 102;
}

impl PbMessage for CreateCharacterResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.CreateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CreateTime);
        }
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
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => self.PlayerId = r.read_varint_i32()?,
            4 => self.CreateTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterGameRequest {
    pub SingleInstanceId: i32,
    pub MultiInstanceId: i32,
    pub Mode: i32,
    pub Pos: Option<Vector>,
}

impl EnterGameRequest {
    pub const MSG_ID: u16 = 105;
}

impl PbMessage for EnterGameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SingleInstanceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SingleInstanceId);
        }
        if self.MultiInstanceId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MultiInstanceId);
        }
        if self.Mode != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Mode);
        }
        if let Some(v) = &self.Pos {
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
            1 => self.SingleInstanceId = r.read_varint_i32()?,
            2 => self.MultiInstanceId = r.read_varint_i32()?,
            3 => self.Mode = r.read_varint_i32()?,
            4 => {
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
pub struct EnterGameResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ClientWaitingMode: i32,
    pub ClientWaitingTime: i32,
    pub ClientAutoInInterval: i32,
}

impl EnterGameResponse {
    pub const MSG_ID: u16 = 106;
}

impl PbMessage for EnterGameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ClientWaitingMode != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientWaitingMode);
        }
        if self.ClientWaitingTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientWaitingTime);
        }
        if self.ClientAutoInInterval != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientAutoInInterval);
        }
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
            2 => self.ClientWaitingMode = r.read_varint_i32()?,
            3 => self.ClientWaitingTime = r.read_varint_i32()?,
            4 => self.ClientAutoInInterval = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReconnectRequest {
    pub PlayerId: i32,
    pub LastSvrSeqNo: i32,
    pub ReconnectToken: Vec<u8>,
    pub ReconnectTraceId: Vec<u8>,
}

impl ReconnectRequest {
    pub const MSG_ID: u16 = 107;
}

impl PbMessage for ReconnectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.LastSvrSeqNo != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastSvrSeqNo);
        }
        if !self.ReconnectToken.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReconnectToken);
        }
        if !self.ReconnectTraceId.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReconnectTraceId);
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
            2 => self.LastSvrSeqNo = r.read_varint_i32()?,
            3 => self.ReconnectToken = r.read_len_delimited()?.to_vec(),
            4 => self.ReconnectTraceId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReconnectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LastRecvSeqNo: i32,
    pub Timestamp: i64,
    pub IsPermittedSilentLogin: bool,
}

impl ReconnectResponse {
    pub const MSG_ID: u16 = 108;
}

impl PbMessage for ReconnectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LastRecvSeqNo != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRecvSeqNo);
        }
        if self.Timestamp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Timestamp);
        }
        if self.IsPermittedSilentLogin {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.LastRecvSeqNo = r.read_varint_i32()?,
            3 => self.Timestamp = r.read_varint_i64()?,
            4 => self.IsPermittedSilentLogin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogoutNotifyBanOneof {
    BanInfo(BanLogoutInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LogoutNotify {
    pub Ban: Option<LogoutNotifyBanOneof>,
    pub ErrorCode: Option<ErrorCode>,
    pub logoutReason: i32,
}

impl LogoutNotify {
    pub const MSG_ID: u16 = 110;
}

impl PbMessage for LogoutNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Ban {
            match oneof {
                LogoutNotifyBanOneof::BanInfo(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.logoutReason != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.logoutReason);
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
                let val = <BanLogoutInfo as PbMessage>::decode(bytes)?;
                self.Ban = Some(LogoutNotifyBanOneof::BanInfo(val));
            }
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.logoutReason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProtoKeyRequest {
    pub IsLogin: bool,
    pub TraceId: Vec<u8>,
}

impl ProtoKeyRequest {
    pub const MSG_ID: u16 = 111;
}

impl PbMessage for ProtoKeyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsLogin {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.TraceId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TraceId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsLogin = r.read_bool()?,
            2 => self.TraceId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProtoKeyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Type: i32,
    pub Key: Vec<u8>,
}

impl ProtoKeyResponse {
    pub const MSG_ID: u16 = 112;
}

impl PbMessage for ProtoKeyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Type != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        if !self.Key.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.Type = r.read_varint_i32()?,
            3 => self.Key = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PushDataCompleteNotify {}

impl PushDataCompleteNotify {
    pub const MSG_ID: u16 = 113;
}

impl PbMessage for PushDataCompleteNotify {
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
pub struct VersionInfoPush {
    pub AppVersion: Vec<u8>,
    pub LauncherVersion: Vec<u8>,
    pub ResourceVersion: Vec<u8>,
}

impl VersionInfoPush {
    pub const MSG_ID: u16 = 116;
}

impl PbMessage for VersionInfoPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AppVersion.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AppVersion);
        }
        if !self.LauncherVersion.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LauncherVersion);
        }
        if !self.ResourceVersion.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourceVersion);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AppVersion = r.read_len_delimited()?.to_vec(),
            2 => self.LauncherVersion = r.read_len_delimited()?.to_vec(),
            3 => self.ResourceVersion = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadingConfigRequest {}

impl LoadingConfigRequest {
    pub const MSG_ID: u16 = 15203;
}

impl PbMessage for LoadingConfigRequest {
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
pub struct LoadingConfigResponse {
    pub LoadingConfig: Vec<LoadingConfig>,
}

impl LoadingConfigResponse {
    pub const MSG_ID: u16 = 20507;
}

impl PbMessage for LoadingConfigResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LoadingConfig {
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
                self.LoadingConfig
                    .push(<LoadingConfig as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadingConfig {
    pub Id: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for LoadingConfig {
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
pub struct PublicResourceVersionInfo {
    pub PublicJsonVersion: i32,
    pub PublicMiscVersion: i32,
    pub PublicUniverseEditorVersion: i32,
}

impl PbMessage for PublicResourceVersionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PublicJsonVersion != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PublicJsonVersion);
        }
        if self.PublicMiscVersion != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PublicMiscVersion);
        }
        if self.PublicUniverseEditorVersion != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PublicUniverseEditorVersion);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PublicJsonVersion = r.read_varint_i32()?,
            2 => self.PublicMiscVersion = r.read_varint_i32()?,
            3 => self.PublicUniverseEditorVersion = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbMailAttachment {
    pub Id: i32,
    pub Count: i32,
}

impl PbMessage for PbMailAttachment {
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
pub struct PbMailInfo {
    pub Id: Vec<u8>,
    pub ReceivedTime: i64,
    pub ReadTime: i64,
    pub State: i32,
    pub Level: Option<MailLevel>,
    pub Title: Vec<u8>,
    pub Content: Vec<u8>,
    pub Sender: Vec<u8>,
    pub ValidTime: i32,
    pub ReadValidTime: i32,
    pub Attachments: Vec<PbMailAttachment>,
    pub ConfigId: i32,
    pub ExpiryTime: i64,
    pub IsIntervalMail: bool,
}

impl PbMessage for PbMailInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Id);
        }
        if self.ReceivedTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReceivedTime);
        }
        if self.ReadTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReadTime);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if let Some(v) = &self.Level {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Title.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Title);
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
        if !self.Sender.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Sender);
        }
        if self.ValidTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ValidTime);
        }
        if self.ReadValidTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReadValidTime);
        }
        for item in &self.Attachments {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.ExpiryTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExpiryTime);
        }
        if self.IsIntervalMail {
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
            1 => self.Id = r.read_len_delimited()?.to_vec(),
            2 => self.ReceivedTime = r.read_varint_i64()?,
            3 => self.ReadTime = r.read_varint_i64()?,
            4 => self.State = r.read_varint_i32()?,
            5 => {
                self.Level = Some(<MailLevel as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.Title = r.read_len_delimited()?.to_vec(),
            7 => self.Content = r.read_len_delimited()?.to_vec(),
            8 => self.Sender = r.read_len_delimited()?.to_vec(),
            9 => self.ValidTime = r.read_varint_i32()?,
            10 => self.ReadValidTime = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Attachments
                    .push(<PbMailAttachment as PbMessage>::decode(bytes)?);
            }
            12 => self.ConfigId = r.read_varint_i32()?,
            13 => self.ExpiryTime = r.read_varint_i64()?,
            14 => self.IsIntervalMail = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailInfosNotify {
    pub MailInfos: Vec<PbMailInfo>,
}

impl MailInfosNotify {
    pub const MSG_ID: u16 = 20632;
}

impl PbMessage for MailInfosNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MailInfos {
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
                self.MailInfos
                    .push(<PbMailInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailBindInfoRequest {}

impl MailBindInfoRequest {
    pub const MSG_ID: u16 = 19713;
}

impl PbMessage for MailBindInfoRequest {
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
pub struct MailBindInfoResponse {
    pub MailBind: Option<MailBind>,
}

impl MailBindInfoResponse {
    pub const MSG_ID: u16 = 24222;
}

impl PbMessage for MailBindInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MailBind {
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
                self.MailBind
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailBind {
    pub IsBind: bool,
    pub IsReward: bool,
    pub CloseTime: i64,
}

impl PbMessage for MailBind {
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
pub struct MarkPointInfo {
    pub PosX: f32,
    pub PosY: f32,
    pub PosZ: f32,
    pub ConfigId: i32,
    pub MarkId: i32,
    pub MapId: i32,
    pub MarkType: i32,
    pub EntityConfigId: i32,
    pub IsServerDisable: bool,
    pub MarkPointState: Option<MarkPointState>,
}

impl PbMessage for MarkPointInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.PosZ != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosZ).to_bits().to_le_bytes());
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.MarkType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkType);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.IsServerDisable {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.MarkPointState {
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
            1 => self.PosX = r.read_f32()?,
            2 => self.PosY = r.read_f32()?,
            3 => self.PosZ = r.read_f32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            5 => self.MarkId = r.read_varint_i32()?,
            6 => self.MapId = r.read_varint_i32()?,
            7 => self.MarkType = r.read_varint_i32()?,
            8 => self.EntityConfigId = r.read_varint_i32()?,
            9 => self.IsServerDisable = r.read_bool()?,
            10 => {
                self.MarkPointState = Some(
                    <MarkPointState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct MarkPointRequestInfo {
    pub PosX: f32,
    pub PosY: f32,
    pub PosZ: f32,
    pub ConfigId: i32,
    pub MarkType: i32,
    pub MarkInfo: bool,
    pub IsTrace: bool,
    pub MapId: i32,
    pub EntityConfigId: i32,
}

impl PbMessage for MarkPointRequestInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.PosZ != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosZ).to_bits().to_le_bytes());
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.MarkType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkType);
        }
        if self.IsTrace {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
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
            1 => self.PosX = r.read_f32()?,
            2 => self.PosY = r.read_f32()?,
            3 => self.PosZ = r.read_f32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            5 => self.MarkType = r.read_varint_i32()?,
            6 => self.IsTrace = r.read_bool()?,
            7 => self.MapId = r.read_varint_i32()?,
            8 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreasureBoxParam {
    pub TreasureBox: Vec<MarkPointRequestInfo>,
    pub DetectionSlotId: i64,
}

impl PbMessage for TreasureBoxParam {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TreasureBox {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.DetectionSlotId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DetectionSlotId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TreasureBox
                    .push(<MarkPointRequestInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.DetectionSlotId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarkTreasureBoxInfo {
    pub MarkPointInfo: Vec<MarkPointInfo>,
}

impl PbMessage for MarkTreasureBoxInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkPointInfo {
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
                self.MarkPointInfo
                    .push(<MarkPointInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttachMarkInfo {
    pub MarkPointInfo: Vec<MarkPointInfo>,
}

impl PbMessage for AttachMarkInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkPointInfo {
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
                self.MarkPointInfo
                    .push(<MarkPointInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapUnlockFieldInfoRequest {}

impl MapUnlockFieldInfoRequest {
    pub const MSG_ID: u16 = 15442;
}

impl PbMessage for MapUnlockFieldInfoRequest {
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
pub struct MapUnlockFieldInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FieldId: Vec<i32>,
}

impl MapUnlockFieldInfoResponse {
    pub const MSG_ID: u16 = 19902;
}

impl PbMessage for MapUnlockFieldInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FieldId {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FieldId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkShowInfo {
    pub MarkId: i32,
    pub ShowFlag: u32,
}

impl PbMessage for MapMarkShowInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if self.ShowFlag != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.ShowFlag));
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
            2 => self.ShowFlag = r.read_varint_u32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemporaryTeleportParam {
    pub temporaryTeleportId: i64,
}

impl PbMessage for TemporaryTeleportParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.temporaryTeleportId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.temporaryTeleportId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.temporaryTeleportId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapMarkRequestParamsOneof {
    TemporaryTeleportParam(TemporaryTeleportParam),
    TreasureBoxParam(TreasureBoxParam),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkRequest {
    pub Params: Option<MapMarkRequestParamsOneof>,
    pub MarkPointRequestInfo: Option<MarkPointRequestInfo>,
}

impl MapMarkRequest {
    pub const MSG_ID: u16 = 16871;
}

impl PbMessage for MapMarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Params {
            match oneof {
                MapMarkRequestParamsOneof::TemporaryTeleportParam(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                MapMarkRequestParamsOneof::TreasureBoxParam(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if let Some(v) = &self.MarkPointRequestInfo {
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
                let bytes = r.read_len_delimited()?;
                let val = <TemporaryTeleportParam as PbMessage>::decode(bytes)?;
                self.Params = Some(MapMarkRequestParamsOneof::TemporaryTeleportParam(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <TreasureBoxParam as PbMessage>::decode(bytes)?;
                self.Params = Some(MapMarkRequestParamsOneof::TreasureBoxParam(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.MarkPointRequestInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Info: Option<MarkPointInfo>,
}

impl MapMarkResponse {
    pub const MSG_ID: u16 = 18621;
}

impl PbMessage for MapMarkResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
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
pub struct RemoveMapMarkRequest {
    pub MarkList: Vec<i32>,
}

impl RemoveMapMarkRequest {
    pub const MSG_ID: u16 = 16284;
}

impl PbMessage for RemoveMapMarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkList {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveMapMarkResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MarkList: Vec<i32>,
}

impl RemoveMapMarkResponse {
    pub const MSG_ID: u16 = 29212;
}

impl PbMessage for RemoveMapMarkResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.MarkList {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapUnlockFieldNotify {
    pub FieldId: i32,
}

impl MapUnlockFieldNotify {
    pub const MSG_ID: u16 = 22762;
}

impl PbMessage for MapUnlockFieldNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FieldId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FieldId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.FieldId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTraceInfoRequest {}

impl MapTraceInfoRequest {
    pub const MSG_ID: u16 = 20733;
}

impl PbMessage for MapTraceInfoRequest {
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
pub struct MapTraceInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MarkIdList: Vec<i32>,
}

impl MapTraceInfoResponse {
    pub const MSG_ID: u16 = 20456;
}

impl PbMessage for MapTraceInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.MarkIdList {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTraceRequest {
    pub MarkId: i32,
}

impl MapTraceRequest {
    pub const MSG_ID: u16 = 24772;
}

impl PbMessage for MapTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTraceResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MarkId: i32,
}

impl MapTraceResponse {
    pub const MSG_ID: u16 = 24396;
}

impl PbMessage for MapTraceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapCancelTraceRequest {
    pub MarkId: i32,
}

impl MapCancelTraceRequest {
    pub const MSG_ID: u16 = 17205;
}

impl PbMessage for MapCancelTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            2 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapCancelTraceResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MarkId: i32,
}

impl MapCancelTraceResponse {
    pub const MSG_ID: u16 = 26282;
}

impl PbMessage for MapCancelTraceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkInfoNotify {
    pub InfoList: Vec<MarkPointInfo>,
    pub ShowMarkIds: Vec<MapMarkShowInfo>,
    pub UnlockMarkIds: Vec<i32>,
    pub EntityMapMarkInfo: Vec<EntityMapMarkInfoPb>,
    pub SystemMarkHideInfo: Vec<SystemMarkHideInfoPb>,
    pub CompleteMarkIds: Vec<i32>,
}

impl MapMarkInfoNotify {
    pub const MSG_ID: u16 = 27725;
}

impl PbMessage for MapMarkInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InfoList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.ShowMarkIds {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.UnlockMarkIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.EntityMapMarkInfo {
            crate::wire::write_submessage(out, 14, item);
        }
        for item in &self.SystemMarkHideInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.CompleteMarkIds {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.InfoList
                    .push(<MarkPointInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ShowMarkIds
                    .push(<MapMarkShowInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockMarkIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.EntityMapMarkInfo
                    .push(<EntityMapMarkInfoPb as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SystemMarkHideInfo
                    .push(<SystemMarkHideInfoPb as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompleteMarkIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkAddNotify {
    pub Info: Option<MarkPointInfo>,
    pub TreasureBoxMarkInfo: Option<MarkTreasureBoxInfo>,
    pub AttackMark: Option<AttachMarkInfo>,
}

impl MapMarkAddNotify {
    pub const MSG_ID: u16 = 19623;
}

impl PbMessage for MapMarkAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.TreasureBoxMarkInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.AttackMark {
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
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TreasureBoxMarkInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.AttackMark
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapUnlockDataNotify {
    pub UnlockMultiMapIds: Vec<i32>,
    pub UnlockMapBlockIds: Vec<i32>,
}

impl MapUnlockDataNotify {
    pub const MSG_ID: u16 = 20261;
}

impl PbMessage for MapUnlockDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockMultiMapIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockMapBlockIds {
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockMultiMapIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockMapBlockIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityMapMarkInfoPb {
    pub InstId: i32,
    pub TemplateId: i32,
    pub Pos: Option<Vector>,
}

impl PbMessage for EntityMapMarkInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.TemplateId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TemplateId);
        }
        if let Some(v) = &self.Pos {
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
            2 => self.TemplateId = r.read_varint_i32()?,
            3 => {
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
pub struct SystemMarkHideInfoPb {
    pub Id: i32,
    pub MapId: i32,
    pub HideInfo: Vec<u8>,
}

impl PbMessage for SystemMarkHideInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if !self.HideInfo.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.HideInfo);
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
            1 => self.MapId = r.read_varint_i32()?,
            3 => self.HideInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonthCardRequest {}

impl MonthCardRequest {
    pub const MSG_ID: u16 = 22235;
}

impl PbMessage for MonthCardRequest {
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
pub struct MonthCardResponse {
    pub Days: i32,
    pub IsDailyGot: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl MonthCardResponse {
    pub const MSG_ID: u16 = 26286;
}

impl PbMessage for MonthCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Days != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Days);
        }
        if self.IsDailyGot {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            7 => self.Days = r.read_varint_i32()?,
            11 => self.IsDailyGot = r.read_bool()?,
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
pub struct MonthCardDailyRewardNotify {
    pub ItemId: i32,
    pub Count: i32,
    pub Days: i32,
}

impl MonthCardDailyRewardNotify {
    pub const MSG_ID: u16 = 28994;
}

impl PbMessage for MonthCardDailyRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.Days != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Days);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.ItemId = r.read_varint_i32()?,
            15 => self.Count = r.read_varint_i32()?,
            1 => self.Days = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTaskProcessPb {
    pub Current: i32,
    pub Target: i32,
}

impl PbMessage for MotorTaskProcessPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Current != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Current);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Current = r.read_varint_i32()?,
            2 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTaskRewardPb {
    pub Rewarded: i32,
    pub WaitReward: i32,
    pub MaxReward: i32,
}

impl PbMessage for MotorTaskRewardPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Rewarded != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rewarded);
        }
        if self.WaitReward != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaitReward);
        }
        if self.MaxReward != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReward);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Rewarded = r.read_varint_i32()?,
            2 => self.WaitReward = r.read_varint_i32()?,
            3 => self.MaxReward = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTaskPb {
    pub Id: i32,
    pub Type: Option<MotorTaskTypePb>,
    pub Process: Option<MotorTaskProcessPb>,
    pub Reward: Option<MotorTaskRewardPb>,
    pub EndTime: i64,
    pub StartTime: i64,
}

impl PbMessage for MotorTaskPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Process {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Reward {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
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
                self.Type = Some(
                    <MotorTaskTypePb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Process
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.EndTime = r.read_varint_i64()?,
            6 => self.StartTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorInfoRequest {}

impl MotorInfoRequest {
    pub const MSG_ID: u16 = 17813;
}

impl PbMessage for MotorInfoRequest {
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
pub struct MotorInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Motor: Option<MotorPb>,
}

impl MotorInfoResponse {
    pub const MSG_ID: u16 = 22936;
}

impl PbMessage for MotorInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Motor {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Motor
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechLevelUpRequest {
    pub TechId: i32,
}

impl MotorTechLevelUpRequest {
    pub const MSG_ID: u16 = 27727;
}

impl PbMessage for MotorTechLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TechId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TechId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.TechId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Tree: Option<MotorTechOneTreePb>,
}

impl MotorTechLevelUpResponse {
    pub const MSG_ID: u16 = 15127;
}

impl PbMessage for MotorTechLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Tree {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Tree
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechTreeSwitchRequest {
    pub TreeId: i32,
}

impl MotorTechTreeSwitchRequest {
    pub const MSG_ID: u16 = 25166;
}

impl PbMessage for MotorTechTreeSwitchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => self.TreeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechTreeSwitchResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TreeInUse: i32,
}

impl MotorTechTreeSwitchResponse {
    pub const MSG_ID: u16 = 17554;
}

impl PbMessage for MotorTechTreeSwitchResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TreeInUse != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeInUse);
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
            1 => self.TreeInUse = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorLevelOneKeyRewardRequest {}

impl MotorLevelOneKeyRewardRequest {
    pub const MSG_ID: u16 = 15900;
}

impl PbMessage for MotorLevelOneKeyRewardRequest {
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
pub struct MotorLevelOneKeyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorRewardedLvMax: i32,
}

impl MotorLevelOneKeyRewardResponse {
    pub const MSG_ID: u16 = 17604;
}

impl PbMessage for MotorLevelOneKeyRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MotorRewardedLvMax != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorRewardedLvMax);
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
            12 => self.MotorRewardedLvMax = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTaskOneKeyRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl MotorTaskOneKeyRewardRequest {
    pub const MSG_ID: u16 = 27567;
}

impl PbMessage for MotorTaskOneKeyRewardRequest {
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
pub struct MotorTaskOneKeyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorTaskOneKeyRewardResponse {
    pub const MSG_ID: u16 = 26986;
}

impl PbMessage for MotorTaskOneKeyRewardResponse {
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
pub struct MotorTaskUpdateNotify {
    pub Task: Vec<MotorTaskPb>,
}

impl MotorTaskUpdateNotify {
    pub const MSG_ID: u16 = 22342;
}

impl PbMessage for MotorTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
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
                self.Task.push(<MotorTaskPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorLockedTechUpdateNotify {
    pub TreeId: i32,
    pub Tech: Vec<MotorTechPb>,
}

impl MotorLockedTechUpdateNotify {
    pub const MSG_ID: u16 = 23811;
}

impl PbMessage for MotorLockedTechUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeId);
        }
        for item in &self.Tech {
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
            7 => self.TreeId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Tech.push(<MotorTechPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechPb {
    pub Id: i32,
    pub Level: i32,
    pub Unlock: bool,
    pub Current: i32,
    pub Target: i32,
}

impl PbMessage for MotorTechPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Current != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Current);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.Unlock = r.read_bool()?,
            10 => self.Current = r.read_varint_i32()?,
            11 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechOneTreePb {
    pub TreeId: i32,
    pub Tech: Vec<MotorTechPb>,
}

impl PbMessage for MotorTechOneTreePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeId);
        }
        for item in &self.Tech {
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
            1 => self.TreeId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Tech.push(<MotorTechPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorPb {
    pub MotorLevel: i32,
    pub MotorExp: i32,
    pub MotorRewardedLvMax: i32,
    pub UnlockedTree: Vec<MotorTechOneTreePb>,
    pub TreeInUse: i32,
    pub TaskTrees: Vec<MotorTaskTreePb>,
    pub MotorExpLimitGainDaily: i32,
    pub MotorExpMonsterDropDailyLimit: i32,
}

impl PbMessage for MotorPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorLevel);
        }
        if self.MotorExp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorExp);
        }
        if self.MotorRewardedLvMax != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorRewardedLvMax);
        }
        for item in &self.UnlockedTree {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.TreeInUse != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeInUse);
        }
        for item in &self.TaskTrees {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.MotorExpLimitGainDaily != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorExpLimitGainDaily);
        }
        if self.MotorExpMonsterDropDailyLimit != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorExpMonsterDropDailyLimit);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MotorLevel = r.read_varint_i32()?,
            2 => self.MotorExp = r.read_varint_i32()?,
            3 => self.MotorRewardedLvMax = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockedTree
                    .push(<MotorTechOneTreePb as PbMessage>::decode(bytes)?);
            }
            5 => self.TreeInUse = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TaskTrees
                    .push(<MotorTaskTreePb as PbMessage>::decode(bytes)?);
            }
            7 => self.MotorExpLimitGainDaily = r.read_varint_i32()?,
            8 => self.MotorExpMonsterDropDailyLimit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTaskTreePb {
    pub TreeId: i32,
    pub Tasks: Vec<MotorTaskPb>,
    pub TpRewarded: i32,
}

impl PbMessage for MotorTaskTreePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeId);
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.TpRewarded != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TpRewarded);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks.push(<MotorTaskPb as PbMessage>::decode(bytes)?);
            }
            3 => self.TpRewarded = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorCreateRequest {
    pub IsCreate: bool,
}

impl MotorCreateRequest {
    pub const MSG_ID: u16 = 29157;
}

impl PbMessage for MotorCreateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsCreate {
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
            12 => self.IsCreate = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorCreateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorCreateResponse {
    pub const MSG_ID: u16 = 16761;
}

impl PbMessage for MotorCreateResponse {
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
pub struct OrnamentInfo {
    pub UnlockOrnamentIds: Vec<i32>,
    pub OrnamentDressInfos: Vec<OrnamentDressInfo>,
    pub RedPointOrnamentIds: Vec<i32>,
}

impl PbMessage for OrnamentInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockOrnamentIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.OrnamentDressInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RedPointOrnamentIds {
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
                &mut self.UnlockOrnamentIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.OrnamentDressInfos
                    .push(<OrnamentDressInfo as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RedPointOrnamentIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrnamentDressInfo {
    pub RoleSkinId: i32,
    pub DressOrnamentIds: Vec<i32>,
}

impl PbMessage for OrnamentDressInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        for item in &self.DressOrnamentIds {
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
            1 => self.RoleSkinId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DressOrnamentIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrnamentInfoNotify {
    pub OrnamentInfo: Option<OrnamentInfo>,
}

impl OrnamentInfoNotify {
    pub const MSG_ID: u16 = 15327;
}

impl PbMessage for OrnamentInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OrnamentInfo {
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
                self.OrnamentInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrnamentDressInfoUpdateNotify {
    pub OrnamentDressInfos: Vec<OrnamentDressInfo>,
}

impl OrnamentDressInfoUpdateNotify {
    pub const MSG_ID: u16 = 29379;
}

impl PbMessage for OrnamentDressInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OrnamentDressInfos {
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
                self.OrnamentDressInfos
                    .push(<OrnamentDressInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeOrnamentRequest {
    pub RoleSkinId: i32,
    pub OrnamentId: i32,
    pub IsDress: bool,
}

impl ChangeOrnamentRequest {
    pub const MSG_ID: u16 = 16754;
}

impl PbMessage for ChangeOrnamentRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.OrnamentId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OrnamentId);
        }
        if self.IsDress {
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
            14 => self.RoleSkinId = r.read_varint_i32()?,
            4 => self.OrnamentId = r.read_varint_i32()?,
            10 => self.IsDress = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeOrnamentResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeOrnamentResponse {
    pub const MSG_ID: u16 = 23664;
}

impl PbMessage for ChangeOrnamentResponse {
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
pub struct PayItemInfo {
    pub Id: i32,
    pub PayId: i32,
    pub ItemId: i32,
    pub ItemCount: i32,
    pub BonusItemCount: i32,
    pub SpecialBonusItemCount: i32,
    pub CanSpecialBonus: bool,
    pub StageImage: Vec<u8>,
    pub ProductId: Vec<u8>,
    pub Amount: Vec<u8>,
    pub ComplianceDetail: Vec<u8>,
    pub Quality: i32,
}

impl PbMessage for PayItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.PayId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PayId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.BonusItemCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BonusItemCount);
        }
        if self.SpecialBonusItemCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpecialBonusItemCount);
        }
        if self.CanSpecialBonus {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.StageImage.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StageImage);
        }
        if !self.ProductId.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ProductId);
        }
        if !self.Amount.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Amount);
        }
        if !self.ComplianceDetail.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ComplianceDetail);
        }
        if self.Quality != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Quality);
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
            2 => self.PayId = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            4 => self.ItemCount = r.read_varint_i32()?,
            5 => self.BonusItemCount = r.read_varint_i32()?,
            6 => self.SpecialBonusItemCount = r.read_varint_i32()?,
            7 => self.CanSpecialBonus = r.read_bool()?,
            8 => self.StageImage = r.read_len_delimited()?.to_vec(),
            9 => self.ProductId = r.read_len_delimited()?.to_vec(),
            10 => self.Amount = r.read_len_delimited()?.to_vec(),
            11 => self.ComplianceDetail = r.read_len_delimited()?.to_vec(),
            12 => self.Quality = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftInfo {
    pub Id: i32,
    pub PayId: i32,
    pub ItemId: i32,
    pub ItemCount: i32,
    pub Sort: i32,
    pub BuyLimit: i32,
    pub BoughtCount: i32,
    pub StageImage: Vec<u8>,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub ProductId: Vec<u8>,
    pub Amount: Vec<u8>,
    pub TabId: i32,
    pub Type: i32,
    pub Locked: bool,
    pub IsCanBuy: bool,
    pub IsRemind: bool,
    pub BuyCondition: i32,
    pub CloudGameTime: i32,
    pub CloudGameIcon: Vec<u8>,
    pub Desc: Vec<u8>,
    pub UpdateType: Option<PayUpdateType>,
    pub UpdateTime: i64,
    pub LastUpdateTime: i64,
    pub Tag: i32,
    pub PromotionShow: i32,
    pub ShowStageImage: Vec<u8>,
    pub CurrencyDiscountTags: BTreeMap<Vec<u8>, i32>,
    pub ComplianceDetail: Vec<u8>,
    pub Quality: i32,
    pub VersionId: i32,
    pub ShopId: i32,
    pub NeedConsoleRulePrompt: bool,
}

impl PbMessage for PayGiftInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.PayId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PayId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.Sort != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sort);
        }
        if self.BuyLimit != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyLimit);
        }
        if self.BoughtCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoughtCount);
        }
        if !self.StageImage.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StageImage);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if !self.ProductId.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ProductId);
        }
        if !self.Amount.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Amount);
        }
        if self.TabId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabId);
        }
        if self.Type != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        if self.Locked {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsCanBuy {
            crate::wire::write_tag(out, 16, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRemind {
            crate::wire::write_tag(out, 17, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BuyCondition != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyCondition);
        }
        if self.CloudGameTime != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CloudGameTime);
        }
        if !self.CloudGameIcon.is_empty() {
            crate::wire::write_tag(out, 20, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CloudGameIcon);
        }
        if !self.Desc.is_empty() {
            crate::wire::write_tag(out, 21, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Desc);
        }
        if let Some(v) = &self.UpdateType {
            crate::wire::write_tag(out, 22, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UpdateTime != 0 {
            crate::wire::write_tag(out, 23, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpdateTime);
        }
        if self.LastUpdateTime != 0 {
            crate::wire::write_tag(out, 24, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastUpdateTime);
        }
        if self.Tag != 0 {
            crate::wire::write_tag(out, 25, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Tag);
        }
        if self.PromotionShow != 0 {
            crate::wire::write_tag(out, 26, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PromotionShow);
        }
        if !self.ShowStageImage.is_empty() {
            crate::wire::write_tag(out, 27, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ShowStageImage);
        }
        if !self.CurrencyDiscountTags.is_empty() {
            crate::wire::write_map(out, 28, &self.CurrencyDiscountTags);
        }
        if !self.ComplianceDetail.is_empty() {
            crate::wire::write_tag(out, 29, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ComplianceDetail);
        }
        if self.Quality != 0 {
            crate::wire::write_tag(out, 30, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Quality);
        }
        if self.VersionId != 0 {
            crate::wire::write_tag(out, 31, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VersionId);
        }
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 32, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.NeedConsoleRulePrompt {
            crate::wire::write_tag(out, 33, WireType::Varint);
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
            2 => self.PayId = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            4 => self.ItemCount = r.read_varint_i32()?,
            5 => self.Sort = r.read_varint_i32()?,
            6 => self.BuyLimit = r.read_varint_i32()?,
            7 => self.BoughtCount = r.read_varint_i32()?,
            8 => self.StageImage = r.read_len_delimited()?.to_vec(),
            9 => self.BeginTime = r.read_varint_i64()?,
            10 => self.EndTime = r.read_varint_i64()?,
            11 => self.ProductId = r.read_len_delimited()?.to_vec(),
            12 => self.Amount = r.read_len_delimited()?.to_vec(),
            13 => self.TabId = r.read_varint_i32()?,
            14 => self.Type = r.read_varint_i32()?,
            15 => self.Locked = r.read_bool()?,
            16 => self.IsCanBuy = r.read_bool()?,
            17 => self.IsRemind = r.read_bool()?,
            18 => self.BuyCondition = r.read_varint_i32()?,
            19 => self.CloudGameTime = r.read_varint_i32()?,
            20 => self.CloudGameIcon = r.read_len_delimited()?.to_vec(),
            21 => self.Desc = r.read_len_delimited()?.to_vec(),
            22 => {
                self.UpdateType = Some(<PayUpdateType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            23 => self.UpdateTime = r.read_varint_i64()?,
            24 => self.LastUpdateTime = r.read_varint_i64()?,
            25 => self.Tag = r.read_varint_i32()?,
            26 => self.PromotionShow = r.read_varint_i32()?,
            27 => self.ShowStageImage = r.read_len_delimited()?.to_vec(),
            28 => crate::wire::decode_map_entry(r, &mut self.CurrencyDiscountTags)?,
            29 => self.ComplianceDetail = r.read_len_delimited()?.to_vec(),
            30 => self.Quality = r.read_varint_i32()?,
            31 => self.VersionId = r.read_varint_i32()?,
            32 => self.ShopId = r.read_varint_i32()?,
            33 => self.NeedConsoleRulePrompt = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayInfoRequest {
    pub Version: Vec<u8>,
}

impl PayInfoRequest {
    pub const MSG_ID: u16 = 23421;
}

impl PbMessage for PayInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayInfoResponse {
    pub Infos: Vec<PayItemInfo>,
    pub Version: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PayInfoResponse {
    pub const MSG_ID: u16 = 21829;
}

impl PbMessage for PayInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 12, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Infos.push(<PayItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.Version = r.read_len_delimited()?.to_vec(),
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
pub struct PayShopPrice {
    pub Id: i32,
    pub Count: i32,
    pub PromotionCount: i32,
}

impl PbMessage for PayShopPrice {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.PromotionCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PromotionCount);
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
            3 => self.PromotionCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopItem {
    pub Id: i32,
    pub ItemId: i32,
    pub ItemCount: i32,
    pub Locked: bool,
    pub BuyLimit: i32,
    pub BoughtCount: i32,
    pub Price: Option<PayShopPrice>,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub BeginPromotionTime: i64,
    pub EndPromotionTime: i64,
    pub UpdateType: Option<PayUpdateType>,
    pub UpdateTime: i64,
    pub ShopItemType: Option<PayShopItemType>,
    pub TagBeginTime: i64,
    pub TagEndTime: i64,
    pub CanBuyGoods: bool,
    pub IsRemind: bool,
    pub BuyLimitConditionId: i32,
    pub Coupons: Vec<i32>,
    pub LastUpdateTime: i64,
    pub StageImage: Vec<u8>,
    pub ShowStageImage: Vec<u8>,
    pub TabId: i32,
    pub ShopId: i32,
    pub Tag: i32,
    pub Sort: i32,
    pub PromotionShow: i32,
    pub SoldOut: bool,
    pub ActivityId: i32,
    pub Show: bool,
    pub ComplianceDetail: Vec<u8>,
    pub Quality: i32,
    pub DiscountSort: i32,
    pub OnceBuyLimit: i32,
    pub IsRecommend: bool,
    pub IsShowHaveNum: bool,
    pub IsBuyMaxButton: bool,
    pub ConfirmLimitCount: i32,
    pub ShowBeginTime: i64,
    pub ShowEndTime: i64,
}

impl PbMessage for PayShopItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.Locked {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BuyLimit != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyLimit);
        }
        if self.BoughtCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoughtCount);
        }
        if let Some(v) = &self.Price {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.BeginPromotionTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginPromotionTime);
        }
        if self.EndPromotionTime != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndPromotionTime);
        }
        if let Some(v) = &self.UpdateType {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UpdateTime != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpdateTime);
        }
        if let Some(v) = &self.ShopItemType {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TagBeginTime != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TagBeginTime);
        }
        if self.TagEndTime != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TagEndTime);
        }
        if self.CanBuyGoods {
            crate::wire::write_tag(out, 22, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRemind {
            crate::wire::write_tag(out, 23, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BuyLimitConditionId != 0 {
            crate::wire::write_tag(out, 24, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyLimitConditionId);
        }
        for item in &self.Coupons {
            crate::wire::write_tag(out, 25, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LastUpdateTime != 0 {
            crate::wire::write_tag(out, 26, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastUpdateTime);
        }
        if !self.StageImage.is_empty() {
            crate::wire::write_tag(out, 27, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StageImage);
        }
        if !self.ShowStageImage.is_empty() {
            crate::wire::write_tag(out, 28, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ShowStageImage);
        }
        if self.TabId != 0 {
            crate::wire::write_tag(out, 29, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabId);
        }
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 30, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.Tag != 0 {
            crate::wire::write_tag(out, 31, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Tag);
        }
        if self.Sort != 0 {
            crate::wire::write_tag(out, 32, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sort);
        }
        if self.PromotionShow != 0 {
            crate::wire::write_tag(out, 33, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PromotionShow);
        }
        if self.SoldOut {
            crate::wire::write_tag(out, 34, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 35, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Show {
            crate::wire::write_tag(out, 36, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.ComplianceDetail.is_empty() {
            crate::wire::write_tag(out, 37, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ComplianceDetail);
        }
        if self.Quality != 0 {
            crate::wire::write_tag(out, 38, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Quality);
        }
        if self.DiscountSort != 0 {
            crate::wire::write_tag(out, 39, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscountSort);
        }
        if self.OnceBuyLimit != 0 {
            crate::wire::write_tag(out, 40, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OnceBuyLimit);
        }
        if self.IsRecommend {
            crate::wire::write_tag(out, 41, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsShowHaveNum {
            crate::wire::write_tag(out, 42, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsBuyMaxButton {
            crate::wire::write_tag(out, 43, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ConfirmLimitCount != 0 {
            crate::wire::write_tag(out, 44, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfirmLimitCount);
        }
        if self.ShowBeginTime != 0 {
            crate::wire::write_tag(out, 45, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ShowBeginTime);
        }
        if self.ShowEndTime != 0 {
            crate::wire::write_tag(out, 46, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ShowEndTime);
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
            3 => self.ItemId = r.read_varint_i32()?,
            4 => self.ItemCount = r.read_varint_i32()?,
            5 => self.Locked = r.read_bool()?,
            6 => self.BuyLimit = r.read_varint_i32()?,
            7 => self.BoughtCount = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Price
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.BeginTime = r.read_varint_i64()?,
            10 => self.EndTime = r.read_varint_i64()?,
            11 => self.BeginPromotionTime = r.read_varint_i64()?,
            12 => self.EndPromotionTime = r.read_varint_i64()?,
            13 => {
                self.UpdateType = Some(<PayUpdateType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.UpdateTime = r.read_varint_i64()?,
            15 => {
                self.ShopItemType = Some(
                    <PayShopItemType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            17 => self.TagBeginTime = r.read_varint_i64()?,
            18 => self.TagEndTime = r.read_varint_i64()?,
            22 => self.CanBuyGoods = r.read_bool()?,
            23 => self.IsRemind = r.read_bool()?,
            24 => self.BuyLimitConditionId = r.read_varint_i32()?,
            25 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Coupons, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            26 => self.LastUpdateTime = r.read_varint_i64()?,
            27 => self.StageImage = r.read_len_delimited()?.to_vec(),
            28 => self.ShowStageImage = r.read_len_delimited()?.to_vec(),
            29 => self.TabId = r.read_varint_i32()?,
            30 => self.ShopId = r.read_varint_i32()?,
            31 => self.Tag = r.read_varint_i32()?,
            32 => self.Sort = r.read_varint_i32()?,
            33 => self.PromotionShow = r.read_varint_i32()?,
            34 => self.SoldOut = r.read_bool()?,
            35 => self.ActivityId = r.read_varint_i32()?,
            36 => self.Show = r.read_bool()?,
            37 => self.ComplianceDetail = r.read_len_delimited()?.to_vec(),
            38 => self.Quality = r.read_varint_i32()?,
            39 => self.DiscountSort = r.read_varint_i32()?,
            40 => self.OnceBuyLimit = r.read_varint_i32()?,
            41 => self.IsRecommend = r.read_bool()?,
            42 => self.IsShowHaveNum = r.read_bool()?,
            43 => self.IsBuyMaxButton = r.read_bool()?,
            44 => self.ConfirmLimitCount = r.read_varint_i32()?,
            45 => self.ShowBeginTime = r.read_varint_i64()?,
            46 => self.ShowEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopInfo {
    pub Id: i32,
    pub Items: Vec<PayShopItem>,
    pub UpdateTime: i64,
    pub LastUpdateTime: i64,
    pub ShopTabViewType: i32,
    pub DynamicTabId: i32,
    pub Sort: i32,
    pub Money: Vec<i32>,
    pub SortRule: i32,
    pub PayShopTabTogContent: Vec<u8>,
    pub EnableServerRedDot: bool,
}

impl PbMessage for PayShopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.UpdateTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpdateTime);
        }
        if self.LastUpdateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastUpdateTime);
        }
        if self.ShopTabViewType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopTabViewType);
        }
        if self.DynamicTabId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DynamicTabId);
        }
        if self.Sort != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sort);
        }
        for item in &self.Money {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SortRule != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SortRule);
        }
        if !self.PayShopTabTogContent.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PayShopTabTogContent);
        }
        if self.EnableServerRedDot {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Items.push(<PayShopItem as PbMessage>::decode(bytes)?);
            }
            3 => self.UpdateTime = r.read_varint_i64()?,
            4 => self.LastUpdateTime = r.read_varint_i64()?,
            5 => self.ShopTabViewType = r.read_varint_i32()?,
            6 => self.DynamicTabId = r.read_varint_i32()?,
            7 => self.Sort = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Money, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => self.SortRule = r.read_varint_i32()?,
            10 => self.PayShopTabTogContent = r.read_len_delimited()?.to_vec(),
            11 => self.EnableServerRedDot = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftShopInfo {
    pub Gifts: Vec<PayGiftInfo>,
    pub Version: Vec<u8>,
}

impl PbMessage for PayGiftShopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Gifts {
            crate::wire::write_submessage(out, 1, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Gifts.push(<PayGiftInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopInfoRequest {
    pub Version: Vec<u8>,
}

impl PayShopInfoRequest {
    pub const MSG_ID: u16 = 21126;
}

impl PbMessage for PayShopInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopInfoResponse {
    pub Infos: Vec<PayShopInfo>,
    pub Version: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
    pub PayGiftShopInfo: Option<PayGiftShopInfo>,
    pub PayShopTabData: Vec<ShopTab>,
    pub PayShopRecommendData: Vec<ShopRecommend>,
}

impl PayShopInfoResponse {
    pub const MSG_ID: u16 = 28666;
}

impl PbMessage for PayShopInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 2, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PayGiftShopInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        for item in &self.PayShopTabData {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PayShopRecommendData {
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
                self.Infos.push(<PayShopInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.Version = r.read_len_delimited()?.to_vec(),
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PayGiftShopInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PayShopTabData
                    .push(<ShopTab as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PayShopRecommendData
                    .push(<ShopRecommend as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopTab {
    pub ShopId: i32,
    pub TabId: i32,
    pub Sort: i32,
    pub name: Vec<u8>,
    pub Logic: i32,
    pub Enable: bool,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub TabSelectSpritePath: Vec<u8>,
    pub TabContentPath: Vec<u8>,
    pub Money: Vec<i32>,
}

impl PbMessage for ShopTab {
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
        if !self.name.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.name);
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
            4 => self.name = r.read_len_delimited()?.to_vec(),
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
pub struct ShopRecommend {
    pub Id: i32,
    pub RecommendType: i32,
    pub RecommendId: i32,
    pub TabName: Vec<u8>,
    pub PrefabPath: Vec<u8>,
    pub Sort: i32,
    pub Show: bool,
    pub TabImage: Vec<u8>,
}

impl PbMessage for ShopRecommend {
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
pub struct PhBaPlanAttr {
    pub AttrId: i32,
    pub Deal: Option<PhBaPlanAttrDeal>,
}

impl PbMessage for PhBaPlanAttr {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrId);
        }
        if let Some(v) = &self.Deal {
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
            1 => self.AttrId = r.read_varint_i32()?,
            2 => {
                self.Deal = Some(
                    <PhBaPlanAttrDeal as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhBaOneCostPlan {
    pub CostType: Option<PhBaCostType>,
    pub AttrList: Vec<PhBaPlanAttr>,
}

impl PbMessage for PhBaOneCostPlan {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CostType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AttrList {
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
                self.CostType = Some(<PhBaCostType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AttrList
                    .push(<PhBaPlanAttr as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaOneSuitPlan {
    pub SuitId: i32,
    pub OneCostList: Vec<PhBaOneCostPlan>,
    pub IsOpen: bool,
}

impl PbMessage for PhBaOneSuitPlan {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SuitId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SuitId);
        }
        for item in &self.OneCostList {
            crate::wire::write_submessage(out, 2, item);
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
            1 => self.SuitId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.OneCostList
                    .push(<PhBaOneCostPlan as PbMessage>::decode(bytes)?);
            }
            3 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaOneAllSuitPlan {
    pub SuitPlanList: Vec<PhBaOneSuitPlan>,
    pub Name: Vec<u8>,
}

impl PbMessage for PhBaOneAllSuitPlan {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SuitPlanList {
            crate::wire::write_submessage(out, 1, item);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.SuitPlanList
                    .push(<PhBaOneSuitPlan as PbMessage>::decode(bytes)?);
            }
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanUsePlanRequest {}

impl PhBaPlanUsePlanRequest {
    pub const MSG_ID: u16 = 27699;
}

impl PbMessage for PhBaPlanUsePlanRequest {
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
pub struct PhBaPlanUsePlanResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfPlanCode: Vec<u8>,
    pub UsePlan: Option<PhBaOneAllSuitPlan>,
    pub FiveStarSwitch: bool,
    pub TowPlanSame: bool,
}

impl PhBaPlanUsePlanResponse {
    pub const MSG_ID: u16 = 17771;
}

impl PbMessage for PhBaPlanUsePlanResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.SelfPlanCode.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SelfPlanCode);
        }
        if let Some(v) = &self.UsePlan {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.FiveStarSwitch {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TowPlanSame {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.SelfPlanCode = r.read_len_delimited()?.to_vec(),
            14 => {
                let bytes = r.read_len_delimited()?;
                self.UsePlan
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.FiveStarSwitch = r.read_bool()?,
            8 => self.TowPlanSame = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSaveUsePlanRequest {
    pub UsePlan: Option<PhBaOneAllSuitPlan>,
}

impl PhBaPlanSaveUsePlanRequest {
    pub const MSG_ID: u16 = 15491;
}

impl PbMessage for PhBaPlanSaveUsePlanRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UsePlan {
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
                self.UsePlan
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSaveUsePlanResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfPlanCode: Vec<u8>,
    pub TowPlanSame: bool,
}

impl PhBaPlanSaveUsePlanResponse {
    pub const MSG_ID: u16 = 26457;
}

impl PbMessage for PhBaPlanSaveUsePlanResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.SelfPlanCode.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SelfPlanCode);
        }
        if self.TowPlanSame {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.SelfPlanCode = r.read_len_delimited()?.to_vec(),
            1 => self.TowPlanSame = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanFindPlanRequest {
    pub TargetCode: Vec<u8>,
}

impl PhBaPlanFindPlanRequest {
    pub const MSG_ID: u16 = 29810;
}

impl PbMessage for PhBaPlanFindPlanRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TargetCode.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TargetCode);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.TargetCode = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanFindPlanResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Plan: Option<PhBaOneAllSuitPlan>,
    pub TowPlanSame: bool,
}

impl PhBaPlanFindPlanResponse {
    pub const MSG_ID: u16 = 26108;
}

impl PbMessage for PhBaPlanFindPlanResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Plan {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.TowPlanSame {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Plan
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.TowPlanSame = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSetStatusInfo {
    pub Open: bool,
    pub Suit: i32,
}

impl PbMessage for PhBaPlanSetStatusInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Open {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Suit != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Suit);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Open = r.read_bool()?,
            2 => self.Suit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSetPlanStatusRequest {
    pub SetInfo: Vec<PhBaPlanSetStatusInfo>,
}

impl PhBaPlanSetPlanStatusRequest {
    pub const MSG_ID: u16 = 24251;
}

impl PbMessage for PhBaPlanSetPlanStatusRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SetInfo {
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
                self.SetInfo
                    .push(<PhBaPlanSetStatusInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSetPlanStatusResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TowPlanSame: bool,
}

impl PhBaPlanSetPlanStatusResponse {
    pub const MSG_ID: u16 = 26043;
}

impl PbMessage for PhBaPlanSetPlanStatusResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TowPlanSame {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.TowPlanSame = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanUpdatePlanRequest {}

impl PhBaPlanUpdatePlanRequest {
    pub const MSG_ID: u16 = 23537;
}

impl PbMessage for PhBaPlanUpdatePlanRequest {
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
pub struct PhBaPlanUpdatePlanResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfPlanCode: Vec<u8>,
}

impl PhBaPlanUpdatePlanResponse {
    pub const MSG_ID: u16 = 20521;
}

impl PbMessage for PhBaPlanUpdatePlanResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.SelfPlanCode.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SelfPlanCode);
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
            7 => self.SelfPlanCode = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSetFiveStarSwitchRequest {
    pub Open: bool,
}

impl PhBaPlanSetFiveStarSwitchRequest {
    pub const MSG_ID: u16 = 17981;
}

impl PbMessage for PhBaPlanSetFiveStarSwitchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Open {
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
            7 => self.Open = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanSetFiveStarSwitchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhBaPlanSetFiveStarSwitchResponse {
    pub const MSG_ID: u16 = 24842;
}

impl PbMessage for PhBaPlanSetFiveStarSwitchResponse {
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
pub struct PhBaPlanBatchOper {
    pub Oper: Option<PhantomFuncValue>,
    pub IncrId: Vec<i32>,
}

impl PbMessage for PhBaPlanBatchOper {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Oper {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.IncrId {
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
                self.Oper = Some(
                    <PhantomFuncValue as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncrId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanBatchOperRequest {
    pub BatchOper: Vec<PhBaPlanBatchOper>,
}

impl PhBaPlanBatchOperRequest {
    pub const MSG_ID: u16 = 26395;
}

impl PbMessage for PhBaPlanBatchOperRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BatchOper {
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
                self.BatchOper
                    .push(<PhBaPlanBatchOper as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhBaPlanBatchOperResponse {
    pub errCode: Option<ErrorCode>,
}

impl PhBaPlanBatchOperResponse {
    pub const MSG_ID: u16 = 28424;
}

impl PbMessage for PhBaPlanBatchOperResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.errCode {
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
                self.errCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomConsumeItem {
    pub IncId: i32,
    pub Count: i32,
    pub ItemId: i32,
}

impl PbMessage for PhantomConsumeItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
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
            2 => self.Count = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomStackConsumeEntry {
    pub IncrId: i32,
    pub Count: i32,
}

impl PbMessage for PhantomStackConsumeEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
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
            1 => self.IncrId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomStackSplitInfo {
    pub OperateItem: Option<PhantomItem>,
    pub OperateBeforeCount: i32,
    pub RemainderItem: Option<PhantomItem>,
}

impl PbMessage for PhantomStackSplitInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OperateItem {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.OperateBeforeCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OperateBeforeCount);
        }
        if let Some(v) = &self.RemainderItem {
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
                self.OperateItem
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.OperateBeforeCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RemainderItem
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomStackMergeInfo {
    pub MergedIncrId: i32,
    pub RepresentativeItem: Option<PhantomItem>,
}

impl PbMessage for PhantomStackMergeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MergedIncrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MergedIncrId);
        }
        if let Some(v) = &self.RepresentativeItem {
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
            1 => self.MergedIncrId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RepresentativeItem
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomLevelUpRequest {
    pub IncId: i32,
    pub ConsumeList: Vec<PhantomConsumeItem>,
    pub SlotCount: i32,
}

impl PhantomLevelUpRequest {
    pub const MSG_ID: u16 = 25666;
}

impl PbMessage for PhantomLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        for item in &self.ConsumeList {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.SlotCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotCount);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ConsumeList
                    .push(<PhantomConsumeItem as PbMessage>::decode(bytes)?);
            }
            2 => self.SlotCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UpdateInfo: Option<PhantomItem>,
    pub ItemMap: BTreeMap<i32, i32>,
    pub StackSplitInfo: Option<PhantomStackSplitInfo>,
}

impl PhantomLevelUpResponse {
    pub const MSG_ID: u16 = 29647;
}

impl PbMessage for PhantomLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UpdateInfo {
            crate::wire::write_submessage(out, 9, v);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 8, &self.ItemMap);
        }
        if let Some(v) = &self.StackSplitInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPutOnNotify {
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
}

impl PhantomPutOnNotify {
    pub const MSG_ID: u16 = 29430;
}

impl PbMessage for PhantomPutOnNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipInfoList {
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
                self.EquipInfoList
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPutOnRequest {
    pub IncId: i32,
    pub RoleId: i32,
    pub Pos: i32,
}

impl PhantomPutOnRequest {
    pub const MSG_ID: u16 = 25608;
}

impl PbMessage for PhantomPutOnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Pos != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Pos);
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
            13 => self.RoleId = r.read_varint_i32()?,
            7 => self.Pos = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPutOnResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
    pub StackSplitInfos: Vec<PhantomStackSplitInfo>,
    pub StackMergeInfos: Vec<PhantomStackMergeInfo>,
}

impl PhantomPutOnResponse {
    pub const MSG_ID: u16 = 21144;
}

impl PbMessage for PhantomPutOnResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.EquipInfoList {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.StackSplitInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.StackMergeInfos {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.EquipInfoList
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfos
                    .push(<PhantomStackSplitInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .push(<PhantomStackMergeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomAutoPutRequest {
    pub RoleId: i32,
    pub PhantomItemIncrId: Vec<i32>,
}

impl PhantomAutoPutRequest {
    pub const MSG_ID: u16 = 25130;
}

impl PbMessage for PhantomAutoPutRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.PhantomItemIncrId {
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
            10 => self.RoleId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(
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
pub struct PhantomAutoPutResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
    pub StackSplitInfos: Vec<PhantomStackSplitInfo>,
    pub StackMergeInfos: Vec<PhantomStackMergeInfo>,
}

impl PhantomAutoPutResponse {
    pub const MSG_ID: u16 = 21508;
}

impl PbMessage for PhantomAutoPutResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.EquipInfoList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.StackSplitInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.StackMergeInfos {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.EquipInfoList
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfos
                    .push(<PhantomStackSplitInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .push(<PhantomStackMergeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomItemUpdateNotify {
    pub UpdateInfo: Vec<PhantomItem>,
}

impl PhantomItemUpdateNotify {
    pub const MSG_ID: u16 = 16679;
}

impl PbMessage for PhantomItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UpdateInfo {
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
                self.UpdateInfo
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RolePhantomPropUpdateNotify {
    pub PropInfo: Vec<RolePhantomPropInfo>,
}

impl RolePhantomPropUpdateNotify {
    pub const MSG_ID: u16 = 23146;
}

impl PbMessage for RolePhantomPropUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PropInfo {
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
                self.PropInfo
                    .push(<RolePhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomIdentifyRequest {
    pub IncrId: i32,
    pub Count: i32,
}

impl PhantomIdentifyRequest {
    pub const MSG_ID: u16 = 26204;
}

impl PbMessage for PhantomIdentifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            3 => self.IncrId = r.read_varint_i32()?,
            10 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomIdentifyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UpdateInfo: Option<PhantomItem>,
}

impl PhantomIdentifyResponse {
    pub const MSG_ID: u16 = 19102;
}

impl PbMessage for PhantomIdentifyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UpdateInfo {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSkinUnlockNotify {
    pub PhantomSkinList: Vec<i32>,
}

impl PhantomSkinUnlockNotify {
    pub const MSG_ID: u16 = 20095;
}

impl PbMessage for PhantomSkinUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomSkinList {
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
                &mut self.PhantomSkinList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSkinChangeRequest {
    pub IncrId: i32,
    pub SkinId: i32,
    pub ChangeDefault: bool,
}

impl PhantomSkinChangeRequest {
    pub const MSG_ID: u16 = 22400;
}

impl PbMessage for PhantomSkinChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.ChangeDefault {
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
            14 => self.IncrId = r.read_varint_i32()?,
            1 => self.SkinId = r.read_varint_i32()?,
            6 => self.ChangeDefault = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSkinChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomSkinChangeResponse {
    pub const MSG_ID: u16 = 26665;
}

impl PbMessage for PhantomSkinChangeResponse {
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
pub struct PhantomRefiningRequest {
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomRefiningRequest {
    pub const MSG_ID: u16 = 27747;
}

impl PbMessage for PhantomRefiningRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StackEntries {
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
                self.StackEntries
                    .push(<PhantomStackConsumeEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomRefiningResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Items: Vec<AddCountItemInfo>,
    pub ExtraItems: Vec<AddCountItemInfo>,
    pub CostPhantoms: Vec<PhantomItem>,
}

impl PhantomRefiningResponse {
    pub const MSG_ID: u16 = 27651;
}

impl PbMessage for PhantomRefiningResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.ExtraItems {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.CostPhantoms {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItems
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.CostPhantoms
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashBatchRefiningRequest {
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl CalabashBatchRefiningRequest {
    pub const MSG_ID: u16 = 27203;
}

impl PbMessage for CalabashBatchRefiningRequest {
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
pub struct CalabashBatchRefiningResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Items: Vec<AddCountItemInfo>,
    pub ExtraItems: Vec<AddCountItemInfo>,
    pub CostPhantoms: Vec<PhantomItem>,
}

impl CalabashBatchRefiningResponse {
    pub const MSG_ID: u16 = 18643;
}

impl PbMessage for CalabashBatchRefiningResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.ExtraItems {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.CostPhantoms {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItems
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CostPhantoms
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBatchDirectRefiningRequest {
    pub TargetFetterGroupId: i32,
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomBatchDirectRefiningRequest {
    pub const MSG_ID: u16 = 29456;
}

impl PbMessage for PhantomBatchDirectRefiningRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFetterGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFetterGroupId);
        }
        for item in &self.StackEntries {
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
            1 => self.TargetFetterGroupId = r.read_varint_i32()?,
            2 => {
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
pub struct PhantomBatchDirectRefiningResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Items: Vec<AddCountItemInfo>,
    pub CostPhantoms: Vec<PhantomItem>,
    pub DirectRefineWeekTimes: i32,
}

impl PhantomBatchDirectRefiningResponse {
    pub const MSG_ID: u16 = 28755;
}

impl PbMessage for PhantomBatchDirectRefiningResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.CostPhantoms {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.DirectRefineWeekTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CostPhantoms
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            2 => self.DirectRefineWeekTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPolishRequest {
    pub IncrId: i32,
    pub PhantomMainPropItemId: i32,
}

impl PhantomPolishRequest {
    pub const MSG_ID: u16 = 25622;
}

impl PbMessage for PhantomPolishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.PhantomMainPropItemId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomMainPropItemId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.IncrId = r.read_varint_i32()?,
            10 => self.PhantomMainPropItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomPolishResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UpdateInfo: Option<PhantomItem>,
    pub StackSplitInfo: Option<PhantomStackSplitInfo>,
    pub StackMergeInfos: Option<PhantomStackMergeInfo>,
}

impl PhantomPolishResponse {
    pub const MSG_ID: u16 = 25461;
}

impl PbMessage for PhantomPolishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UpdateInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.StackSplitInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.StackMergeInfos {
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomManageConfigUpdateRequest {
    pub Setting: Option<PhantomManageConfig>,
    pub SettingType: Option<PhantomSettingType>,
}

impl PhantomManageConfigUpdateRequest {
    pub const MSG_ID: u16 = 27251;
}

impl PbMessage for PhantomManageConfigUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Setting {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.SettingType {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Setting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1 => {
                self.SettingType = Some(
                    <PhantomSettingType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomManageConfigUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Setting: Option<PhantomManageConfig>,
}

impl PhantomManageConfigUpdateResponse {
    pub const MSG_ID: u16 = 25467;
}

impl PbMessage for PhantomManageConfigUpdateResponse {
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
pub struct PhantomSettingInfo {
    pub Setting: Option<PhantomManageConfig>,
    pub SettingType: Option<PhantomSettingType>,
}

impl PbMessage for PhantomSettingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Setting {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.SettingType {
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
                self.Setting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                self.SettingType = Some(
                    <PhantomSettingType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomSettingBatchUpdateRequest {
    pub Settings: Vec<PhantomSettingInfo>,
}

impl PhantomSettingBatchUpdateRequest {
    pub const MSG_ID: u16 = 21039;
}

impl PbMessage for PhantomSettingBatchUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Settings {
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
                self.Settings
                    .push(<PhantomSettingInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSettingBatchUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Settings: Vec<PhantomSettingInfo>,
}

impl PhantomSettingBatchUpdateResponse {
    pub const MSG_ID: u16 = 28959;
}

impl PbMessage for PhantomSettingBatchUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Settings {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Settings
                    .push(<PhantomSettingInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomManageConfigRequest {}

impl PhantomManageConfigRequest {
    pub const MSG_ID: u16 = 24025;
}

impl PbMessage for PhantomManageConfigRequest {
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
pub struct PhantomManageConfigResponse {
    pub AutoLock: Vec<PhantomManageConfig>,
    pub AutoDisuse: Vec<PhantomManageConfig>,
}

impl PhantomManageConfigResponse {
    pub const MSG_ID: u16 = 29760;
}

impl PbMessage for PhantomManageConfigResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AutoLock {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.AutoDisuse {
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.AutoLock
                    .push(<PhantomManageConfig as PbMessage>::decode(bytes)?);
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.AutoDisuse
                    .push(<PhantomManageConfig as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomManageConfig {
    pub Index: i32,
    pub IsOn: bool,
    pub Name: Vec<u8>,
    pub PhantomRule: Vec<PhantomRuleMap>,
}

impl PbMessage for PhantomManageConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.IsOn {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        for item in &self.PhantomRule {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => self.IsOn = r.read_bool()?,
            3 => self.Name = r.read_len_delimited()?.to_vec(),
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomRule
                    .push(<PhantomRuleMap as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomRuleMap {
    pub RuleId: i32,
    pub ValueList: Vec<i32>,
}

impl PbMessage for PhantomRuleMap {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RuleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RuleId);
        }
        for item in &self.ValueList {
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
            1 => self.RuleId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ValueList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomFuncValueBatchRequest {
    pub FuncValue: Option<PhantomFuncValue>,
    pub IncrId: Vec<i32>,
}

impl PhantomFuncValueBatchRequest {
    pub const MSG_ID: u16 = 15779;
}

impl PbMessage for PhantomFuncValueBatchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FuncValue {
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
                self.FuncValue = Some(
                    <PhantomFuncValue as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomFuncValueBatchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomFuncValueBatchResponse {
    pub const MSG_ID: u16 = 25271;
}

impl PbMessage for PhantomFuncValueBatchResponse {
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
pub struct PhantomBatchPolishRequest {
    pub PhantomMainPropItemId: i32,
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomBatchPolishRequest {
    pub const MSG_ID: u16 = 17121;
}

impl PbMessage for PhantomBatchPolishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomMainPropItemId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomMainPropItemId);
        }
        for item in &self.StackEntries {
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
            6 => self.PhantomMainPropItemId = r.read_varint_i32()?,
            14 => {
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
pub struct PhantomBatchPolishResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UpdateInfos: Vec<PhantomItem>,
    pub StackSplitInfos: Vec<PhantomStackSplitInfo>,
    pub StackMergeInfos: Vec<PhantomStackMergeInfo>,
}

impl PhantomBatchPolishResponse {
    pub const MSG_ID: u16 = 21024;
}

impl PbMessage for PhantomBatchPolishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.UpdateInfos {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.StackSplitInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.StackMergeInfos {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateInfos
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfos
                    .push(<PhantomStackSplitInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .push(<PhantomStackMergeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomVicePolishRequest {
    pub IncrId: i32,
    pub LockPropIndex: Vec<i32>,
}

impl PhantomVicePolishRequest {
    pub const MSG_ID: u16 = 21223;
}

impl PbMessage for PhantomVicePolishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        for item in &self.LockPropIndex {
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
            7 => self.IncrId = r.read_varint_i32()?,
            8 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockPropIndex, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomVicePolishResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomSubProp: Vec<PhantomPropInfo>,
}

impl PhantomVicePolishResponse {
    pub const MSG_ID: u16 = 25546;
}

impl PbMessage for PhantomVicePolishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomSubProp {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomSubProp
                    .push(<PhantomPropInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomVicePolishAckRequest {
    pub IncrId: i32,
    pub Ack: bool,
}

impl PhantomVicePolishAckRequest {
    pub const MSG_ID: u16 = 26021;
}

impl PbMessage for PhantomVicePolishAckRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.Ack {
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
            4 => self.IncrId = r.read_varint_i32()?,
            15 => self.Ack = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomVicePolishAckResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UpdateInfo: Option<PhantomItem>,
}

impl PhantomVicePolishAckResponse {
    pub const MSG_ID: u16 = 22940;
}

impl PbMessage for PhantomVicePolishAckResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UpdateInfo {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RefreshVisionEquipGroupData {
    pub IncId: Vec<i32>,
    pub Name: Vec<u8>,
}

impl PbMessage for RefreshVisionEquipGroupData {
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
pub struct VisionEquipGroupInfoRequest {
    pub RoleId: i32,
}

impl VisionEquipGroupInfoRequest {
    pub const MSG_ID: u16 = 25068;
}

impl PbMessage for VisionEquipGroupInfoRequest {
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
pub struct VisionEquipGroupInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionEquipList: Vec<RefreshVisionEquipGroupData>,
    pub IsOldData: bool,
}

impl VisionEquipGroupInfoResponse {
    pub const MSG_ID: u16 = 17071;
}

impl PbMessage for VisionEquipGroupInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionEquipList {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.IsOldData {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.VisionEquipList
                    .push(<RefreshVisionEquipGroupData as PbMessage>::decode(bytes)?);
            }
            6 => self.IsOldData = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddVisionEquipGroupRequest {
    pub RoleId: i32,
    pub Name: Vec<u8>,
}

impl AddVisionEquipGroupRequest {
    pub const MSG_ID: u16 = 18869;
}

impl PbMessage for AddVisionEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
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
            15 => self.RoleId = r.read_varint_i32()?,
            7 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddVisionEquipGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionEquipList: Vec<RefreshVisionEquipGroupData>,
    pub IsOldData: bool,
}

impl AddVisionEquipGroupResponse {
    pub const MSG_ID: u16 = 17340;
}

impl PbMessage for AddVisionEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionEquipList {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsOldData {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.VisionEquipList
                    .push(<RefreshVisionEquipGroupData as PbMessage>::decode(bytes)?);
            }
            6 => self.IsOldData = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeleteVisionEquipGroupRequest {
    pub Index: i32,
    pub RoleId: i32,
}

impl DeleteVisionEquipGroupRequest {
    pub const MSG_ID: u16 = 28863;
}

impl PbMessage for DeleteVisionEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            7 => self.Index = r.read_varint_i32()?,
            6 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeleteVisionEquipGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionEquipList: Vec<RefreshVisionEquipGroupData>,
}

impl DeleteVisionEquipGroupResponse {
    pub const MSG_ID: u16 = 16721;
}

impl PbMessage for DeleteVisionEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionEquipList {
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.VisionEquipList
                    .push(<RefreshVisionEquipGroupData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PutVisionGroupToTopRequest {
    pub Index: i32,
    pub RoleId: i32,
}

impl PutVisionGroupToTopRequest {
    pub const MSG_ID: u16 = 19922;
}

impl PbMessage for PutVisionGroupToTopRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
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
            4 => self.Index = r.read_varint_i32()?,
            1 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PutVisionGroupToTopResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionEquipList: Vec<RefreshVisionEquipGroupData>,
}

impl PutVisionGroupToTopResponse {
    pub const MSG_ID: u16 = 24824;
}

impl PbMessage for PutVisionGroupToTopResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionEquipList {
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
                self.VisionEquipList
                    .push(<RefreshVisionEquipGroupData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeVisionGroupNameRequest {
    pub Index: i32,
    pub Name: Vec<u8>,
    pub RoleId: i32,
}

impl ChangeVisionGroupNameRequest {
    pub const MSG_ID: u16 = 18907;
}

impl PbMessage for ChangeVisionGroupNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            9 => self.Index = r.read_varint_i32()?,
            13 => self.Name = r.read_len_delimited()?.to_vec(),
            6 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeVisionGroupNameResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionEquipList: Vec<RefreshVisionEquipGroupData>,
}

impl ChangeVisionGroupNameResponse {
    pub const MSG_ID: u16 = 16060;
}

impl PbMessage for ChangeVisionGroupNameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionEquipList {
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
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.VisionEquipList
                    .push(<RefreshVisionEquipGroupData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyVisionGroupRequest {
    pub Index: i32,
    pub RoleId: i32,
}

impl ApplyVisionGroupRequest {
    pub const MSG_ID: u16 = 23641;
}

impl PbMessage for ApplyVisionGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
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
            12 => self.Index = r.read_varint_i32()?,
            7 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyVisionGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
    pub StackSplitInfos: Vec<PhantomStackSplitInfo>,
    pub StackMergeInfos: Vec<PhantomStackMergeInfo>,
}

impl ApplyVisionGroupResponse {
    pub const MSG_ID: u16 = 19113;
}

impl PbMessage for ApplyVisionGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.EquipInfoList {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.StackSplitInfos {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.StackMergeInfos {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .push(<PhantomStackMergeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetMusicInfoRequest {}

impl GetMusicInfoRequest {
    pub const MSG_ID: u16 = 16180;
}

impl PbMessage for GetMusicInfoRequest {
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
pub struct GetMusicInfoResponse {
    pub CurMusicId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub AlbumTimeInfos: Vec<PhonographAlbumTimeInfo>,
    pub CollectMusicIds: Vec<i32>,
}

impl GetMusicInfoResponse {
    pub const MSG_ID: u16 = 16334;
}

impl PbMessage for GetMusicInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurMusicId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurMusicId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AlbumTimeInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.CollectMusicIds {
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
            13 => self.CurMusicId = r.read_varint_i32()?,
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AlbumTimeInfos
                    .push(<PhonographAlbumTimeInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
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
pub struct FavoriteMusicMotorCycleUpdateRequest {
    pub FavoriteMusicList: Vec<i32>,
}

impl FavoriteMusicMotorCycleUpdateRequest {
    pub const MSG_ID: u16 = 18730;
}

impl PbMessage for FavoriteMusicMotorCycleUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FavoriteMusicList {
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
                &mut self.FavoriteMusicList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FavoriteMusicMotorCycleUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FavoriteMusicMotorCycleUpdateResponse {
    pub const MSG_ID: u16 = 18049;
}

impl PbMessage for FavoriteMusicMotorCycleUpdateResponse {
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
pub struct PhonographAlbumTimeInfo {
    pub AlbumId: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub MusicIds: Vec<i32>,
}

impl PbMessage for PhonographAlbumTimeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AlbumId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AlbumId);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.MusicIds {
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
            1 => self.AlbumId = r.read_varint_i32()?,
            2 => self.BeginTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MusicIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FragmentMemoryData {
    pub Id: i32,
    pub Flag: i32,
    pub FinishTime: i64,
}

impl PbMessage for FragmentMemoryData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Flag != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Flag);
        }
        if self.FinishTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Flag = r.read_varint_i32()?,
            3 => self.FinishTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FragmentMemoryItem {
    pub Id: i32,
    pub Data: Vec<FragmentMemoryData>,
    pub IsUnlock: bool,
}

impl PbMessage for FragmentMemoryItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Data {
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
                self.Data
                    .push(<FragmentMemoryData as PbMessage>::decode(bytes)?);
            }
            3 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoMemoryRequest {}

impl PhotoMemoryRequest {
    pub const MSG_ID: u16 = 25225;
}

impl PbMessage for PhotoMemoryRequest {
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
pub struct PhotoMemoryResponse {
    pub Item: Vec<FragmentMemoryItem>,
}

impl PhotoMemoryResponse {
    pub const MSG_ID: u16 = 22761;
}

impl PbMessage for PhotoMemoryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Item {
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
                self.Item
                    .push(<FragmentMemoryItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionFetterRecommendInfo {
    pub Usage: i32,
    pub RecommendFetterGroupInfos: Vec<RecommendFetterGroupInfo>,
    pub CostCombinationInfos: Vec<CostCombinationInfo>,
    pub MainPhantomRecommendInfos: Vec<MainPhantomRecommendInfo>,
}

impl PbMessage for VisionFetterRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Usage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Usage);
        }
        for item in &self.RecommendFetterGroupInfos {
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
                self.RecommendFetterGroupInfos
                    .push(<RecommendFetterGroupInfo as PbMessage>::decode(bytes)?);
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
pub struct RecommendFetterGroupInfo {
    pub RecommendFetterGroupId: i32,
    pub CountNeed: i32,
}

impl PbMessage for RecommendFetterGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecommendFetterGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommendFetterGroupId);
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
            1 => self.RecommendFetterGroupId = r.read_varint_i32()?,
            2 => self.CountNeed = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostCombinationInfo {
    pub CostId: i32,
    pub Usage: i32,
}

impl PbMessage for CostCombinationInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CostId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostId);
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
            1 => self.CostId = r.read_varint_i32()?,
            2 => self.Usage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionAttrRecommendInfo {
    pub AttrType: i32,
    pub AddType: i32,
    pub Usage: i32,
}

impl PbMessage for VisionAttrRecommendInfo {
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
pub struct CostVisionAttrRecommendInfo {
    pub Cost: i32,
    pub GetMainAttrRecommendInfo: Vec<VisionAttrRecommendInfo>,
    pub GetSubAttrRecommendInfo: Vec<VisionAttrRecommendInfo>,
}

impl PbMessage for CostVisionAttrRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Cost != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cost);
        }
        for item in &self.GetMainAttrRecommendInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.GetSubAttrRecommendInfo {
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
            1 => self.Cost = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GetMainAttrRecommendInfo
                    .push(<VisionAttrRecommendInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.GetSubAttrRecommendInfo
                    .push(<VisionAttrRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleVisionRecommendDataRequest {
    pub RoleId: i32,
}

impl RoleVisionRecommendDataRequest {
    pub const MSG_ID: u16 = 29643;
}

impl PbMessage for RoleVisionRecommendDataRequest {
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
pub struct RoleVisionRecommendDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionFetterRecommendInfo: Vec<VisionFetterRecommendInfo>,
}

impl RoleVisionRecommendDataResponse {
    pub const MSG_ID: u16 = 25672;
}

impl PbMessage for RoleVisionRecommendDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionFetterRecommendInfo {
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
                self.VisionFetterRecommendInfo
                    .push(<VisionFetterRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleVisionRecommendAttrRequest {
    pub RoleId: i32,
}

impl RoleVisionRecommendAttrRequest {
    pub const MSG_ID: u16 = 18178;
}

impl PbMessage for RoleVisionRecommendAttrRequest {
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
pub struct RoleVisionRecommendAttrResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VisionAttrRecommendInfos: Vec<CostVisionAttrRecommendInfo>,
}

impl RoleVisionRecommendAttrResponse {
    pub const MSG_ID: u16 = 29590;
}

impl PbMessage for RoleVisionRecommendAttrResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.VisionAttrRecommendInfos {
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.VisionAttrRecommendInfos
                    .push(<CostVisionAttrRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MainPhantomRecommendInfo {
    pub Usage: i32,
    pub MonsterId: i32,
    pub FetterGroupId: i32,
}

impl PbMessage for MainPhantomRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Usage != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Usage);
        }
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        if self.FetterGroupId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FetterGroupId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Usage = r.read_varint_i32()?,
            2 => self.MonsterId = r.read_varint_i32()?,
            3 => self.FetterGroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleVisionMainPhantomRequest {
    pub RoleId: i32,
}

impl RoleVisionMainPhantomRequest {
    pub const MSG_ID: u16 = 15154;
}

impl PbMessage for RoleVisionMainPhantomRequest {
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
pub struct RoleVisionMainPhantomResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RecommendInfo: Vec<MainPhantomRecommendInfo>,
}

impl RoleVisionMainPhantomResponse {
    pub const MSG_ID: u16 = 26732;
}

impl PbMessage for RoleVisionMainPhantomResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RecommendInfo {
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
                self.RecommendInfo
                    .push(<MainPhantomRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillInfo {
    pub SkillId: i64,
    pub SkillCdEndTime: i64,
}

impl PbMessage for PassiveSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.SkillCdEndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillCdEndTime);
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
            2 => self.SkillCdEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RolePassiveSkillInfo {
    pub RoleId: i32,
    pub PassiveSkillInfoList: Vec<PassiveSkillInfo>,
}

impl PbMessage for RolePassiveSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.PassiveSkillInfoList {
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
                self.PassiveSkillInfoList
                    .push(<PassiveSkillInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillNotify {
    pub RolePassiveSkillInfoList: Vec<RolePassiveSkillInfo>,
}

impl PassiveSkillNotify {
    pub const MSG_ID: u16 = 17786;
}

impl PbMessage for PassiveSkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RolePassiveSkillInfoList {
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
                self.RolePassiveSkillInfoList
                    .push(<RolePassiveSkillInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTitleData {
    pub PlayerTitleId: i32,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub StarLevel: i32,
    pub ActivityServerRewardItemData: Option<ConditionTask>,
}

impl PbMessage for PlayerTitleData {
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
        if self.StarLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StarLevel);
        }
        if let Some(v) = &self.ActivityServerRewardItemData {
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
            4 => self.StarLevel = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityServerRewardItemData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DressPlayerTitleUpdateNotify {
    pub PlayerTitleId: i32,
    pub PlayerTitleExtraParam: i32,
}

impl DressPlayerTitleUpdateNotify {
    pub const MSG_ID: u16 = 19561;
}

impl PbMessage for DressPlayerTitleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
        }
        if self.PlayerTitleExtraParam != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleExtraParam);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.PlayerTitleId = r.read_varint_i32()?,
            11 => self.PlayerTitleExtraParam = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTitleDataRequest {}

impl PlayerTitleDataRequest {
    pub const MSG_ID: u16 = 25420;
}

impl PbMessage for PlayerTitleDataRequest {
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
pub struct PlayerTitleDataResponse {
    pub PlayerTitleData: Vec<PlayerTitleData>,
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerTitleLimitInfos: Vec<PlayerTitleLimitInfo>,
}

impl PlayerTitleDataResponse {
    pub const MSG_ID: u16 = 23404;
}

impl PbMessage for PlayerTitleDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerTitleData {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PlayerTitleLimitInfos {
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
                self.PlayerTitleData
                    .push(<PlayerTitleData as PbMessage>::decode(bytes)?);
            }
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => {
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
pub struct PlayerTitleLimitInfo {
    pub PlayerTitleId: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for PlayerTitleLimitInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
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
            1 => self.PlayerTitleId = r.read_varint_i32()?,
            2 => self.BeginTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangePlayerTitleRequest {
    pub PlayerTitleId: i32,
}

impl ChangePlayerTitleRequest {
    pub const MSG_ID: u16 = 28211;
}

impl PbMessage for ChangePlayerTitleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.PlayerTitleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangePlayerTitleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangePlayerTitleResponse {
    pub const MSG_ID: u16 = 23826;
}

impl PbMessage for ChangePlayerTitleResponse {
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
pub struct PlayerVarNotify {
    pub VarInfos: BTreeMap<Vec<u8>, VarDefinePb>,
}

impl PlayerVarNotify {
    pub const MSG_ID: u16 = 27186;
}

impl PbMessage for PlayerVarNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.VarInfos.is_empty() {
            crate::wire::write_map(out, 2, &self.VarInfos);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_map_entry(r, &mut self.VarInfos)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerMotionRequest {
    pub Motion: Option<MotionType>,
}

impl PlayerMotionRequest {
    pub const MSG_ID: u16 = 17665;
}

impl PbMessage for PlayerMotionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Motion {
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
                self.Motion = Some(<MotionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerMotionResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl PlayerMotionResponse {
    pub const MSG_ID: u16 = 29174;
}

impl PbMessage for PlayerMotionResponse {
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
pub struct JSPatchNotify {
    pub Content: Vec<u8>,
}

impl JSPatchNotify {
    pub const MSG_ID: u16 = 19879;
}

impl PbMessage for JSPatchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
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
            5 => self.Content = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestInfo {
    pub QuestId: i32,
    pub Status: Option<QuestState>,
}

impl PbMessage for QuestInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
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
            1 => self.QuestId = r.read_varint_i32()?,
            2 => {
                self.Status = Some(<QuestState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestListNotify {
    pub Quests: Vec<QuestInfo>,
}

impl QuestListNotify {
    pub const MSG_ID: u16 = 18320;
}

impl PbMessage for QuestListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Quests {
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
                self.Quests.push(<QuestInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceQuestNotify {
    pub QuestId: i32,
}

impl TraceQuestNotify {
    pub const MSG_ID: u16 = 25995;
}

impl PbMessage for TraceQuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceQuestRequest {
    pub TraceType: i32,
    pub QuestId: i32,
    pub Operate: i32,
}

impl TraceQuestRequest {
    pub const MSG_ID: u16 = 26774;
}

impl PbMessage for TraceQuestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TraceType != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceType);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
        if self.Operate != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            13 => self.TraceType = r.read_varint_i32()?,
            10 => self.QuestId = r.read_varint_i32()?,
            5 => self.Operate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceQuestResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl TraceQuestResponse {
    pub const MSG_ID: u16 = 15680;
}

impl PbMessage for TraceQuestResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct QuestReadyListNotify {
    pub QuestId: Vec<i32>,
}

impl QuestReadyListNotify {
    pub const MSG_ID: u16 = 29359;
}

impl PbMessage for QuestReadyListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestShowListNotify {
    pub QuestId: Vec<i32>,
}

impl QuestShowListNotify {
    pub const MSG_ID: u16 = 24184;
}

impl PbMessage for QuestShowListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFinishListNotify {
    pub QuestId: Vec<i32>,
}

impl QuestFinishListNotify {
    pub const MSG_ID: u16 = 17785;
}

impl PbMessage for QuestFinishListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestRedDotRequest {
    pub QuestId: i32,
    pub Operate: i32,
}

impl QuestRedDotRequest {
    pub const MSG_ID: u16 = 28280;
}

impl PbMessage for QuestRedDotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
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
            14 => self.QuestId = r.read_varint_i32()?,
            11 => self.Operate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestRedDotResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl QuestRedDotResponse {
    pub const MSG_ID: u16 = 29563;
}

impl PbMessage for QuestRedDotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct QuestRedDotNotify {
    pub QuestId: Vec<i32>,
}

impl QuestRedDotNotify {
    pub const MSG_ID: u16 = 19285;
}

impl PbMessage for QuestRedDotNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfirmQuestResourceRequest {
    pub QuestIds: Vec<i32>,
}

impl ConfirmQuestResourceRequest {
    pub const MSG_ID: u16 = 23000;
}

impl PbMessage for ConfirmQuestResourceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfirmQuestResourceResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl ConfirmQuestResourceResponse {
    pub const MSG_ID: u16 = 24175;
}

impl PbMessage for ConfirmQuestResourceResponse {
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
pub struct SetQuestFocusModeRequest {
    pub QuestId: i32,
}

impl SetQuestFocusModeRequest {
    pub const MSG_ID: u16 = 16174;
}

impl PbMessage for SetQuestFocusModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetQuestFocusModeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetQuestFocusModeResponse {
    pub const MSG_ID: u16 = 22593;
}

impl PbMessage for SetQuestFocusModeResponse {
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
pub struct CancelQuestFocusModeRequest {
    pub QuestId: i32,
}

impl CancelQuestFocusModeRequest {
    pub const MSG_ID: u16 = 15853;
}

impl PbMessage for CancelQuestFocusModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancelQuestFocusModeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CancelQuestFocusModeResponse {
    pub const MSG_ID: u16 = 28253;
}

impl PbMessage for CancelQuestFocusModeResponse {
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
pub struct AcceptFocusWaitQuestRequest {
    pub QuestId: i32,
}

impl AcceptFocusWaitQuestRequest {
    pub const MSG_ID: u16 = 25258;
}

impl PbMessage for AcceptFocusWaitQuestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AcceptFocusWaitQuestResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AcceptFocusWaitQuestResponse {
    pub const MSG_ID: u16 = 23033;
}

impl PbMessage for AcceptFocusWaitQuestResponse {
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
pub struct FocusQuestSetNotify {
    pub FocusQuestId: i32,
    pub Reason: Option<QuestFocusReason>,
}

impl FocusQuestSetNotify {
    pub const MSG_ID: u16 = 28873;
}

impl PbMessage for FocusQuestSetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FocusQuestId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FocusQuestId);
        }
        if let Some(v) = &self.Reason {
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
            7 => self.FocusQuestId = r.read_varint_i32()?,
            6 => {
                self.Reason = Some(
                    <QuestFocusReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FocusQuestChangeNotify {
    pub FocusQuestId: i32,
}

impl FocusQuestChangeNotify {
    pub const MSG_ID: u16 = 16030;
}

impl PbMessage for FocusQuestChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FocusQuestId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.FocusQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFocusModeDeterConditionRequest {
    pub DisableId: bool,
}

impl SetFocusModeDeterConditionRequest {
    pub const MSG_ID: u16 = 23749;
}

impl PbMessage for SetFocusModeDeterConditionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DisableId {
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
            4 => self.DisableId = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFocusModeDeterConditionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetFocusModeDeterConditionResponse {
    pub const MSG_ID: u16 = 15892;
}

impl PbMessage for SetFocusModeDeterConditionResponse {
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
pub struct ClientPullResourcePackageRequest {
    pub Holder: bool,
}

impl ClientPullResourcePackageRequest {
    pub const MSG_ID: u16 = 28300;
}

impl PbMessage for ClientPullResourcePackageRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Holder {
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
            11 => self.Holder = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientPullResourcePackageResponse {
    pub ErrorId: Option<ErrorCode>,
    pub FinishMp4QuestIds: Vec<i32>,
    pub NeedConfirmQuestIds: Vec<i32>,
}

impl ClientPullResourcePackageResponse {
    pub const MSG_ID: u16 = 28454;
}

impl PbMessage for ClientPullResourcePackageResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FinishMp4QuestIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.NeedConfirmQuestIds {
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
            13 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishMp4QuestIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.NeedConfirmQuestIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewDataRequest {}

impl QuestReviewDataRequest {
    pub const MSG_ID: u16 = 23616;
}

impl PbMessage for QuestReviewDataRequest {
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
pub struct QuestReviewDataResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestReviewDataResponse {
    pub const MSG_ID: u16 = 25121;
}

impl PbMessage for QuestReviewDataResponse {
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
pub struct QuestBranchInfo {
    pub QuestBranchInfos: Vec<OneQuestBranchPageInfo>,
    pub UnlockTimePoints: Vec<i32>,
    pub UnlockBranchComponentsGroup: Vec<i32>,
}

impl PbMessage for QuestBranchInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestBranchInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.UnlockTimePoints {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBranchComponentsGroup {
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
                self.QuestBranchInfos
                    .push(<OneQuestBranchPageInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockTimePoints,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBranchComponentsGroup,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneQuestBranchPageInfo {
    pub id: i32,
    pub CurBranch: i32,
    pub CompleteBranches: Vec<i32>,
}

impl PbMessage for OneQuestBranchPageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.id);
        }
        if self.CurBranch != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurBranch);
        }
        for item in &self.CompleteBranches {
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
            1 => self.id = r.read_varint_i32()?,
            2 => self.CurBranch = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompleteBranches,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestBranchRequest {}

impl QuestBranchRequest {
    pub const MSG_ID: u16 = 19642;
}

impl PbMessage for QuestBranchRequest {
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
pub struct QuestBranchResponse {
    pub errorCode: Option<ErrorCode>,
    pub QuestBranchInfo: Option<QuestBranchInfo>,
}

impl QuestBranchResponse {
    pub const MSG_ID: u16 = 20238;
}

impl PbMessage for QuestBranchResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.errorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.QuestBranchInfo {
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
            6 => {
                self.errorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.QuestBranchInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAccessInfo {
    pub EntityId: i64,
    pub RangeType: Option<RangeType>,
    pub AcessRangeResults: BTreeMap<i32, ErrorCode>,
}

impl PbMessage for EntityAccessInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.RangeType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.AcessRangeResults.is_empty() {
            crate::wire::write_map(out, 3, &self.AcessRangeResults);
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
                self.RangeType = Some(<RangeType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => crate::wire::decode_map_entry(r, &mut self.AcessRangeResults)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAccessRangeRequest {
    pub EntityId: i64,
    pub EntitiesToCheck: Vec<i64>,
    pub RangeType: Option<RangeType>,
}

impl EntityAccessRangeRequest {
    pub const MSG_ID: u16 = 22475;
}

impl PbMessage for EntityAccessRangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.EntitiesToCheck {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.RangeType {
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
            9 => self.EntityId = r.read_varint_i64()?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EntitiesToCheck,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            7 => {
                self.RangeType = Some(<RangeType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAccessRangeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EntityId: i64,
    pub Info: Vec<EntityAccessInfo>,
}

impl EntityAccessRangeResponse {
    pub const MSG_ID: u16 = 15094;
}

impl PbMessage for EntityAccessRangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.Info {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.EntityId = r.read_varint_i64()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .push(<EntityAccessInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessEffectAreaRequest {
    pub EntityId: i64,
    pub RangeType: Option<RangeType>,
}

impl PlayerAccessEffectAreaRequest {
    pub const MSG_ID: u16 = 20505;
}

impl PbMessage for PlayerAccessEffectAreaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.RangeType {
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
            2 => self.EntityId = r.read_varint_i64()?,
            9 => {
                self.RangeType = Some(<RangeType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessEffectAreaResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EntityId: i64,
    pub Info: Option<EntityAccessInfo>,
}

impl PlayerAccessEffectAreaResponse {
    pub const MSG_ID: u16 = 29671;
}

impl PbMessage for PlayerAccessEffectAreaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.EntityId = r.read_varint_i64()?,
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
pub struct InitRangeRequest {
    pub EntityId: i64,
    pub EntitiesToRequest: Vec<i64>,
    pub IsPlayerInRange: bool,
}

impl InitRangeRequest {
    pub const MSG_ID: u16 = 16207;
}

impl PbMessage for InitRangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.EntitiesToRequest {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.IsPlayerInRange {
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
            4 => self.EntityId = r.read_varint_i64()?,
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EntitiesToRequest,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            10 => self.IsPlayerInRange = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InitRangeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EntityId: i64,
    pub Info: Vec<EntityAccessInfo>,
    pub PlayerAccessRangeResult: Option<EntityAccessInfo>,
}

impl InitRangeResponse {
    pub const MSG_ID: u16 = 18075;
}

impl PbMessage for InitRangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.Info {
            crate::wire::write_submessage(out, 12, item);
        }
        if let Some(v) = &self.PlayerAccessRangeResult {
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.EntityId = r.read_varint_i64()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .push(<EntityAccessInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerAccessRangeResult
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtraDeadInfoMessageOneof {
    BtBloodBathedModeInfo(BtBloodBathedModeInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExtraDeadInfo {
    pub Message: Option<ExtraDeadInfoMessageOneof>,
}

impl PbMessage for ExtraDeadInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                ExtraDeadInfoMessageOneof::BtBloodBathedModeInfo(v) => {
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
                let val = <BtBloodBathedModeInfo as PbMessage>::decode(bytes)?;
                self.Message = Some(ExtraDeadInfoMessageOneof::BtBloodBathedModeInfo(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerDeadNotify {
    pub PlayerId: i32,
    pub DelaySeconds: i32,
    pub IsAutoRevive: bool,
    pub ReviveId: i32,
    pub IsLogin: bool,
    pub IsShowRevive: bool,
    pub ExtraDeadInfos: Vec<ExtraDeadInfo>,
}

impl PlayerDeadNotify {
    pub const MSG_ID: u16 = 23671;
}

impl PbMessage for PlayerDeadNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.DelaySeconds != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DelaySeconds);
        }
        if self.IsAutoRevive {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ReviveId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveId);
        }
        if self.IsLogin {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsShowRevive {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ExtraDeadInfos {
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
            5 => self.PlayerId = r.read_varint_i32()?,
            8 => self.DelaySeconds = r.read_varint_i32()?,
            15 => self.IsAutoRevive = r.read_bool()?,
            10 => self.ReviveId = r.read_varint_i32()?,
            11 => self.IsLogin = r.read_bool()?,
            2 => self.IsShowRevive = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraDeadInfos
                    .push(<ExtraDeadInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeathStatusInfo {
    pub GroupType: i32,
    pub LivingStatus: Option<LivingStatus>,
}

impl PbMessage for DeathStatusInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupType);
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
            1 => self.GroupType = r.read_varint_i32()?,
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
pub struct AliveStatusNotify {
    pub PlayerId: i32,
    pub Info: Vec<DeathStatusInfo>,
}

impl AliveStatusNotify {
    pub const MSG_ID: u16 = 26412;
}

impl PbMessage for AliveStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.Info {
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
                self.Info
                    .push(<DeathStatusInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtBloodBathedModeInfo {
    pub BtType: Option<BtType>,
    pub BtObjId: i32,
    pub BtObjSetting: i32,
}

impl PbMessage for BtBloodBathedModeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BtType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BtObjId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjId);
        }
        if self.BtObjSetting != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjSetting);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.BtType = Some(<BtType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.BtObjId = r.read_varint_i32()?,
            3 => self.BtObjSetting = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeCurrencyNotify {
    pub V2s: BTreeMap<i32, i32>,
}

impl RoguelikeCurrencyNotify {
    pub const MSG_ID: u16 = 29861;
}

impl PbMessage for RoguelikeCurrencyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.V2s.is_empty() {
            crate::wire::write_map(out, 12, &self.V2s);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => crate::wire::decode_map_entry(r, &mut self.V2s)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitPlayerNeedBlockPush {
    pub PlayerNeedBlockId: Vec<i32>,
}

impl SceneBlockSplitPlayerNeedBlockPush {
    pub const MSG_ID: u16 = 29579;
}

impl PbMessage for SceneBlockSplitPlayerNeedBlockPush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerNeedBlockId {
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
                &mut self.PlayerNeedBlockId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetRewardTreasureBoxRequest {
    pub EntityId: i64,
}

impl GetRewardTreasureBoxRequest {
    pub const MSG_ID: u16 = 20848;
}

impl PbMessage for GetRewardTreasureBoxRequest {
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
pub struct GetRewardTreasureBoxResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GetRewardTreasureBoxResponse {
    pub const MSG_ID: u16 = 23102;
}

impl PbMessage for GetRewardTreasureBoxResponse {
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
pub struct MobileButtonSetting {
    pub Id: i32,
    pub Size: f32,
    pub Transparency: f32,
    pub ScreenX: f32,
    pub ScreenY: f32,
    pub ButtonLevel: i32,
    pub PanelLevel: i32,
}

impl PbMessage for MobileButtonSetting {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Size != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Size).to_bits().to_le_bytes());
        }
        if self.Transparency != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Transparency).to_bits().to_le_bytes());
        }
        if self.ScreenX != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ScreenX).to_bits().to_le_bytes());
        }
        if self.ScreenY != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ScreenY).to_bits().to_le_bytes());
        }
        if self.ButtonLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ButtonLevel);
        }
        if self.PanelLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PanelLevel);
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
            2 => self.Size = r.read_f32()?,
            3 => self.Transparency = r.read_f32()?,
            4 => self.ScreenX = r.read_f32()?,
            5 => self.ScreenY = r.read_f32()?,
            6 => self.ButtonLevel = r.read_varint_i32()?,
            7 => self.PanelLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SettingNotify {
    pub MobileButtonSettings: Vec<MobileButtonSetting>,
}

impl SettingNotify {
    pub const MSG_ID: u16 = 17038;
}

impl PbMessage for SettingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MobileButtonSettings {
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
                self.MobileButtonSettings
                    .push(<MobileButtonSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LanguageSettingUpdateRequest {
    pub Language: i32,
}

impl LanguageSettingUpdateRequest {
    pub const MSG_ID: u16 = 22740;
}

impl PbMessage for LanguageSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Language != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.Language = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LanguageSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LanguageSettingUpdateResponse {
    pub const MSG_ID: u16 = 23816;
}

impl PbMessage for LanguageSettingUpdateResponse {
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
pub struct ServerPlayStationPlayOnlyStateRequest {}

impl ServerPlayStationPlayOnlyStateRequest {
    pub const MSG_ID: u16 = 29119;
}

impl PbMessage for ServerPlayStationPlayOnlyStateRequest {
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
pub struct ServerPlayStationPlayOnlyStateResponse {
    pub CrossPlayEnabled: bool,
}

impl ServerPlayStationPlayOnlyStateResponse {
    pub const MSG_ID: u16 = 23928;
}

impl PbMessage for ServerPlayStationPlayOnlyStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CrossPlayEnabled {
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
            5 => self.CrossPlayEnabled = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombinationKey {
    pub KeyNameList: Vec<Vec<u8>>,
}

impl PbMessage for CombinationKey {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.KeyNameList {
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
            1 => self.KeyNameList.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputAction {
    pub ActionName: Vec<u8>,
    pub KeyNameList: Vec<Vec<u8>>,
    pub Version: i32,
    pub InputType: Option<SettingInputType>,
}

impl PbMessage for InputAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActionName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActionName);
        }
        for item in &self.KeyNameList {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if let Some(v) = &self.InputType {
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
            1 => self.ActionName = r.read_len_delimited()?.to_vec(),
            2 => self.KeyNameList.push(r.read_len_delimited()?.to_vec()),
            3 => self.Version = r.read_varint_i32()?,
            4 => {
                self.InputType = Some(
                    <SettingInputType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct InputAxis {
    pub AxisName: Vec<u8>,
    pub KeyScaleMap: BTreeMap<Vec<u8>, i32>,
    pub Version: i32,
    pub InputType: Option<SettingInputType>,
}

impl PbMessage for InputAxis {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AxisName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AxisName);
        }
        if !self.KeyScaleMap.is_empty() {
            crate::wire::write_map(out, 2, &self.KeyScaleMap);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if let Some(v) = &self.InputType {
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
            1 => self.AxisName = r.read_len_delimited()?.to_vec(),
            2 => crate::wire::decode_map_entry(r, &mut self.KeyScaleMap)?,
            3 => self.Version = r.read_varint_i32()?,
            4 => {
                self.InputType = Some(
                    <SettingInputType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct CombinationAction {
    pub ActionName: Vec<u8>,
    pub CombinationKeyList: Vec<CombinationKey>,
    pub Version: i32,
    pub InputType: Option<SettingInputType>,
}

impl PbMessage for CombinationAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActionName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActionName);
        }
        for item in &self.CombinationKeyList {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if let Some(v) = &self.InputType {
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
            1 => self.ActionName = r.read_len_delimited()?.to_vec(),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CombinationKeyList
                    .push(<CombinationKey as PbMessage>::decode(bytes)?);
            }
            3 => self.Version = r.read_varint_i32()?,
            4 => {
                self.InputType = Some(
                    <SettingInputType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct CombinationAxis {
    pub AxisName: Vec<u8>,
    pub CombinationKeyList: Vec<CombinationKey>,
    pub Version: i32,
    pub InputType: Option<SettingInputType>,
}

impl PbMessage for CombinationAxis {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AxisName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AxisName);
        }
        for item in &self.CombinationKeyList {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if let Some(v) = &self.InputType {
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
            1 => self.AxisName = r.read_len_delimited()?.to_vec(),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CombinationKeyList
                    .push(<CombinationKey as PbMessage>::decode(bytes)?);
            }
            3 => self.Version = r.read_varint_i32()?,
            4 => {
                self.InputType = Some(
                    <SettingInputType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct InputSettingData {
    pub InputSettings: Vec<DeviceInputSetting>,
}

impl PbMessage for InputSettingData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InputSettings {
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
                self.InputSettings
                    .push(<DeviceInputSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeviceInputSetting {
    pub Device: Option<InputSettingDevice>,
    pub DeviceSubType: Vec<u8>,
    pub InputAction: Vec<InputAction>,
    pub InputAxis: Vec<InputAxis>,
    pub InputCombinationAction: Vec<CombinationAction>,
    pub InputCombinationAxis: Vec<CombinationAxis>,
}

impl PbMessage for DeviceInputSetting {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Device {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.DeviceSubType.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceSubType);
        }
        for item in &self.InputAction {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.InputAxis {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.InputCombinationAction {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.InputCombinationAxis {
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
                self.Device = Some(
                    <InputSettingDevice as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => self.DeviceSubType = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.InputAction
                    .push(<InputAction as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.InputAxis
                    .push(<InputAxis as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.InputCombinationAction
                    .push(<CombinationAction as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.InputCombinationAxis
                    .push(<CombinationAxis as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputSettingRequest {}

impl InputSettingRequest {
    pub const MSG_ID: u16 = 28228;
}

impl PbMessage for InputSettingRequest {
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
pub struct InputSettingResponse {
    pub InputSettingData: Option<InputSettingData>,
}

impl InputSettingResponse {
    pub const MSG_ID: u16 = 20083;
}

impl PbMessage for InputSettingResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InputSettingData {
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
pub struct InputSettingUpdateRequest {
    pub InputSettingData: Option<InputSettingData>,
}

impl InputSettingUpdateRequest {
    pub const MSG_ID: u16 = 20344;
}

impl PbMessage for InputSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InputSettingData {
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
pub struct InputSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InputSettingUpdateResponse {
    pub const MSG_ID: u16 = 20916;
}

impl PbMessage for InputSettingUpdateResponse {
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
pub struct XboxSettingRequest {}

impl XboxSettingRequest {
    pub const MSG_ID: u16 = 17199;
}

impl PbMessage for XboxSettingRequest {
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
pub struct XboxSettingResponse {
    pub MatchXboxUser: bool,
    pub XboxBlockUserCreatedContentViewing: bool,
}

impl XboxSettingResponse {
    pub const MSG_ID: u16 = 16665;
}

impl PbMessage for XboxSettingResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MatchXboxUser {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.XboxBlockUserCreatedContentViewing {
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
            1 => self.MatchXboxUser = r.read_bool()?,
            10 => self.XboxBlockUserCreatedContentViewing = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffCriminalInfo {
    pub CriminalId: i32,
    pub state: i32,
    pub IdentityId: i32,
}

impl PbMessage for SheriffCriminalInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CriminalId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CriminalId);
        }
        if self.state != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.state);
        }
        if self.IdentityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IdentityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CriminalId = r.read_varint_i32()?,
            2 => self.state = r.read_varint_i32()?,
            3 => self.IdentityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffAnomalyInfo {
    pub AnomalyId: i32,
    pub ClueIds: Vec<i32>,
    pub ProgressIds: Vec<i32>,
    pub EndingId: i32,
    pub CompleteTime: i64,
    pub IsActivated: bool,
    pub IsUnlocked: bool,
}

impl PbMessage for SheriffAnomalyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AnomalyId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AnomalyId);
        }
        for item in &self.ClueIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ProgressIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EndingId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndingId);
        }
        if self.CompleteTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CompleteTime);
        }
        if self.IsActivated {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsUnlocked {
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
            1 => self.AnomalyId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ClueIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ProgressIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.EndingId = r.read_varint_i32()?,
            5 => self.CompleteTime = r.read_varint_i64()?,
            6 => self.IsActivated = r.read_bool()?,
            7 => self.IsUnlocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffZoneInfo {
    pub ZoneId: i32,
    pub AnomalyInfos: Vec<SheriffAnomalyInfo>,
    pub CriminalInfos: Vec<SheriffCriminalInfo>,
    pub ItemCount: i64,
}

impl PbMessage for SheriffZoneInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ZoneId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ZoneId);
        }
        for item in &self.AnomalyInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.CriminalInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ItemCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ZoneId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AnomalyInfos
                    .push(<SheriffAnomalyInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CriminalInfos
                    .push(<SheriffCriminalInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.ItemCount = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffZoneInfoRequest {
    pub ZoneIds: Vec<i32>,
}

impl SheriffZoneInfoRequest {
    pub const MSG_ID: u16 = 20973;
}

impl PbMessage for SheriffZoneInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ZoneIds {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ZoneIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffZoneInfoResponse {
    pub errorCode: Option<ErrorCode>,
    pub ZoneInfos: Vec<SheriffZoneInfo>,
}

impl SheriffZoneInfoResponse {
    pub const MSG_ID: u16 = 25351;
}

impl PbMessage for SheriffZoneInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.errorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ZoneInfos {
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
            12 => {
                self.errorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ZoneInfos
                    .push(<SheriffZoneInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageInfo {
    pub ConfigId: i32,
    pub LastConfigId: i32,
    pub IsRead: bool,
    pub IsReceived: bool,
    pub Options: BTreeMap<i32, i32>,
    pub UnlockTime: i64,
    pub IsFinish: bool,
}

impl PbMessage for ShortMessageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.LastConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastConfigId);
        }
        if self.IsRead {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsReceived {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Options.is_empty() {
            crate::wire::write_map(out, 5, &self.Options);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsFinish {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.LastConfigId = r.read_varint_i32()?,
            3 => self.IsRead = r.read_bool()?,
            4 => self.IsReceived = r.read_bool()?,
            5 => crate::wire::decode_map_entry(r, &mut self.Options)?,
            6 => self.UnlockTime = r.read_varint_i64()?,
            7 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AllMsgRequest {}

impl AllMsgRequest {
    pub const MSG_ID: u16 = 19425;
}

impl PbMessage for AllMsgRequest {
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
pub struct AllMsgResponse {
    pub ShortMessageInfos: Vec<ShortMessageInfo>,
    pub BubbleIds: Vec<i32>,
    pub BubbleId: i32,
    pub ChatBgIds: Vec<i32>,
    pub ChatBgId: i32,
    pub ErrCode: Option<ErrorCode>,
    pub PartnerChange: Vec<i32>,
    pub ChatThemeId: i32,
    pub ChatThemeIds: Vec<i32>,
    pub ChatCharmId: i32,
    pub ChatCharmIds: Vec<i32>,
}

impl AllMsgResponse {
    pub const MSG_ID: u16 = 17966;
}

impl PbMessage for AllMsgResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShortMessageInfos {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.BubbleIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.BubbleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BubbleId);
        }
        for item in &self.ChatBgIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatBgId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatBgId);
        }
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PartnerChange {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatThemeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatThemeId);
        }
        for item in &self.ChatThemeIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatCharmId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatCharmId);
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ShortMessageInfos
                    .push(<ShortMessageInfo as PbMessage>::decode(bytes)?);
            }
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BubbleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.BubbleId = r.read_varint_i32()?,
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatBgIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.ChatBgId = r.read_varint_i32()?,
            14 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PartnerChange, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => self.ChatThemeId = r.read_varint_i32()?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatThemeIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.ChatCharmId = r.read_varint_i32()?,
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
pub struct BattleFormation {
    pub SelectRoles: Vec<i32>,
    pub BuffSelect: i32,
    pub SkillBranchIds: Vec<i32>,
}

impl PbMessage for BattleFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectRoles {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.BuffSelect != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffSelect);
        }
        for item in &self.SkillBranchIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectRoles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.BuffSelect = r.read_varint_i32()?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashLevelPlayInfo {
    pub Id: i32,
    pub IsLocked: bool,
    pub FirstScore: i32,
    pub SecondScore: i32,
    pub FirstBattle: Option<BattleFormation>,
    pub SecondBattle: Option<BattleFormation>,
    pub IsPassed: bool,
    pub IsEasyPass: bool,
}

impl PbMessage for SlashLevelPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsLocked {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FirstScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstScore);
        }
        if self.SecondScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondScore);
        }
        if let Some(v) = &self.FirstBattle {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.SecondBattle {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.IsPassed {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsEasyPass {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.IsLocked = r.read_bool()?,
            3 => self.FirstScore = r.read_varint_i32()?,
            4 => self.SecondScore = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FirstBattle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SecondBattle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.IsPassed = r.read_bool()?,
            8 => self.IsEasyPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerInfoRequest {}

impl SlashAndTowerInfoRequest {
    pub const MSG_ID: u16 = 21662;
}

impl PbMessage for SlashAndTowerInfoRequest {
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
pub struct SlashAndTowerInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SlashLevelPlayInfo: Vec<SlashLevelPlayInfo>,
    pub RewardsReceived: Vec<i32>,
    pub CurSeasonEndTime: i64,
    pub UpdateSeason: bool,
    pub CurIsHaveRecord: bool,
    pub BuffCache: Vec<i32>,
}

impl SlashAndTowerInfoResponse {
    pub const MSG_ID: u16 = 29704;
}

impl PbMessage for SlashAndTowerInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SlashLevelPlayInfo {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.RewardsReceived {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CurSeasonEndTime != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurSeasonEndTime);
        }
        if self.UpdateSeason {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurIsHaveRecord {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.BuffCache {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SlashLevelPlayInfo
                    .push(<SlashLevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardsReceived,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => self.CurSeasonEndTime = r.read_varint_i64()?,
            11 => self.UpdateSeason = r.read_bool()?,
            1 => self.CurIsHaveRecord = r.read_bool()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffCache, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerInfo {
    pub CurrentSeason: i32,
    pub DataSeason: i32,
    pub TowerDifficulties: Vec<TowerDifficultyPb>,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub MaxUnlockDifficulty: i32,
    pub QuickPassId: i32,
}

impl PbMessage for TowerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrentSeason != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentSeason);
        }
        if self.DataSeason != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DataSeason);
        }
        for item in &self.TowerDifficulties {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.MaxUnlockDifficulty != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxUnlockDifficulty);
        }
        if self.QuickPassId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuickPassId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurrentSeason = r.read_varint_i32()?,
            2 => self.DataSeason = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TowerDifficulties
                    .push(<TowerDifficultyPb as PbMessage>::decode(bytes)?);
            }
            4 => self.BeginTime = r.read_varint_i64()?,
            5 => self.EndTime = r.read_varint_i64()?,
            6 => self.MaxUnlockDifficulty = r.read_varint_i32()?,
            7 => self.QuickPassId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDifficultyPb {
    pub Difficulty: i32,
    pub RewardIndex: Vec<i32>,
    pub TowerAreas: Vec<TowerAreaPb>,
    pub MaxStar: i32,
}

impl PbMessage for TowerDifficultyPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Difficulty != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Difficulty);
        }
        for item in &self.RewardIndex {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TowerAreas {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.MaxStar != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxStar);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Difficulty = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIndex, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TowerAreas
                    .push(<TowerAreaPb as PbMessage>::decode(bytes)?);
            }
            4 => self.MaxStar = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerAreaPb {
    pub AreaNum: i32,
    pub TowerFloors: Vec<TowerFloorPb>,
}

impl PbMessage for TowerAreaPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaNum);
        }
        for item in &self.TowerFloors {
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
            1 => self.AreaNum = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TowerFloors
                    .push(<TowerFloorPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerFloorPb {
    pub TowerConfigId: i32,
    pub Star: i32,
    pub Formation: Vec<TowerRolePb>,
    pub StarIndex: Vec<i32>,
    pub IsQuickPass: bool,
}

impl PbMessage for TowerFloorPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerConfigId);
        }
        if self.Star != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Star);
        }
        for item in &self.Formation {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.StarIndex {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsQuickPass {
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
            1 => self.TowerConfigId = r.read_varint_i32()?,
            2 => self.Star = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .push(<TowerRolePb as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StarIndex, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => self.IsQuickPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerRolePb {
    pub RoleId: i32,
    pub LeaveSkillId: i32,
    pub SkillBranchId: i32,
}

impl PbMessage for TowerRolePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.LeaveSkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeaveSkillId);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerRequest {}

impl TowerRequest {
    pub const MSG_ID: u16 = 26406;
}

impl PbMessage for TowerRequest {
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
pub struct TowerResponse {
    pub TowerInfo: Option<TowerInfo>,
}

impl TowerResponse {
    pub const MSG_ID: u16 = 21047;
}

impl PbMessage for TowerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TowerInfo {
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
                self.TowerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerSeasonUpdateRequest {}

impl TowerSeasonUpdateRequest {
    pub const MSG_ID: u16 = 24072;
}

impl PbMessage for TowerSeasonUpdateRequest {
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
pub enum TowerSeasonUpdateResponseTowersOneof {
    TowerInfo(TowerInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerSeasonUpdateResponse {
    pub Towers: Option<TowerSeasonUpdateResponseTowersOneof>,
    pub MaxUnlockDifficulty: i32,
}

impl TowerSeasonUpdateResponse {
    pub const MSG_ID: u16 = 21534;
}

impl PbMessage for TowerSeasonUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Towers {
            match oneof {
                TowerSeasonUpdateResponseTowersOneof::TowerInfo(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.MaxUnlockDifficulty != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxUnlockDifficulty);
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
                let val = <TowerInfo as PbMessage>::decode(bytes)?;
                self.Towers = Some(TowerSeasonUpdateResponseTowersOneof::TowerInfo(val));
            }
            13 => self.MaxUnlockDifficulty = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoonChasingTrackMoonHandbookRewardNotify {
    pub Ids: Vec<i32>,
}

impl MoonChasingTrackMoonHandbookRewardNotify {
    pub const MSG_ID: u16 = 28408;
}

impl PbMessage for MoonChasingTrackMoonHandbookRewardNotify {
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
pub struct MoonChasingTargetGetCountNotify {
    pub TargetGetCount: i32,
}

impl MoonChasingTargetGetCountNotify {
    pub const MSG_ID: u16 = 19882;
}

impl PbMessage for MoonChasingTargetGetCountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetGetCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetGetCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.TargetGetCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TutorialInfo {
    pub Id: i32,
    pub CreateTime: u32,
    pub GetAward: bool,
}

impl PbMessage for TutorialInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CreateTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.CreateTime));
        }
        if self.GetAward {
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
            2 => self.CreateTime = r.read_varint_u32()?,
            3 => self.GetAward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TutorialInfoRequest {}

impl TutorialInfoRequest {
    pub const MSG_ID: u16 = 15006;
}

impl PbMessage for TutorialInfoRequest {
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
pub struct TutorialInfoResponse {
    pub UnlockList: Vec<TutorialInfo>,
}

impl TutorialInfoResponse {
    pub const MSG_ID: u16 = 22904;
}

impl PbMessage for TutorialInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockList {
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
                self.UnlockList
                    .push(<TutorialInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TutorialReceiveRequest {
    pub Id: i32,
}

impl TutorialReceiveRequest {
    pub const MSG_ID: u16 = 27270;
}

impl PbMessage for TutorialReceiveRequest {
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
pub struct TutorialReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl TutorialReceiveResponse {
    pub const MSG_ID: u16 = 18170;
}

impl PbMessage for TutorialReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 1, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 3, &self.ItemMap);
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
            1 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            3 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TutorialUnlockRequest {
    pub Id: i32,
}

impl TutorialUnlockRequest {
    pub const MSG_ID: u16 = 21852;
}

impl PbMessage for TutorialUnlockRequest {
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
pub struct TutorialUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub UnLockInfo: Option<TutorialInfo>,
}

impl TutorialUnlockResponse {
    pub const MSG_ID: u16 = 26280;
}

impl PbMessage for TutorialUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 14, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if let Some(v) = &self.UnLockInfo {
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            12 => {
                let bytes = r.read_len_delimited()?;
                self.UnLockInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponConsumeItem {
    pub IncId: i32,
    pub Count: i32,
    pub ItemId: i32,
}

impl PbMessage for WeaponConsumeItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
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
            2 => self.Count = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponLevelUpRequest {
    pub IncId: i32,
    pub ConsumeList: Vec<WeaponConsumeItem>,
}

impl WeaponLevelUpRequest {
    pub const MSG_ID: u16 = 19242;
}

impl PbMessage for WeaponLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        for item in &self.ConsumeList {
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
            8 => self.IncId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.ConsumeList
                    .push(<WeaponConsumeItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IncId: i32,
    pub WeaponLevel: i32,
    pub WeaponExp: i32,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl WeaponLevelUpResponse {
    pub const MSG_ID: u16 = 17061;
}

impl PbMessage for WeaponLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.WeaponLevel != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponLevel);
        }
        if self.WeaponExp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponExp);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 13, &self.ItemMap);
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
            14 => self.IncId = r.read_varint_i32()?,
            9 => self.WeaponLevel = r.read_varint_i32()?,
            1 => self.WeaponExp = r.read_varint_i32()?,
            13 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponBreachRequest {
    pub IncId: i32,
}

impl WeaponBreachRequest {
    pub const MSG_ID: u16 = 16833;
}

impl PbMessage for WeaponBreachRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponBreachResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IncId: i32,
    pub WeaponBreach: i32,
}

impl WeaponBreachResponse {
    pub const MSG_ID: u16 = 26514;
}

impl PbMessage for WeaponBreachResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.WeaponBreach != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBreach);
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
            4 => self.IncId = r.read_varint_i32()?,
            12 => self.WeaponBreach = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponResonUpRequest {
    pub IncId: i32,
    pub ConsumeList: Vec<i32>,
    pub ConsumeItemList: Vec<WeaponConsumeItem>,
}

impl WeaponResonUpRequest {
    pub const MSG_ID: u16 = 17476;
}

impl PbMessage for WeaponResonUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        for item in &self.ConsumeList {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ConsumeItemList {
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
            1 => self.IncId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ConsumeList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ConsumeItemList
                    .push(<WeaponConsumeItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponResonUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IncId: i32,
    pub ResonLevel: i32,
}

impl WeaponResonUpResponse {
    pub const MSG_ID: u16 = 21310;
}

impl PbMessage for WeaponResonUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ResonLevel != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResonLevel);
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
            9 => self.IncId = r.read_varint_i32()?,
            11 => self.ResonLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeeklyFrameworkInfoRequest {}

impl WeeklyFrameworkInfoRequest {
    pub const MSG_ID: u16 = 29511;
}

impl PbMessage for WeeklyFrameworkInfoRequest {
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
pub struct WeeklyFrameworkInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FrameworkInfo: Option<WeeklyFrameworkInfo>,
}

impl WeeklyFrameworkInfoResponse {
    pub const MSG_ID: u16 = 22499;
}

impl PbMessage for WeeklyFrameworkInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
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
pub struct WeeklyFrameworkInfo {
    pub ConfigId: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub ScoreTasks: Vec<i32>,
    pub WeeklyPlayDatas: Vec<WeeklyPlayData>,
    pub WorldLevel: i32,
}

impl PbMessage for WeeklyFrameworkInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.ScoreTasks {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.WeeklyPlayDatas {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.WorldLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            3 => self.BeginTime = r.read_varint_i64()?,
            4 => self.EndTime = r.read_varint_i64()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTasks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.WeeklyPlayDatas
                    .push(<WeeklyPlayData as PbMessage>::decode(bytes)?);
            }
            7 => self.WorldLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeeklyPlayDataQWpOneof {
    RogueWeeklyPlayData(RogueWeeklyPlayData),
    FloroFarmPlayData(FloroFarmPlayData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeeklyPlayData {
    pub qWp: Option<WeeklyPlayDataQWpOneof>,
    pub id: i32,
    pub r#type: Option<ActivityType>,
}

impl PbMessage for WeeklyPlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.qWp {
            match oneof {
                WeeklyPlayDataQWpOneof::RogueWeeklyPlayData(v) => {
                    crate::wire::write_submessage(out, 10000, v);
                }
                WeeklyPlayDataQWpOneof::FloroFarmPlayData(v) => {
                    crate::wire::write_submessage(out, 10001, v);
                }
            }
        }
        if self.id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.id);
        }
        if let Some(v) = &self.r#type {
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
            10000 => {
                let bytes = r.read_len_delimited()?;
                let val = <RogueWeeklyPlayData as PbMessage>::decode(bytes)?;
                self.qWp = Some(WeeklyPlayDataQWpOneof::RogueWeeklyPlayData(val));
            }
            10001 => {
                let bytes = r.read_len_delimited()?;
                let val = <FloroFarmPlayData as PbMessage>::decode(bytes)?;
                self.qWp = Some(WeeklyPlayDataQWpOneof::FloroFarmPlayData(val));
            }
            1 => self.id = r.read_varint_i32()?,
            2 => {
                self.r#type = Some(<ActivityType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
