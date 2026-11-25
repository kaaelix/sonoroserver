use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLinkageTabData {
    pub TabDataId: i32,
    pub EndTime: i64,
    pub RewardData: Vec<ActivityLinkageRewardData>,
    pub IsReceive: bool,
    pub StartTime: i64,
}

impl PbMessage for ActivityLinkageTabData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TabDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabDataId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.RewardData {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsReceive {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.TabDataId = r.read_varint_i32()?,
            2 => self.EndTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RewardData
                    .push(<ActivityLinkageRewardData as PbMessage>::decode(bytes)?);
            }
            4 => self.IsReceive = r.read_bool()?,
            5 => self.StartTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLinkageData {
    pub ActivityId: i32,
    pub Data: Vec<ActivityLinkageTabData>,
}

impl PbMessage for ActivityLinkageData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Data {
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<ActivityLinkageTabData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestionaireRewardState {
    pub Id: i32,
    pub Status: Option<ActivityTaskState>,
}

impl PbMessage for QuestionaireRewardState {
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
                    <ActivityTaskState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ActivityRegressData {
    pub TaskProgressReward: Vec<ActivityTask>,
    pub ClaimedReward: Vec<ActivityTask>,
    pub TaskScoreRewardId: Vec<i32>,
    pub Grade: i32,
    pub EndTime: i64,
    pub RefreshTime: i64,
    pub BossDoubleDropCount: i32,
    pub WeekDoubleDropCount: i32,
    pub Questionnaire: Vec<i32>,
    pub QuestionaireRewardState: Vec<QuestionaireRewardState>,
    pub BossDoubleDropUnlock: bool,
    pub WeekDoubleDropUnlock: bool,
    pub PayScoreRewards: Vec<i32>,
    pub DisposableReward: bool,
    pub RoleInfo: Vec<NewTrialRoleInfo>,
    pub PayRewardUnlock: bool,
    pub CurUseTrialRoleId: i32,
    pub CurUseRoleInfo: Option<RoleInfo>,
}

impl PbMessage for ActivityRegressData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskProgressReward {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ClaimedReward {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.TaskScoreRewardId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Grade != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Grade);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.RefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RefreshTime);
        }
        if self.BossDoubleDropCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossDoubleDropCount);
        }
        if self.WeekDoubleDropCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekDoubleDropCount);
        }
        for item in &self.Questionnaire {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.QuestionaireRewardState {
            crate::wire::write_submessage(out, 13, item);
        }
        if self.BossDoubleDropUnlock {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.WeekDoubleDropUnlock {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.PayScoreRewards {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.DisposableReward {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleInfo {
            crate::wire::write_submessage(out, 16, item);
        }
        if self.PayRewardUnlock {
            crate::wire::write_tag(out, 17, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurUseTrialRoleId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurUseTrialRoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 19, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TaskProgressReward
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
            4 => self.Grade = r.read_varint_i32()?,
            6 => self.EndTime = r.read_varint_i64()?,
            7 => self.RefreshTime = r.read_varint_i64()?,
            8 => self.BossDoubleDropCount = r.read_varint_i32()?,
            9 => self.WeekDoubleDropCount = r.read_varint_i32()?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Questionnaire, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.QuestionaireRewardState
                    .push(<QuestionaireRewardState as PbMessage>::decode(bytes)?);
            }
            11 => self.BossDoubleDropUnlock = r.read_bool()?,
            12 => self.WeekDoubleDropUnlock = r.read_bool()?,
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PayScoreRewards,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => self.DisposableReward = r.read_bool()?,
            16 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .push(<NewTrialRoleInfo as PbMessage>::decode(bytes)?);
            }
            17 => self.PayRewardUnlock = r.read_bool()?,
            18 => self.CurUseTrialRoleId = r.read_varint_i32()?,
            19 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportActivityData {
    pub TrialRoleInfoList: Vec<NewTrialRoleInfo>,
    pub TaskDataList: Vec<ConditionTask>,
    pub CurUseTrialRoleId: i32,
    pub CurUseRoleInfo: Option<RoleInfo>,
    pub NewPlayerPoolFinalGachaRoleId: i32,
}

impl PbMessage for NewPlayerSupportActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrialRoleInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.TaskDataList {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CurUseTrialRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurUseTrialRoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.NewPlayerPoolFinalGachaRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewPlayerPoolFinalGachaRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TrialRoleInfoList
                    .push(<NewTrialRoleInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDataList
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            3 => self.CurUseTrialRoleId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.NewPlayerPoolFinalGachaRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTrialRoleInfo {
    pub TrialRoleId: i32,
    pub WorldLv: i32,
}

impl PbMessage for NewTrialRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrialRoleId);
        }
        if self.WorldLv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WorldLv);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TrialRoleId = r.read_varint_i32()?,
            2 => self.WorldLv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CumulativeShopTaskData {
    pub Current: i32,
    pub TargetProgress: i32,
}

impl PbMessage for CumulativeShopTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Current != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Current);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Current = r.read_varint_i32()?,
            2 => self.TargetProgress = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CumulativeShopSubTaskData {
    pub CanGetReward: i32,
    pub ProgressCount: i32,
    pub TotalProgressCount: i32,
}

impl PbMessage for CumulativeShopSubTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanGetReward != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanGetReward);
        }
        if self.ProgressCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProgressCount);
        }
        if self.TotalProgressCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalProgressCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CanGetReward = r.read_varint_i32()?,
            2 => self.ProgressCount = r.read_varint_i32()?,
            3 => self.TotalProgressCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CumulativeShopTaskConfig {
    pub Id: i32,
    pub Type: Option<ConsumptiveTaskType>,
    pub CumulativeShopTaskData: Option<CumulativeShopTaskData>,
    pub CumulativeShopSubTaskData: Option<CumulativeShopSubTaskData>,
}

impl PbMessage for CumulativeShopTaskConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CumulativeShopTaskData {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.CumulativeShopSubTaskData {
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
            2 => {
                self.Type = Some(
                    <ConsumptiveTaskType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CumulativeShopTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CumulativeShopSubTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CumulativeShopData {
    pub ActivityId: i32,
    pub TaskData: Vec<CumulativeShopTaskConfig>,
    pub TotalScore: i32,
}

impl PbMessage for CumulativeShopData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TaskData {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TaskData
                    .push(<CumulativeShopTaskConfig as PbMessage>::decode(bytes)?);
            }
            4 => self.TotalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleFlag {
    pub FlagId: i32,
    pub BoxReceivedCount: i32,
    pub BoxTotalCount: i32,
}

impl PbMessage for MoraleFlag {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlagId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlagId);
        }
        if self.BoxReceivedCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxReceivedCount);
        }
        if self.BoxTotalCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxTotalCount);
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
            2 => self.BoxReceivedCount = r.read_varint_i32()?,
            3 => self.BoxTotalCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleAreaData {
    pub AreaDataId: i32,
    pub ExploreBoxReceivedCount: i32,
}

impl PbMessage for MoraleAreaData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaDataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaDataId);
        }
        if self.ExploreBoxReceivedCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreBoxReceivedCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AreaDataId = r.read_varint_i32()?,
            2 => self.ExploreBoxReceivedCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityMoraleData {
    pub AreaData: Vec<MoraleAreaData>,
    pub MoraleProgressReward: Vec<i32>,
    pub MoraleFlags: Vec<MoraleFlag>,
}

impl PbMessage for ActivityMoraleData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.MoraleProgressReward {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MoraleFlags {
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
                self.AreaData
                    .push(<MoraleAreaData as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MoraleProgressReward,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MoraleFlags
                    .push(<MoraleFlag as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchActivityData {
    pub FloroRangeData: Option<FloroRangeData>,
    pub UnFinishedSubIns: i32,
    pub SavedStage: i32,
    pub CurWeeklyInsId: i32,
}

impl PbMessage for FloroRanchActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FloroRangeData {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.UnFinishedSubIns != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnFinishedSubIns);
        }
        if self.SavedStage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SavedStage);
        }
        if self.CurWeeklyInsId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWeeklyInsId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRangeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.UnFinishedSubIns = r.read_varint_i32()?,
            3 => self.SavedStage = r.read_varint_i32()?,
            4 => self.CurWeeklyInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchCommonData {
    pub DataId: i32,
    pub ConditionId: i32,
    pub IsLocked: bool,
}

impl PbMessage for FloroRanchCommonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DataId);
        }
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.IsLocked {
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
            1 => self.DataId = r.read_varint_i32()?,
            2 => self.ConditionId = r.read_varint_i32()?,
            3 => self.IsLocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSubDungeonData {
    pub DataId: i32,
    pub ConditionId: i32,
    pub IsLocked: bool,
    pub IsFinished: bool,
}

impl PbMessage for FloroRanchSubDungeonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DataId);
        }
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.IsLocked {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsFinished {
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
            1 => self.DataId = r.read_varint_i32()?,
            2 => self.ConditionId = r.read_varint_i32()?,
            3 => self.IsLocked = r.read_bool()?,
            4 => self.IsFinished = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRangeData {
    pub FloroRanchCardData: Vec<FloroRanchCommonData>,
    pub FloroRanchUnlockedTechDataIds: Vec<i32>,
    pub FloroRanchToyData: Vec<FloroRanchCommonData>,
    pub FloroRanchSkillData: Vec<FloroRanchCommonData>,
    pub FloroRanchMilestoneData: Vec<i32>,
    pub FloroRanchRaceData: Vec<FloroRanchCommonData>,
    pub FloroRanchSubDungeonData: Vec<FloroRanchSubDungeonData>,
    pub ConditionTask: Vec<ConditionTask>,
    pub FloroRanchSubDungeonHistoryData: Vec<FloroRanchSubDungeonHistoryData>,
    pub FloroRanchSubDungeonIdsRedDot: Vec<i32>,
    pub IsReadComic: bool,
    pub FloroRangeUnlockTime: i64,
    pub FloroRangeEndTime: i64,
    pub InsUnLockCondition: Vec<FloroRanchCommonData>,
}

impl PbMessage for FloroRangeData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FloroRanchCardData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.FloroRanchUnlockedTechDataIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FloroRanchToyData {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.FloroRanchSkillData {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.FloroRanchMilestoneData {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FloroRanchRaceData {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.FloroRanchSubDungeonData {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.ConditionTask {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.FloroRanchSubDungeonHistoryData {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.FloroRanchSubDungeonIdsRedDot {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsReadComic {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FloroRangeUnlockTime != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FloroRangeUnlockTime);
        }
        if self.FloroRangeEndTime != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FloroRangeEndTime);
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
                self.FloroRanchCardData
                    .push(<FloroRanchCommonData as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FloroRanchUnlockedTechDataIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchToyData
                    .push(<FloroRanchCommonData as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchSkillData
                    .push(<FloroRanchCommonData as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FloroRanchMilestoneData,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchRaceData
                    .push(<FloroRanchCommonData as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchSubDungeonData
                    .push(<FloroRanchSubDungeonData as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTask
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchSubDungeonHistoryData.push(
                    <FloroRanchSubDungeonHistoryData as PbMessage>::decode(bytes)?,
                );
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FloroRanchSubDungeonIdsRedDot,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => self.IsReadComic = r.read_bool()?,
            16 => self.FloroRangeUnlockTime = r.read_varint_i64()?,
            17 => self.FloroRangeEndTime = r.read_varint_i64()?,
            18 => {
                let bytes = r.read_len_delimited()?;
                self.InsUnLockCondition
                    .push(<FloroRanchCommonData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSubDungeonHistoryData {
    pub DataId: i32,
    pub MaxDays: i32,
    pub MaxCoins: i64,
}

impl PbMessage for FloroRanchSubDungeonHistoryData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DataId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DataId);
        }
        if self.MaxDays != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxDays);
        }
        if self.MaxCoins != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxCoins);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DataId = r.read_varint_i32()?,
            2 => self.MaxDays = r.read_varint_i32()?,
            3 => self.MaxCoins = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LifePointChallengeData {
    pub ChallengeId: i32,
    pub CanGetReward: bool,
    pub OpenTime: i64,
    pub RewardId: i32,
    pub EntityConfigId: i32,
    pub IsPreChallengeState: bool,
}

impl PbMessage for LifePointChallengeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CanGetReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.IsPreChallengeState {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.CanGetReward = r.read_bool()?,
            3 => self.OpenTime = r.read_varint_i64()?,
            4 => self.RewardId = r.read_varint_i32()?,
            5 => self.EntityConfigId = r.read_varint_i32()?,
            6 => self.IsPreChallengeState = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LifePointDrawActivityData {
    pub LifePointChallengeData: Vec<LifePointChallengeData>,
}

impl PbMessage for LifePointDrawActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LifePointChallengeData {
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
                self.LifePointChallengeData
                    .push(<LifePointChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityTrapDefenseData {
    pub TrapDefenseTalentNodeIds: Vec<i32>,
    pub SpecialReward: Vec<TrapDefenseRewardData>,
    pub Rewards: Vec<TrapDefenseRewardData>,
    pub Auxiliaries: Vec<TrapDefenseAuxiliaryData>,
    pub Buildings: Vec<TrapDefenseBuildingData>,
    pub Challenges: Vec<TrapDefenseLevelData>,
    pub StartTime: i64,
    pub EndTime: i64,
    pub TrapDefenseBdDataIdUnlocks: Vec<i32>,
    pub TrapDefenseTalentTreeMaxPoints: i32,
    pub TrapDefenseTalentTreePoints: i32,
    pub TrapDefenseRemainPoints: i32,
    pub TrapDefenseTotalPoints: i32,
    pub TrapDefenseBdBuffIdUnlocks: Vec<i32>,
}

impl PbMessage for ActivityTrapDefenseData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrapDefenseTalentNodeIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialReward {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Rewards {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.Auxiliaries {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.Buildings {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.Challenges {
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
        for item in &self.TrapDefenseBdDataIdUnlocks {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TrapDefenseTalentTreeMaxPoints != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrapDefenseTalentTreeMaxPoints);
        }
        if self.TrapDefenseTalentTreePoints != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrapDefenseTalentTreePoints);
        }
        if self.TrapDefenseRemainPoints != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrapDefenseRemainPoints);
        }
        if self.TrapDefenseTotalPoints != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrapDefenseTotalPoints);
        }
        for item in &self.TrapDefenseBdBuffIdUnlocks {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TrapDefenseTalentNodeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SpecialReward
                    .push(<TrapDefenseRewardData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Rewards
                    .push(<TrapDefenseRewardData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Auxiliaries
                    .push(<TrapDefenseAuxiliaryData as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Buildings
                    .push(<TrapDefenseBuildingData as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Challenges
                    .push(<TrapDefenseLevelData as PbMessage>::decode(bytes)?);
            }
            7 => self.StartTime = r.read_varint_i64()?,
            8 => self.EndTime = r.read_varint_i64()?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TrapDefenseBdDataIdUnlocks,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.TrapDefenseTalentTreeMaxPoints = r.read_varint_i32()?,
            11 => self.TrapDefenseTalentTreePoints = r.read_varint_i32()?,
            12 => self.TrapDefenseRemainPoints = r.read_varint_i32()?,
            13 => self.TrapDefenseTotalPoints = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TrapDefenseBdBuffIdUnlocks,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseRewardData {
    pub ActivityServerRewardItemData: Option<ConditionTask>,
    pub StartTime: i64,
    pub EndTime: i64,
}

impl PbMessage for TrapDefenseRewardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityServerRewardItemData {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityServerRewardItemData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.StartTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseLevelData {
    pub ChallengeId: i32,
    pub CanUnlock: bool,
    pub TargetProgress: Vec<i32>,
    pub IsPassed: bool,
    pub CanGetReward: bool,
    pub UnlockTime: i64,
    pub IsLeaved: bool,
    pub MaxFinishWaveTimes: i32,
}

impl PbMessage for TrapDefenseLevelData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CanUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.TargetProgress {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsPassed {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanGetReward {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsLeaved {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxFinishWaveTimes != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFinishWaveTimes);
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
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetProgress, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => self.IsPassed = r.read_bool()?,
            5 => self.CanGetReward = r.read_bool()?,
            6 => self.UnlockTime = r.read_varint_i64()?,
            7 => self.IsLeaved = r.read_bool()?,
            8 => self.MaxFinishWaveTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuildingData {
    pub Id: i32,
    pub Level: i32,
    pub Branch: i32,
    pub MaxLevel: i32,
    pub CellPrice: i32,
    pub OriginalConstructPrice: i32,
    pub DiscountConstructPrice: i32,
    pub DeconstructReturn: i32,
}

impl PbMessage for TrapDefenseBuildingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
        }
        if self.MaxLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLevel);
        }
        if self.CellPrice != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CellPrice);
        }
        if self.OriginalConstructPrice != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginalConstructPrice);
        }
        if self.DiscountConstructPrice != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscountConstructPrice);
        }
        if self.DeconstructReturn != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.Branch = r.read_varint_i32()?,
            4 => self.MaxLevel = r.read_varint_i32()?,
            5 => self.CellPrice = r.read_varint_i32()?,
            6 => self.OriginalConstructPrice = r.read_varint_i32()?,
            7 => self.DiscountConstructPrice = r.read_varint_i32()?,
            8 => self.DeconstructReturn = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryData {
    pub Id: i32,
    pub Level: i32,
    pub Branch: i32,
    pub MaxLevel: i32,
}

impl PbMessage for TrapDefenseAuxiliaryData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
        }
        if self.MaxLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLevel);
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
            3 => self.Branch = r.read_varint_i32()?,
            4 => self.MaxLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineCrossChallengeData {
    pub ChallengeId: i32,
    pub CanGetReward: bool,
    pub OpenTime: i64,
    pub RewardDataId: i32,
    pub EntityConfigId: i32,
    pub IsPreChallengeState: bool,
}

impl PbMessage for LineCrossChallengeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CanGetReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if self.RewardDataId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardDataId);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.IsPreChallengeState {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.CanGetReward = r.read_bool()?,
            3 => self.OpenTime = r.read_varint_i64()?,
            4 => self.RewardDataId = r.read_varint_i32()?,
            5 => self.EntityConfigId = r.read_varint_i32()?,
            6 => self.IsPreChallengeState = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLineCrossData {
    pub Challenges: Vec<LineCrossChallengeData>,
}

impl PbMessage for ActivityLineCrossData {
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
pub struct MoonSignInConfigData {
    pub MoonId: i32,
    pub MoonLabelTopId: i32,
    pub MoonLabelBottomId: i32,
}

impl PbMessage for MoonSignInConfigData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoonId);
        }
        if self.MoonLabelTopId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoonLabelTopId);
        }
        if self.MoonLabelBottomId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoonLabelBottomId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MoonId = r.read_varint_i32()?,
            2 => self.MoonLabelTopId = r.read_varint_i32()?,
            3 => self.MoonLabelBottomId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityMoonSignInData {
    pub MoonPhaseSelectList: Vec<MoonSignInConfigData>,
    pub IsGrandReward: bool,
    pub CurrentMoonId: i32,
}

impl PbMessage for ActivityMoonSignInData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MoonPhaseSelectList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.IsGrandReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurrentMoonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentMoonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.MoonPhaseSelectList
                    .push(<MoonSignInConfigData as PbMessage>::decode(bytes)?);
            }
            2 => self.IsGrandReward = r.read_bool()?,
            3 => self.CurrentMoonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightPhotoActivityData {
    pub ActivityId: i32,
    pub LevelGroups: Vec<LevelGroupData>,
    pub Tasks: Vec<TaskData>,
}

impl PbMessage for FightPhotoActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.LevelGroups {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Tasks {
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
                self.LevelGroups
                    .push(<LevelGroupData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks.push(<TaskData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelGroupData {
    pub GroupId: i32,
    pub OpenTime: i64,
    pub EndTime: i64,
    pub levels: Vec<LevelData>,
}

impl PbMessage for LevelGroupData {
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
        for item in &self.levels {
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
                self.levels.push(<LevelData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelData {
    pub LevelId: i32,
    pub InstId: i32,
    pub Roles: Vec<i32>,
    pub GroupId: i32,
    pub IsUnlocked: bool,
}

impl PbMessage for LevelData {
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
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.IsUnlocked {
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
            4 => self.GroupId = r.read_varint_i32()?,
            5 => self.IsUnlocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightPhotoLevelDataUpdateNotify {
    pub levels: Vec<LevelData>,
}

impl FightPhotoLevelDataUpdateNotify {
    pub const MSG_ID: u16 = 18787;
}

impl PbMessage for FightPhotoLevelDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.levels {
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
                self.levels.push(<LevelData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TaskData {
    pub Id: i32,
    pub Status: Option<ActivityTaskState>,
    pub Progress: BTreeMap<i32, i32>,
}

impl PbMessage for TaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Progress.is_empty() {
            crate::wire::write_map(out, 5, &self.Progress);
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
            5 => crate::wire::decode_map_entry(r, &mut self.Progress)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurvivorsLevelDataModeInfoOneof {
    EndlessInfo(SurvivorsLevelInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsLevelData {
    pub ModeInfo: Option<SurvivorsLevelDataModeInfoOneof>,
    pub LevelId: i32,
    pub OpenTime: i64,
    pub NormalInfo: Option<SurvivorsLevelInfo>,
}

impl PbMessage for SurvivorsLevelData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ModeInfo {
            match oneof {
                SurvivorsLevelDataModeInfoOneof::EndlessInfo(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if let Some(v) = &self.NormalInfo {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsLevelInfo as PbMessage>::decode(bytes)?;
                self.ModeInfo = Some(SurvivorsLevelDataModeInfoOneof::EndlessInfo(val));
            }
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.OpenTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.NormalInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsLevelInfo {
    pub IsUnlocked: bool,
    pub ConditionGroupId: i32,
    pub WaveId: i32,
    pub KillMonsterCount: i32,
    pub IsFinished: bool,
}

impl PbMessage for SurvivorsLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsUnlocked {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ConditionGroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionGroupId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
        if self.KillMonsterCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillMonsterCount);
        }
        if self.IsFinished {
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
            1 => self.IsUnlocked = r.read_bool()?,
            2 => self.ConditionGroupId = r.read_varint_i32()?,
            3 => self.WaveId = r.read_varint_i32()?,
            4 => self.KillMonsterCount = r.read_varint_i32()?,
            5 => self.IsFinished = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsActivityData {
    pub NormalTaskData: Option<ActivityTaskData>,
    pub ScoreTaskDatas: Vec<i32>,
    pub UnlockedWeapons: Vec<i32>,
    pub UnlockedRoles: Vec<i32>,
    pub UnlockedItems: Vec<i32>,
    pub TalentTreeNodes: BTreeMap<i32, i32>,
    pub SurvivorsChallengeInfos: Vec<SurvivorsLevelData>,
}

impl PbMessage for SurvivorsActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NormalTaskData {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ScoreTaskDatas {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedWeapons {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedRoles {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedItems {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.TalentTreeNodes.is_empty() {
            crate::wire::write_map(out, 6, &self.TalentTreeNodes);
        }
        for item in &self.SurvivorsChallengeInfos {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.NormalTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTaskDatas, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedWeapons,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedRoles, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedItems, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => crate::wire::decode_map_entry(r, &mut self.TalentTreeNodes)?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsChallengeInfos
                    .push(<SurvivorsLevelData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityPrizeDrawingData {
    pub ActivityId: i32,
    pub KujiId: i32,
    pub AwardGroups: Vec<AwardGroupData>,
    pub CostItemId: i32,
    pub CostItemCount: i32,
    pub QuestFinishedCount: i32,
    pub QuestTotalCount: i32,
    pub QuestId: i32,
}

impl PbMessage for ActivityPrizeDrawingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.KujiId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KujiId);
        }
        for item in &self.AwardGroups {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.CostItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostItemId);
        }
        if self.CostItemCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostItemCount);
        }
        if self.QuestFinishedCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestFinishedCount);
        }
        if self.QuestTotalCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestTotalCount);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.KujiId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.AwardGroups
                    .push(<AwardGroupData as PbMessage>::decode(bytes)?);
            }
            4 => self.CostItemId = r.read_varint_i32()?,
            5 => self.CostItemCount = r.read_varint_i32()?,
            6 => self.QuestFinishedCount = r.read_varint_i32()?,
            7 => self.QuestTotalCount = r.read_varint_i32()?,
            8 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AwardGroupData {
    pub GroupId: i32,
    pub GroupRank: i32,
    pub CurrentAmount: i32,
    pub AllAmount: i32,
    pub RewardItems: BTreeMap<i32, i32>,
}

impl PbMessage for AwardGroupData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.GroupRank != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupRank);
        }
        if self.CurrentAmount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentAmount);
        }
        if self.AllAmount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AllAmount);
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 5, &self.RewardItems);
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
            2 => self.GroupRank = r.read_varint_i32()?,
            3 => self.CurrentAmount = r.read_varint_i32()?,
            4 => self.AllAmount = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HonamiStoryItemInfoItemInfoOneof {
    HonamiStoryNormalItemInfo(HonamiStoryNormalItemInfo),
    EquipItemInfo(HonamiStoryEquipItemInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemInfo {
    pub ItemInfo: Option<HonamiStoryItemInfoItemInfoOneof>,
    pub IncrId: i32,
    pub ItemId: i32,
    pub FuncValue: i32,
}

impl PbMessage for HonamiStoryItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ItemInfo {
            match oneof {
                HonamiStoryItemInfoItemInfoOneof::HonamiStoryNormalItemInfo(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                HonamiStoryItemInfoItemInfoOneof::EquipItemInfo(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
            }
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryNormalItemInfo as PbMessage>::decode(bytes)?;
                self.ItemInfo = Some(HonamiStoryItemInfoItemInfoOneof::HonamiStoryNormalItemInfo(
                    val,
                ));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryEquipItemInfo as PbMessage>::decode(bytes)?;
                self.ItemInfo = Some(HonamiStoryItemInfoItemInfoOneof::EquipItemInfo(val));
            }
            1 => self.IncrId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.FuncValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryNormalItemInfo {}

impl PbMessage for HonamiStoryNormalItemInfo {
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
pub struct HonamiStoryEquipItemInfo {
    pub MainPropLibraryId: i32,
    pub OriBuffTempId: Vec<i32>,
    pub ChildBuffTempId: Vec<i32>,
}

impl PbMessage for HonamiStoryEquipItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MainPropLibraryId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MainPropLibraryId);
        }
        for item in &self.OriBuffTempId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ChildBuffTempId {
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
            1 => self.MainPropLibraryId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.OriBuffTempId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ChildBuffTempId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryPosInfo {
    pub IsCross: bool,
    pub Posotion: i32,
}

impl PbMessage for HonamiStoryPosInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsCross {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Posotion != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Posotion);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsCross = r.read_bool()?,
            2 => self.Posotion = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerInfoData {
    pub DangerLevel: i32,
    pub MaxFloor: i32,
}

impl PbMessage for TowerInfoData {
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
pub struct HonamiStoryBackpackEntry {
    pub Item: Option<HonamiStoryItemInfo>,
    pub State: Option<HonamiStoryPosInfo>,
}

impl PbMessage for HonamiStoryBackpackEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Item {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.State {
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
                self.Item
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.State
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBackpack {
    pub BackpackId: i32,
    pub Width: i32,
    pub Capacity: i32,
    pub Items: Vec<HonamiStoryBackpackEntry>,
}

impl PbMessage for HonamiStoryBackpack {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BackpackId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BackpackId);
        }
        if self.Width != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Width);
        }
        if self.Capacity != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Capacity);
        }
        for item in &self.Items {
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
            1 => self.BackpackId = r.read_varint_i32()?,
            2 => self.Width = r.read_varint_i32()?,
            3 => self.Capacity = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<HonamiStoryBackpackEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryPlayerBagInfo {
    pub Warehouse: Option<HonamiStoryBackpack>,
    pub EquipRack: Option<HonamiStoryBackpack>,
    pub RoleEquipList: Vec<HonamiStoryRoleData>,
    pub UnlockedWeaponIds: Vec<i32>,
}

impl PbMessage for HonamiStoryPlayerBagInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Warehouse {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.EquipRack {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.RoleEquipList {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.UnlockedWeaponIds {
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
                self.Warehouse
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.EquipRack
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEquipList
                    .push(<HonamiStoryRoleData as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedWeaponIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryRoleData {
    pub RoleId: i32,
    pub RoleSlots: Vec<HonamiStoryRoleSlot>,
    pub DressWeapon: i32,
}

impl PbMessage for HonamiStoryRoleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.RoleSlots {
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
                self.RoleSlots
                    .push(<HonamiStoryRoleSlot as PbMessage>::decode(bytes)?);
            }
            3 => self.DressWeapon = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryRoleSlot {
    pub SlotId: i32,
    pub IsUnlocked: bool,
}

impl PbMessage for HonamiStoryRoleSlot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.IsUnlocked {
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
            2 => self.IsUnlocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemCollectionConfig {
    pub Id: i32,
    pub Status: i32,
}

impl PbMessage for HonamiStoryItemCollectionConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryMascotConfig {
    pub MascotId: i32,
    pub State: i32,
}

impl PbMessage for HonamiStoryMascotConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MascotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MascotId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.MascotId = r.read_varint_i32()?,
            2 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryAreaConfig {
    pub AreaId: i32,
    pub Status: i32,
    pub SecreteStatus: i32,
}

impl PbMessage for HonamiStoryAreaConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
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
            1 => self.AreaId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            3 => self.SecreteStatus = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryScoreRewardInfo {
    pub ScoreRewardId: i32,
    pub Status: i32,
}

impl PbMessage for HonamiStoryScoreRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ScoreRewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScoreRewardId);
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
            1 => self.ScoreRewardId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TalentInfoData {
    pub TalentId: i32,
    pub State: i32,
}

impl PbMessage for TalentInfoData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.TalentId = r.read_varint_i32()?,
            2 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleCoopActivityData {
    pub CoopRoleInfos: Vec<CoopRoleInfo>,
    pub RewardGetList: Vec<i32>,
    pub CoopTaskCompleteInfos: Vec<CoopTaskCompleteInfo>,
    pub PreCompleteIds: Vec<i32>,
}

impl PbMessage for RoleCoopActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CoopRoleInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RewardGetList {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.CoopTaskCompleteInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.PreCompleteIds {
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
                let bytes = r.read_len_delimited()?;
                self.CoopRoleInfos
                    .push(<CoopRoleInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardGetList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CoopTaskCompleteInfos
                    .push(<CoopTaskCompleteInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreCompleteIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopTaskCompleteInfo {
    pub CoopTaskId: i32,
    pub Task: Option<ConditionTask>,
    pub UnLockTime: i64,
    pub LevelPlay1Done: bool,
    pub LevelPlay2Done: bool,
}

impl PbMessage for CoopTaskCompleteInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CoopTaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CoopTaskId);
        }
        if let Some(v) = &self.Task {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.UnLockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnLockTime);
        }
        if self.LevelPlay1Done {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelPlay2Done {
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
            1 => self.CoopTaskId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.UnLockTime = r.read_varint_i64()?,
            4 => self.LevelPlay1Done = r.read_bool()?,
            5 => self.LevelPlay2Done = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopRoleInfo {
    pub CoopRoleId: i32,
    pub RoleLevel: i32,
    pub RewardLevel: i32,
    pub FinishTime: i64,
}

impl PbMessage for CoopRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CoopRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CoopRoleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.RewardLevel != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardLevel);
        }
        if self.FinishTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.CoopRoleId = r.read_varint_i32()?,
            2 => self.RoleLevel = r.read_varint_i32()?,
            3 => self.RewardLevel = r.read_varint_i32()?,
            4 => self.FinishTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdvertisingPageInfo {
    pub ActivityId: i32,
    pub UnlockIndex: i32,
    pub RewardedIndex: i32,
}

impl PbMessage for AdvertisingPageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.UnlockIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockIndex);
        }
        if self.RewardedIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedIndex);
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
            2 => self.UnlockIndex = r.read_varint_i32()?,
            3 => self.RewardedIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorCycleIpActivityData {
    pub TaskDataList: Vec<ConditionTask>,
}

impl PbMessage for MotorCycleIpActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskDataList {
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
                self.TaskDataList
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardSkillUnlockInfo {
    pub CardId: i32,
    pub Unlock: bool,
    pub TargetNum: i32,
    pub CurNum: i32,
}

impl PbMessage for PhantomBattleCardSkillUnlockInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TargetNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetNum);
        }
        if self.CurNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.CardId = r.read_varint_i32()?,
            2 => self.Unlock = r.read_bool()?,
            3 => self.TargetNum = r.read_varint_i32()?,
            4 => self.CurNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourActivityInfo {
    pub MotorParkourLevelInfos: Vec<MotorParkourLevelInfo>,
}

impl PbMessage for MotorParkourActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorParkourLevelInfos {
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
                self.MotorParkourLevelInfos
                    .push(<MotorParkourLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourLevelInfo {
    pub MotorParkourId: i32,
    pub RewardStates: Vec<MotorParkourRewardState>,
    pub UnlockTime: i64,
    pub BestPassTime: i32,
}

impl PbMessage for MotorParkourLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorParkourId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorParkourId);
        }
        for item in &self.RewardStates {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.BestPassTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestPassTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MotorParkourId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardStates, |sr| {
                Ok(
                    <MotorParkourRewardState as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ),
                )
            })?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => self.BestPassTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookActivityInfo {
    pub ActivityTasks: Vec<ConditionTask>,
    pub MonsterGain: Vec<i32>,
    pub GetFullReward: bool,
    pub RoadBookLevel: i32,
    pub UnLockAreas: Vec<i32>,
    pub SoarLevels: Vec<RoadBookMotorcycleInfo>,
}

impl PbMessage for RoadBookActivityInfo {
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
        if self.RoadBookLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoadBookLevel);
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
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MonsterGain, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.GetFullReward = r.read_bool()?,
            4 => self.RoadBookLevel = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnLockAreas, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SoarLevels
                    .push(<RoadBookMotorcycleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookMotorcycleInfo {
    pub MotorcyclePlayId: i32,
    pub HistorySoarScore: i32,
    pub ReceiveIds: Vec<i32>,
}

impl PbMessage for RoadBookMotorcycleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorcyclePlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorcyclePlayId);
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
            1 => self.MotorcyclePlayId = r.read_varint_i32()?,
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
pub struct MotorFightActivityPb {
    pub MotorFightLevelPb: Vec<MotorFightLevelPb>,
    pub Task: Vec<ConditionTask>,
    pub TalentTree: Option<MotorFightTalentTreePb>,
    pub UnlockedItem: Vec<i32>,
    pub UnlockedRole: Vec<i32>,
}

impl PbMessage for MotorFightActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorFightLevelPb {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Task {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.TalentTree {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.UnlockedItem {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedRole {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MotorFightLevelPb
                    .push(<MotorFightLevelPb as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TalentTree
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedItem, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedRole, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightTalentTreePb {
    pub Talent: Vec<MotorFightTalentPb>,
}

impl PbMessage for MotorFightTalentTreePb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Talent {
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
                self.Talent
                    .push(<MotorFightTalentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightLevelPb {
    pub LevelId: i32,
    pub OpenTime: i64,
    pub Cleared: bool,
    pub BestScore: i32,
    pub LastRoleId: i32,
}

impl PbMessage for MotorFightLevelPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if self.Cleared {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BestScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestScore);
        }
        if self.LastRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRoleId);
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
            3 => self.Cleared = r.read_bool()?,
            4 => self.BestScore = r.read_varint_i32()?,
            5 => self.LastRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightTalentPb {
    pub Id: i32,
    pub Unlock: bool,
    pub InUse: bool,
}

impl PbMessage for MotorFightTalentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InUse {
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
            3 => self.Unlock = r.read_bool()?,
            4 => self.InUse = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EncircleChallengePb {
    pub ChallengeId: i32,
    pub OpenTime: i64,
    pub Pass: bool,
    pub MinStep: i32,
}

impl PbMessage for EncircleChallengePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if self.Pass {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MinStep != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinStep);
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
            2 => self.OpenTime = r.read_varint_i64()?,
            3 => self.Pass = r.read_bool()?,
            5 => self.MinStep = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EncircleActivityPb {
    pub Challenges: Vec<EncircleChallengePb>,
}

impl PbMessage for EncircleActivityPb {
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
                    .push(<EncircleChallengePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerClimbingActivityData {
    pub CycleId: i32,
    pub Records: Vec<NewTowerClimbingLevelRecord>,
    pub ScoreTasks: Vec<i32>,
    pub ActivityTasks: Vec<ActivityTask>,
    pub CycleBeginTime: i64,
    pub CycleCloseTime: i64,
    pub SeasonId: i32,
    pub SeasonBeginTime: i64,
    pub SeasonCloseTime: i64,
    pub SeasonTasks: Vec<ActivityTask>,
}

impl PbMessage for NewTowerClimbingActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        for item in &self.Records {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.ScoreTasks {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.CycleBeginTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CycleBeginTime);
        }
        if self.CycleCloseTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CycleCloseTime);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.SeasonBeginTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SeasonBeginTime);
        }
        if self.SeasonCloseTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SeasonCloseTime);
        }
        for item in &self.SeasonTasks {
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
            1 => self.CycleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Records
                    .push(<NewTowerClimbingLevelRecord as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTasks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            5 => self.CycleBeginTime = r.read_varint_i64()?,
            6 => self.CycleCloseTime = r.read_varint_i64()?,
            7 => self.SeasonId = r.read_varint_i32()?,
            8 => self.SeasonBeginTime = r.read_varint_i64()?,
            9 => self.SeasonCloseTime = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SeasonTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerClimbingLevelRecord {
    pub LevelId: i32,
    pub WaveConfigIds: Vec<i32>,
    pub NextMonsterInfoPreview: Option<MonsterInfoPreview>,
    pub TeamChallengeInfos: Vec<TeamChallengeInfo>,
    pub Score: i32,
    pub IsUnlock: bool,
    pub RoleEnergyDict: BTreeMap<i32, i32>,
    pub HistoryScore: i32,
}

impl PbMessage for NewTowerClimbingLevelRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.WaveConfigIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.NextMonsterInfoPreview {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.TeamChallengeInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.RoleEnergyDict.is_empty() {
            crate::wire::write_map(out, 9, &self.RoleEnergyDict);
        }
        if self.HistoryScore != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryScore);
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.WaveConfigIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.NextMonsterInfoPreview
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TeamChallengeInfos
                    .push(<TeamChallengeInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.Score = r.read_varint_i32()?,
            8 => self.IsUnlock = r.read_bool()?,
            9 => crate::wire::decode_map_entry(r, &mut self.RoleEnergyDict)?,
            10 => self.HistoryScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterInfoPreview {
    pub WaveConfigId: i32,
    pub HpPpb: i32,
    pub Damage: i64,
    pub Round: i32,
    pub IsDead: bool,
}

impl PbMessage for MonsterInfoPreview {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveConfigId);
        }
        if self.HpPpb != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HpPpb);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Damage);
        }
        if self.Round != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Round);
        }
        if self.IsDead {
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
            1 => self.WaveConfigId = r.read_varint_i32()?,
            2 => self.HpPpb = r.read_varint_i32()?,
            3 => self.Damage = r.read_varint_i64()?,
            4 => self.Round = r.read_varint_i32()?,
            5 => self.IsDead = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSaveInfo {
    pub RoleId: i32,
    pub WeaponIncId: i32,
    pub PhantomIncId: Vec<i32>,
    pub SkillBranchId: i32,
}

impl PbMessage for RoleSaveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.WeaponIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponIncId);
        }
        for item in &self.PhantomIncId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            2 => self.WeaponIncId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PhantomIncId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamChallengeInfo {
    pub RoleSaveInfos: Vec<RoleSaveInfo>,
    pub BuffIds: Vec<i32>,
    pub LastMonsterInfoPreview: Option<MonsterInfoPreview>,
    pub TeamScore: i32,
}

impl PbMessage for TeamChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleSaveInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.LastMonsterInfoPreview {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.TeamScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeamScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RoleSaveInfos
                    .push(<RoleSaveInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LastMonsterInfoPreview
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.TeamScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportActivityV2Pb {
    pub TrialRoleInfoList: Vec<NewTrialRoleInfo>,
    pub CurUseTrialRoleId: i32,
    pub CurUseRoleInfo: Option<RoleInfo>,
    pub DeduplicateGachaRoleIds: Vec<i32>,
    pub NbWeekCardEndShowTime: i64,
    pub NbGiftPackEndShowTime: i64,
    pub NbGachaEndShowTime: i64,
    pub NbLivenessEndShowTime: i64,
    pub NewPlayerPoolFinalGachaRoleId: i32,
}

impl PbMessage for NewPlayerSupportActivityV2Pb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrialRoleInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CurUseTrialRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurUseTrialRoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.DeduplicateGachaRoleIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.NbWeekCardEndShowTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NbWeekCardEndShowTime);
        }
        if self.NbGiftPackEndShowTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NbGiftPackEndShowTime);
        }
        if self.NbGachaEndShowTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NbGachaEndShowTime);
        }
        if self.NbLivenessEndShowTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NbLivenessEndShowTime);
        }
        if self.NewPlayerPoolFinalGachaRoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewPlayerPoolFinalGachaRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TrialRoleInfoList
                    .push(<NewTrialRoleInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.CurUseTrialRoleId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DeduplicateGachaRoleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => self.NbWeekCardEndShowTime = r.read_varint_i64()?,
            7 => self.NbGiftPackEndShowTime = r.read_varint_i64()?,
            8 => self.NbGachaEndShowTime = r.read_varint_i64()?,
            9 => self.NbLivenessEndShowTime = r.read_varint_i64()?,
            10 => self.NewPlayerPoolFinalGachaRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalActivityInfo {
    pub AreaInfos: Vec<AreaInfo>,
    pub UnlockFurnitures: Vec<i32>,
    pub DrinkMixData: Option<DrinkMixData>,
    pub OneBrochureInfos: Vec<OneBrochureInfo>,
    pub JokerLevelInfos: Vec<GuessJokerLevelInfo>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub SpringFunctionIds: Vec<i32>,
    pub RewardScoreIds: Vec<i32>,
    pub RewardLevelIds: Vec<i32>,
    pub Atmosphere: i32,
    pub AtmosphereLevel: i32,
    pub SpringSkipEntries: Vec<SpringSkipEntry>,
}

impl PbMessage for SpringFestivalActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.UnlockFurnitures {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.DrinkMixData {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.OneBrochureInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.JokerLevelInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.SpringFunctionIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardScoreIds {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardLevelIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Atmosphere != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Atmosphere);
        }
        if self.AtmosphereLevel != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AtmosphereLevel);
        }
        for item in &self.SpringSkipEntries {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.AreaInfos.push(<AreaInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockFurnitures,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DrinkMixData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.OneBrochureInfos
                    .push(<OneBrochureInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.JokerLevelInfos
                    .push(<GuessJokerLevelInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SpringFunctionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            8 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardScoreIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardLevelIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            10 => self.Atmosphere = r.read_varint_i32()?,
            11 => self.AtmosphereLevel = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SpringSkipEntries
                    .push(<SpringSkipEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringSkipEntry {
    pub Id: i32,
    pub UnLock: bool,
    pub Finish: bool,
}

impl PbMessage for SpringSkipEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.UnLock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Finish {
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
            2 => self.UnLock = r.read_bool()?,
            3 => self.Finish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrinkMixData {
    pub RoleLevelInfo: Vec<DrinkMixRole>,
}

impl PbMessage for DrinkMixData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleLevelInfo {
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
                self.RoleLevelInfo
                    .push(<DrinkMixRole as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrinkMixRole {
    pub RoleId: i32,
    pub FirstPass: bool,
    pub MaxLike: bool,
    pub RewardGet: bool,
}

impl PbMessage for DrinkMixRole {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.FirstPass {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxLike {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardGet {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.FirstPass = r.read_bool()?,
            3 => self.MaxLike = r.read_bool()?,
            4 => self.RewardGet = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuessJokerLevelInfo {
    pub LevelId: i32,
    pub LevelPass: bool,
    pub UnLock: bool,
    pub RewardGet: bool,
    pub PlayerWin: bool,
}

impl PbMessage for GuessJokerLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.LevelPass {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnLock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardGet {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerWin {
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
            2 => self.LevelPass = r.read_bool()?,
            3 => self.UnLock = r.read_bool()?,
            4 => self.RewardGet = r.read_bool()?,
            5 => self.PlayerWin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaInfo {
    pub AreaId: i32,
    pub Atmosphere: i32,
    pub FurnitureDiySlotInfos: Vec<FurnitureDiySlotInfo>,
}

impl PbMessage for AreaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        if self.Atmosphere != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Atmosphere);
        }
        for item in &self.FurnitureDiySlotInfos {
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
            1 => self.AreaId = r.read_varint_i32()?,
            2 => self.Atmosphere = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FurnitureDiySlotInfos
                    .push(<FurnitureDiySlotInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FurnitureDiySlotInfo {
    pub SlotEntityCfgId: i32,
    pub RootFurnitureId: i32,
    pub SubFurnitureIds: Vec<i32>,
}

impl PbMessage for FurnitureDiySlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotEntityCfgId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotEntityCfgId);
        }
        if self.RootFurnitureId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RootFurnitureId);
        }
        for item in &self.SubFurnitureIds {
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
            1 => self.SlotEntityCfgId = r.read_varint_i32()?,
            2 => self.RootFurnitureId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SubFurnitureIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneBrochureInfo {
    pub BrochureId: i32,
    pub BookItemInfos: Vec<BookItemInfo>,
}

impl PbMessage for OneBrochureInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BrochureId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BrochureId);
        }
        for item in &self.BookItemInfos {
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
            1 => self.BrochureId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BookItemInfos
                    .push(<BookItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BookItemInfo {
    pub BookItemId: i32,
    pub BookItemState: Option<BookItemState>,
}

impl PbMessage for BookItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BookItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BookItemId);
        }
        if let Some(v) = &self.BookItemState {
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
            1 => self.BookItemId = r.read_varint_i32()?,
            2 => {
                self.BookItemState =
                    Some(<BookItemState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpRewardInfo {
    pub Id: i32,
    pub Score: i32,
    pub RewardContent: BTreeMap<i32, i32>,
    pub Status: i32,
}

impl PbMessage for TotalTopUpRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if !self.RewardContent.is_empty() {
            crate::wire::write_map(out, 3, &self.RewardContent);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.RewardContent)?,
            4 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpActivityInfo {
    pub Score: i32,
    pub TotalTopUpRewardInfos: Vec<TotalTopUpRewardInfo>,
}

impl PbMessage for TotalTopUpActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Score != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        for item in &self.TotalTopUpRewardInfos {
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
            1 => self.Score = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TotalTopUpRewardInfos
                    .push(<TotalTopUpRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct H5ViewActivityData {
    pub RedDot: bool,
    pub AllRewardClaimed: bool,
}

impl PbMessage for H5ViewActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RedDot {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.AllRewardClaimed {
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
            1 => self.RedDot = r.read_bool()?,
            2 => self.AllRewardClaimed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkinRewardActivityData {
    pub RewardInfos: Vec<SkinRewardActivityRewardInfo>,
}

impl PbMessage for SkinRewardActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardInfos {
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
                self.RewardInfos
                    .push(<SkinRewardActivityRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkinRewardActivityRewardInfo {
    pub ConfigId: i32,
    pub State: Option<SkinRewardActivityRewardState>,
}

impl PbMessage for SkinRewardActivityRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <SkinRewardActivityRewardState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct MotorDevelopActivityData {
    pub Task: Vec<ConditionTask>,
}

impl PbMessage for MotorDevelopActivityData {
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
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeLevelInfo {
    pub Id: i32,
    pub UnlockTime: i64,
    pub State: i32,
}

impl PbMessage for FlagChallengeLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagStrongholdInfo {
    pub Id: i32,
    pub IsPass: bool,
}

impl PbMessage for FlagStrongholdInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsPass {
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
            2 => self.IsPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeRoleLevelInfo {
    pub PerLevel: i32,
    pub PerExp: i32,
}

impl PbMessage for FlagChallengeRoleLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PerLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PerLevel);
        }
        if self.PerExp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PerExp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PerLevel = r.read_varint_i32()?,
            2 => self.PerExp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeiXuePreheatInfo {
    pub Id: i32,
    pub State: i32,
    pub QuestUnlockTime: i64,
}

impl PbMessage for FeiXuePreheatInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if self.QuestUnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.QuestUnlockTime);
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
            2 => self.State = r.read_varint_i32()?,
            3 => self.QuestUnlockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeiXuePreheatActivityInfo {
    pub FeiXuePreheatInfos: Vec<FeiXuePreheatInfo>,
}

impl PbMessage for FeiXuePreheatActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FeiXuePreheatInfos {
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
                self.FeiXuePreheatInfos
                    .push(<FeiXuePreheatInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmActivityPb {
    pub RhythmShipPlanetPb: Vec<RhythmShipPlanetPb>,
    pub RhythmRoleId: i32,
    pub RhythmTask: Vec<RhythmTaskPb>,
    pub UnlockedRole: Vec<i32>,
    pub RedDot: Option<RhythmRedDotPb>,
}

impl PbMessage for RhythmActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RhythmShipPlanetPb {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.RhythmRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RhythmRoleId);
        }
        for item in &self.RhythmTask {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.UnlockedRole {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.RedDot {
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
                let bytes = r.read_len_delimited()?;
                self.RhythmShipPlanetPb
                    .push(<RhythmShipPlanetPb as PbMessage>::decode(bytes)?);
            }
            3 => self.RhythmRoleId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RhythmTask
                    .push(<RhythmTaskPb as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedRole, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RedDot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmShipPlanetPb {
    pub PlanetId: i32,
    pub OpenTime: i64,
    pub RhythmShipLevelPb: Vec<RhythmShipLevelPb>,
}

impl PbMessage for RhythmShipPlanetPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlanetId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlanetId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        for item in &self.RhythmShipLevelPb {
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
            1 => self.PlanetId = r.read_varint_i32()?,
            2 => self.OpenTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RhythmShipLevelPb
                    .push(<RhythmShipLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmShipLevelPb {
    pub LevelId: i32,
    pub RhythmSubLevelPb: Vec<RhythmSubLevelPb>,
}

impl PbMessage for RhythmShipLevelPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.RhythmSubLevelPb {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RhythmSubLevelPb
                    .push(<RhythmSubLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSubLevelPb {
    pub SubLevelId: i32,
    pub Cleared: bool,
    pub BestScore: i32,
    pub BestAccuracy: i32,
    pub BestRank: Option<RhythmShipRank>,
}

impl PbMessage for RhythmSubLevelPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelId);
        }
        if self.Cleared {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BestScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestScore);
        }
        if self.BestAccuracy != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestAccuracy);
        }
        if let Some(v) = &self.BestRank {
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
            1 => self.SubLevelId = r.read_varint_i32()?,
            2 => self.Cleared = r.read_bool()?,
            3 => self.BestScore = r.read_varint_i32()?,
            4 => self.BestAccuracy = r.read_varint_i32()?,
            5 => {
                self.BestRank = Some(
                    <RhythmShipRank as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RhythmTaskPb {
    pub TaskType: Option<RhythmTaskTypePb>,
    pub Task: Vec<ConditionTask>,
}

impl PbMessage for RhythmTaskPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TaskType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Task {
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
                self.TaskType = Some(
                    <RhythmTaskTypePb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRedDotPb {
    pub ReadPlanet: Vec<i32>,
    pub ReadSubLevel: Vec<i32>,
    pub ReadRole: Vec<i32>,
}

impl PbMessage for RhythmRedDotPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReadPlanet {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ReadSubLevel {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ReadRole {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadPlanet, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadSubLevel, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadRole, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchActivityInfo {
    pub DropCatchLevelInfos: Vec<DropCatchLevelInfo>,
}

impl PbMessage for DropCatchActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DropCatchLevelInfos {
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
                self.DropCatchLevelInfos
                    .push(<DropCatchLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchLevelInfo {
    pub DropCatchId: i32,
    pub RewardStates: Vec<i32>,
    pub UnlockTime: i64,
    pub Score: i32,
}

impl PbMessage for DropCatchLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropCatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropCatchId);
        }
        for item in &self.RewardStates {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
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
            1 => self.DropCatchId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardStates, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TetrisActivityInfo {
    pub TetrisLevelInfos: Vec<TetrisLevelInfo>,
}

impl PbMessage for TetrisActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TetrisLevelInfos {
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
                self.TetrisLevelInfos
                    .push(<TetrisLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TetrisLevelInfoVdCOneof {
    DifficultyIdx(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TetrisLevelInfoEhCOneof {
    State(TetrisState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TetrisLevelInfoThCOneof {
    UnlockTime(i64),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TetrisLevelInfo {
    pub vdC: Option<TetrisLevelInfoVdCOneof>,
    pub ehC: Option<TetrisLevelInfoEhCOneof>,
    pub thC: Option<TetrisLevelInfoThCOneof>,
    pub Id: i32,
    pub Results: BTreeMap<i32, i32>,
}

impl PbMessage for TetrisLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.vdC {
            match oneof {
                TetrisLevelInfoVdCOneof::DifficultyIdx(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.ehC {
            match oneof {
                TetrisLevelInfoEhCOneof::State(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if let Some(oneof) = &self.thC {
            match oneof {
                TetrisLevelInfoThCOneof::UnlockTime(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.Results.is_empty() {
            crate::wire::write_map(out, 5, &self.Results);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.vdC = Some(TetrisLevelInfoVdCOneof::DifficultyIdx(r.read_varint_i32()?)),
            3 => {
                self.ehC = Some(TetrisLevelInfoEhCOneof::State(
                    <TetrisState as crate::wire::PbEnum>::from_i32_or_default(r.read_varint_i32()?),
                ))
            }
            4 => self.thC = Some(TetrisLevelInfoThCOneof::UnlockTime(r.read_varint_i64()?)),
            1 => self.Id = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.Results)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballActivityData {
    pub Chapters: Vec<PinballChapterData>,
    pub Levels: Vec<PinballLevelData>,
    pub Weapons: Option<PinballWeapons>,
    pub Roles: Option<PinballRoles>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub GroupFormations: Vec<PinballGroupFormation>,
}

impl PbMessage for PinballActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Chapters {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.Levels {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.Weapons {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Roles {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.GroupFormations {
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
                self.Chapters
                    .push(<PinballChapterData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Levels
                    .push(<PinballLevelData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Weapons
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.GroupFormations
                    .push(<PinballGroupFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballGroupFormation {
    pub LevelGroup: i32,
    pub RoleIds: Vec<i32>,
}

impl PbMessage for PinballGroupFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelGroup != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGroup);
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
            1 => self.LevelGroup = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballChapterData {
    pub ChapterId: i32,
    pub UnLockTime: i64,
}

impl PbMessage for PinballChapterData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterId);
        }
        if self.UnLockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnLockTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ChapterId = r.read_varint_i32()?,
            2 => self.UnLockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PinballLevelDataDataOneof {
    NormalLevel(NormalLevel),
    CowLevel(CowLevel),
    TowerLevel(TowerLevel),
    DailyLevel(DailyLevel),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballLevelData {
    pub data: Option<PinballLevelDataDataOneof>,
    pub ConfigId: i32,
}

impl PbMessage for PinballLevelData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.data {
            match oneof {
                PinballLevelDataDataOneof::NormalLevel(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                PinballLevelDataDataOneof::CowLevel(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                PinballLevelDataDataOneof::TowerLevel(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                PinballLevelDataDataOneof::DailyLevel(v) => {
                    crate::wire::write_submessage(out, 5, v);
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
                let val = <NormalLevel as PbMessage>::decode(bytes)?;
                self.data = Some(PinballLevelDataDataOneof::NormalLevel(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <CowLevel as PbMessage>::decode(bytes)?;
                self.data = Some(PinballLevelDataDataOneof::CowLevel(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <TowerLevel as PbMessage>::decode(bytes)?;
                self.data = Some(PinballLevelDataDataOneof::TowerLevel(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <DailyLevel as PbMessage>::decode(bytes)?;
                self.data = Some(PinballLevelDataDataOneof::DailyLevel(val));
            }
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalLevel {
    pub StarByte: i32,
}

impl PbMessage for NormalLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StarByte != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StarByte);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StarByte = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CowLevel {
    pub LevelScore: i32,
}

impl PbMessage for CowLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LevelScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerLevel {
    pub StarByte: i32,
    pub CostTime: i32,
}

impl PbMessage for TowerLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StarByte != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StarByte);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StarByte = r.read_varint_i32()?,
            2 => self.CostTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyLevel {
    pub RandomLevelId: i32,
    pub reward: bool,
}

impl PbMessage for DailyLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RandomLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RandomLevelId);
        }
        if self.reward {
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
            1 => self.RandomLevelId = r.read_varint_i32()?,
            2 => self.reward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballWeapon {
    pub ConfigId: i32,
    pub IncrId: i32,
    pub FuncValue: i32,
    pub roleId: i32,
    pub SubEntryId: i32,
}

impl PbMessage for PinballWeapon {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.FuncValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncValue);
        }
        if self.roleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.roleId);
        }
        if self.SubEntryId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubEntryId);
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
            2 => self.IncrId = r.read_varint_i32()?,
            3 => self.FuncValue = r.read_varint_i32()?,
            4 => self.roleId = r.read_varint_i32()?,
            5 => self.SubEntryId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballWeapons {
    pub PinballWeaponList: Vec<PinballWeapon>,
}

impl PbMessage for PinballWeapons {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PinballWeaponList {
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
                self.PinballWeaponList
                    .push(<PinballWeapon as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballRoles {
    pub Roles: Vec<PinballRoleData>,
}

impl PbMessage for PinballRoles {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
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
                self.Roles
                    .push(<PinballRoleData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballRoleData {
    pub ConfigId: i32,
    pub RoleLevel: i32,
}

impl PbMessage for PinballRoleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.RoleLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingLevelInfo {
    pub id: i32,
    pub UnlockTime: i64,
    pub BossHpNum: i32,
    pub SelectRoleIds: Vec<i32>,
    pub SkillBranchId: Vec<i32>,
}

impl PbMessage for BossPilingLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.id);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.BossHpNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossHpNum);
        }
        for item in &self.SelectRoleIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkillBranchId {
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
            1 => self.id = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.BossHpNum = r.read_varint_i32()?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectRoleIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => {
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
pub struct ThemeCelebration {
    pub PersonalRewardIds: Vec<i32>,
    pub WorldRewardIds: Vec<i32>,
    pub SubActivityTimes: Vec<SubActivityBeginTime>,
}

impl PbMessage for ThemeCelebration {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PersonalRewardIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.WorldRewardIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SubActivityTimes {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PersonalRewardIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.WorldRewardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SubActivityTimes
                    .push(<SubActivityBeginTime as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuWeekActivity {
    pub ConditionTasks: Vec<ConditionTask>,
    pub TaskPack: Vec<WuWuTaskPack>,
}

impl PbMessage for WuWuWeekActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.TaskPack {
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
                self.TaskPack
                    .push(<WuWuTaskPack as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuTaskPack {
    pub WuWuPackageId: i32,
    pub UnLockTime: i64,
    pub HadReward: bool,
}

impl PbMessage for WuWuTaskPack {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WuWuPackageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WuWuPackageId);
        }
        if self.UnLockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnLockTime);
        }
        if self.HadReward {
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
            1 => self.WuWuPackageId = r.read_varint_i32()?,
            2 => self.UnLockTime = r.read_varint_i64()?,
            3 => self.HadReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubActivityBeginTime {
    pub Id: i32,
    pub BeginTime: i64,
}

impl PbMessage for SubActivityBeginTime {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDecalActivityData {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl PbMessage for MotorDecalActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LinkageCheckInActivityData {
    pub CheckInDay: i32,
    pub NormalReward: Vec<i32>,
    pub KeepReward: Vec<i32>,
}

impl PbMessage for LinkageCheckInActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CheckInDay != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CheckInDay);
        }
        for item in &self.NormalReward {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.KeepReward {
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
            1 => self.CheckInDay = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NormalReward, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.KeepReward, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GolemCrackActivityInfo {
    pub GolemCrackLevelInfos: Vec<GolemCrackLevelInfo>,
}

impl PbMessage for GolemCrackActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GolemCrackLevelInfos {
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
                self.GolemCrackLevelInfos
                    .push(<GolemCrackLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GolemCrackLevelInfoNlCOneof {
    state(GolemCrackState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GolemCrackLevelInfoVlCOneof {
    UnlockTime(i64),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GolemCrackLevelInfo {
    pub NlC: Option<GolemCrackLevelInfoNlCOneof>,
    pub VlC: Option<GolemCrackLevelInfoVlCOneof>,
    pub id: i32,
}

impl PbMessage for GolemCrackLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.NlC {
            match oneof {
                GolemCrackLevelInfoNlCOneof::state(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if let Some(oneof) = &self.VlC {
            match oneof {
                GolemCrackLevelInfoVlCOneof::UnlockTime(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if self.id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            2 => {
                self.NlC = Some(GolemCrackLevelInfoNlCOneof::state(
                    <GolemCrackState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            3 => {
                self.VlC = Some(GolemCrackLevelInfoVlCOneof::UnlockTime(
                    r.read_varint_i64()?,
                ))
            }
            1 => self.id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerActivityInfo {
    pub EdgeRunnerFunctionIds: Vec<i32>,
    pub RewardScoreId: i32,
    pub EdgeRunnerLordGymPassRecords: Vec<EdgeRunnerLordGymPassRecord>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub PreUnlockIds: Vec<i32>,
}

impl PbMessage for EdgeRunnerActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EdgeRunnerFunctionIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RewardScoreId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardScoreId);
        }
        for item in &self.EdgeRunnerLordGymPassRecords {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.PreUnlockIds {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EdgeRunnerFunctionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => self.RewardScoreId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.EdgeRunnerLordGymPassRecords
                    .push(<EdgeRunnerLordGymPassRecord as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreUnlockIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerLordGymPassRecord {
    pub LoadGymId: i32,
    pub PassTime: i32,
}

impl PbMessage for EdgeRunnerLordGymPassRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadGymId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadGymId);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.LoadGymId = r.read_varint_i32()?,
            2 => self.PassTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymInfoRequest {}

impl LordGymInfoRequest {
    pub const MSG_ID: u16 = 24178;
}

impl PbMessage for LordGymInfoRequest {
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
pub struct LordGymInfoResponse {
    pub UnlockLoadGymIds: Vec<i32>,
    pub ReadLoadGymIds: Vec<i32>,
    pub LordGymPassRecords: Vec<LordGymPassRecord>,
    pub LordGymEntranceInfos: Vec<LordGymEntranceInfo>,
    pub LordGymGroupInfos: Vec<LordGymGroupInfo>,
}

impl LordGymInfoResponse {
    pub const MSG_ID: u16 = 24508;
}

impl PbMessage for LordGymInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockLoadGymIds {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ReadLoadGymIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LordGymPassRecords {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.LordGymEntranceInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.LordGymGroupInfos {
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
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockLoadGymIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadLoadGymIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.LordGymPassRecords
                    .push(<LordGymPassRecord as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.LordGymEntranceInfos
                    .push(<LordGymEntranceInfo as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.LordGymGroupInfos
                    .push(<LordGymGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymEntranceInfo {
    pub ConfigId: i32,
    pub EffectBeginTime: i64,
    pub EffectEndTime: i64,
}

impl PbMessage for LordGymEntranceInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.EffectBeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EffectBeginTime);
        }
        if self.EffectEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EffectEndTime);
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
            2 => self.EffectBeginTime = r.read_varint_i64()?,
            3 => self.EffectEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymGroupInfo {
    pub GroupId: i32,
    pub PassDiff: i32,
}

impl PbMessage for LordGymGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.PassDiff != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassDiff);
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
            2 => self.PassDiff = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleBrief {
    pub RoleId: i32,
    pub Level: i32,
}

impl PbMessage for RoleBrief {
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
pub struct LordGymPassRecord {
    pub LoadGymId: i32,
    pub PassTime: i32,
    pub RoleIds: Vec<RoleBrief>,
}

impl PbMessage for LordGymPassRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadGymId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadGymId);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        for item in &self.RoleIds {
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
            1 => self.LoadGymId = r.read_varint_i32()?,
            2 => self.PassTime = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleIds.push(<RoleBrief as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleGiftActivityData {
    pub RewardHadGet: bool,
}

impl PbMessage for RoleGiftActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardHadGet {
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
            1 => self.RewardHadGet = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoItemPanelPbData {
    pub ItemPbDatas: Vec<KurotatoItemPbData>,
}

impl PbMessage for KurotatoItemPanelPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemPbDatas {
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
                self.ItemPbDatas
                    .push(<KurotatoItemPbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoItemPbData {
    pub itemId: i32,
    pub count: i32,
    pub PreWaveDealtDamage: i32,
}

impl PbMessage for KurotatoItemPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.itemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.itemId);
        }
        if self.count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.count);
        }
        if self.PreWaveDealtDamage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreWaveDealtDamage);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.itemId = r.read_varint_i32()?,
            2 => self.count = r.read_varint_i32()?,
            3 => self.PreWaveDealtDamage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoWeaponPanelPbData {
    pub WeaponPbDatas: Vec<KurotatoWeaponPbData>,
}

impl PbMessage for KurotatoWeaponPanelPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponPbDatas {
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
                self.WeaponPbDatas
                    .push(<KurotatoWeaponPbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoWeaponPbData {
    pub WeaponId: i32,
    pub IncId: i32,
    pub PreWaveDealtDamage: i32,
    pub SellPrice: i32,
}

impl PbMessage for KurotatoWeaponPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.PreWaveDealtDamage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreWaveDealtDamage);
        }
        if self.SellPrice != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SellPrice);
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
            2 => self.IncId = r.read_varint_i32()?,
            3 => self.PreWaveDealtDamage = r.read_varint_i32()?,
            4 => self.SellPrice = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoActivityData {
    pub KurotatoLevelInfos: Vec<KurotatoLevelInfo>,
    pub KurotatoRoleInfos: Vec<KurotatoRoleInfo>,
    pub UnlockWeapons: Vec<i32>,
    pub UnlockItems: Vec<i32>,
    pub ScoreTasks: Vec<i32>,
    pub ResTasks: Vec<ConditionTask>,
    pub LimitTasks: Vec<ConditionTask>,
}

impl PbMessage for KurotatoActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.KurotatoLevelInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.KurotatoRoleInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnlockWeapons {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockItems {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ScoreTasks {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ResTasks {
            crate::wire::write_submessage(out, 6, item);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoLevelInfos
                    .push(<KurotatoLevelInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoRoleInfos
                    .push(<KurotatoRoleInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockWeapons, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTasks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ResTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
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

#[derive(Debug, Clone, PartialEq)]
pub enum KurotatoLevelInfoJSpOneof {
    EndlessLevelInfo(KurotatoEndlessLevelInfo),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KurotatoLevelInfoDOLLARSpOneof {
    InstData(KurotatoInstInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoLevelInfo {
    pub jSp: Option<KurotatoLevelInfoJSpOneof>,
    pub DOLLARSp: Option<KurotatoLevelInfoDOLLARSpOneof>,
    pub LevelId: i32,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub IsFinished: bool,
}

impl PbMessage for KurotatoLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.jSp {
            match oneof {
                KurotatoLevelInfoJSpOneof::EndlessLevelInfo(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
            }
        }
        if let Some(oneof) = &self.DOLLARSp {
            match oneof {
                KurotatoLevelInfoDOLLARSpOneof::InstData(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
            }
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsFinished {
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
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoEndlessLevelInfo as PbMessage>::decode(bytes)?;
                self.jSp = Some(KurotatoLevelInfoJSpOneof::EndlessLevelInfo(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoInstInfo as PbMessage>::decode(bytes)?;
                self.DOLLARSp = Some(KurotatoLevelInfoDOLLARSpOneof::InstData(val));
            }
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => self.IsFinished = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoEndlessLevelInfo {
    pub FinishWave: i32,
    pub TotalKillCount: i32,
    pub PassRoleIds: Vec<i32>,
}

impl PbMessage for KurotatoEndlessLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FinishWave != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinishWave);
        }
        if self.TotalKillCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalKillCount);
        }
        for item in &self.PassRoleIds {
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
            1 => self.FinishWave = r.read_varint_i32()?,
            2 => self.TotalKillCount = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PassRoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KurotatoRoleInfoDOLLARSpOneof {
    InstData(KurotatoInstInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoRoleInfo {
    pub DOLLARSp: Option<KurotatoRoleInfoDOLLARSpOneof>,
    pub roleId: i32,
    pub IsUnlock: bool,
    pub MaxFinishWave: i32,
    pub KillCount: i32,
}

impl PbMessage for KurotatoRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.DOLLARSp {
            match oneof {
                KurotatoRoleInfoDOLLARSpOneof::InstData(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.roleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.roleId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxFinishWave != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFinishWave);
        }
        if self.KillCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillCount);
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
                let val = <KurotatoInstInfo as PbMessage>::decode(bytes)?;
                self.DOLLARSp = Some(KurotatoRoleInfoDOLLARSpOneof::InstData(val));
            }
            1 => self.roleId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.MaxFinishWave = r.read_varint_i32()?,
            5 => self.KillCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoInstInfo {
    pub LevelId: i32,
    pub CurWave: i32,
    pub roleId: i32,
    pub RoleLevel: i32,
    pub RoleExp: i32,
    pub ItemPanelPbData: Option<KurotatoItemPanelPbData>,
    pub WeaponPanelPbData: Option<KurotatoWeaponPanelPbData>,
    pub PropertyMap: BTreeMap<i32, i32>,
    pub SaveTimestamp: i64,
    pub WaveType: i32,
}

impl PbMessage for KurotatoInstInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.CurWave != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWave);
        }
        if self.roleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.roleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.RoleExp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleExp);
        }
        if let Some(v) = &self.ItemPanelPbData {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.WeaponPanelPbData {
            crate::wire::write_submessage(out, 7, v);
        }
        if !self.PropertyMap.is_empty() {
            crate::wire::write_map(out, 8, &self.PropertyMap);
        }
        if self.SaveTimestamp != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SaveTimestamp);
        }
        if self.WaveType != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveType);
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
            2 => self.CurWave = r.read_varint_i32()?,
            3 => self.roleId = r.read_varint_i32()?,
            4 => self.RoleLevel = r.read_varint_i32()?,
            5 => self.RoleExp = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ItemPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => crate::wire::decode_map_entry(r, &mut self.PropertyMap)?,
            9 => self.SaveTimestamp = r.read_varint_i64()?,
            10 => self.WaveType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorLevelInfo {
    pub LevelId: i32,
    pub Ranking: i32,
    pub TimeCost: i32,
}

impl PbMessage for OnlineMotorLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.Ranking != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ranking);
        }
        if self.TimeCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            2 => self.Ranking = r.read_varint_i32()?,
            3 => self.TimeCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorLevelUnLockTime {
    pub LevelId: i32,
    pub UnLockTime: i64,
}

impl PbMessage for OnlineMotorLevelUnLockTime {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.UnLockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnLockTime);
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
            2 => self.UnLockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorActivityData {
    pub OnlineMotorLevelInfos: Vec<OnlineMotorLevelInfo>,
    pub LevelTasks: Vec<OnlineMotorTask>,
    pub GlobalTasks: Vec<OnlineMotorTask>,
    pub UnLocks: Vec<OnlineMotorLevelUnLockTime>,
}

impl PbMessage for OnlineMotorActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OnlineMotorLevelInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.LevelTasks {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.GlobalTasks {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.UnLocks {
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
                self.OnlineMotorLevelInfos
                    .push(<OnlineMotorLevelInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.LevelTasks
                    .push(<OnlineMotorTask as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.GlobalTasks
                    .push(<OnlineMotorTask as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.UnLocks
                    .push(<OnlineMotorLevelUnLockTime as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorTask {
    pub taskId: i32,
    pub state: Option<ActivityTaskState>,
    pub PlayCount: i32,
    pub Champion: i32,
    pub SpeedLap: i32,
    pub FirstRunner: i32,
}

impl PbMessage for OnlineMotorTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.taskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.taskId);
        }
        if let Some(v) = &self.state {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayCount);
        }
        if self.Champion != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Champion);
        }
        if self.SpeedLap != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedLap);
        }
        if self.FirstRunner != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstRunner);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.taskId = r.read_varint_i32()?,
            2 => {
                self.state = Some(
                    <ActivityTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.PlayCount = r.read_varint_i32()?,
            4 => self.Champion = r.read_varint_i32()?,
            5 => self.SpeedLap = r.read_varint_i32()?,
            6 => self.FirstRunner = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenActivityInfo {
    pub ActivityTasks: Vec<ConditionTask>,
    pub MonsterGain: Vec<i32>,
    pub GetFullReward: bool,
    pub RealmBetweenLevel: i32,
    pub UnLockAreas: Vec<i32>,
    pub SoarLevels: Vec<RealmBetweenMotorcycleInfo>,
}

impl PbMessage for RealmBetweenActivityInfo {
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
        if self.RealmBetweenLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RealmBetweenLevel);
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
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MonsterGain, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.GetFullReward = r.read_bool()?,
            4 => self.RealmBetweenLevel = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnLockAreas, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SoarLevels
                    .push(<RealmBetweenMotorcycleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenMotorcycleInfo {
    pub MotorcyclePlayId: i32,
    pub HistorySoarScore: i32,
    pub ReceiveIds: Vec<i32>,
}

impl PbMessage for RealmBetweenMotorcycleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorcyclePlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorcyclePlayId);
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
            1 => self.MotorcyclePlayId = r.read_varint_i32()?,
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
pub struct NewbieMainActivityPb {
    pub NewbieMainTabs: Vec<NewbieMainTabPb>,
    pub TakenScoreRewardIds: Vec<i32>,
    pub ProgressScore: i32,
}

impl PbMessage for NewbieMainActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NewbieMainTabs {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.TakenScoreRewardIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ProgressScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProgressScore);
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
                self.NewbieMainTabs
                    .push(<NewbieMainTabPb as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TakenScoreRewardIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => self.ProgressScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieMainTabPb {
    pub TabId: i32,
    pub CompletedTaskIds: Vec<i32>,
}

impl PbMessage for NewbieMainTabPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TabId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabId);
        }
        for item in &self.CompletedTaskIds {
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
            1 => self.TabId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompletedTaskIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCourseV2ActivityPb {
    pub HadTakeReward: Vec<i32>,
    pub BeginOpenTime: i64,
}

impl PbMessage for NewbieCourseV2ActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HadTakeReward {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
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
            2 => self.BeginOpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureV2Pb {
    pub Chapter: Vec<NewbieAdventureV2ChapterPb>,
}

impl PbMessage for NewbieAdventureV2Pb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Chapter {
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
                self.Chapter
                    .push(<NewbieAdventureV2ChapterPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureV2ChapterPb {
    pub id: i32,
    pub Task: Vec<ConditionTask>,
    pub RewardRoleId: bool,
    pub RewardedRoleId: i32,
    pub RewardDrop: bool,
}

impl PbMessage for NewbieAdventureV2ChapterPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.id);
        }
        for item in &self.Task {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.RewardRoleId {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardedRoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedRoleId);
        }
        if self.RewardDrop {
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
            1 => self.id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            3 => self.RewardRoleId = r.read_bool()?,
            4 => self.RewardedRoleId = r.read_varint_i32()?,
            5 => self.RewardDrop = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueInsEntry {
    pub instId: i32,
    pub DifficultyName: Vec<u8>,
    pub Level: i32,
    pub ExpectedTimeMinSec: i32,
    pub ExpectedTimeMaxSec: i32,
    pub Unlocked: bool,
    pub BestGrade: Option<RoverRogueGrade>,
}

impl PbMessage for RoverRogueInsEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.instId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.instId);
        }
        if !self.DifficultyName.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DifficultyName);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.ExpectedTimeMinSec != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExpectedTimeMinSec);
        }
        if self.ExpectedTimeMaxSec != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExpectedTimeMaxSec);
        }
        if self.Unlocked {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.BestGrade {
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
            1 => self.instId = r.read_varint_i32()?,
            2 => self.DifficultyName = r.read_len_delimited()?.to_vec(),
            3 => self.Level = r.read_varint_i32()?,
            4 => self.ExpectedTimeMinSec = r.read_varint_i32()?,
            5 => self.ExpectedTimeMaxSec = r.read_varint_i32()?,
            6 => self.Unlocked = r.read_bool()?,
            7 => {
                self.BestGrade = Some(
                    <RoverRogueGrade as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoverRogueActivityData {
    pub activityId: i32,
    pub HistoryInsInfo: Option<RoverRogueHistoryInsInfo>,
    pub UnlockRoleIdList: Vec<i32>,
    pub UnlockBlessRoleIdList: Vec<i32>,
    pub UnlockBlessIdList: Vec<i32>,
    pub UnlockTalentIdList: Vec<i32>,
    pub UnlockItemIdList: Vec<i32>,
    pub UnlockLootIdList: Vec<i32>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub ActiveTalentIdList: Vec<i32>,
    pub TokenItem: i32,
    pub TalentItem: i32,
    pub UnlockRoleType: Vec<i32>,
    pub EquippedLoot: i32,
    pub InsInfoList: Vec<RoverRogueInsEntry>,
}

impl PbMessage for RoverRogueActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.activityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.activityId);
        }
        if let Some(v) = &self.HistoryInsInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.UnlockRoleIdList {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBlessRoleIdList {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBlessIdList {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockTalentIdList {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockItemIdList {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockLootIdList {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.ActiveTalentIdList {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TokenItem != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenItem);
        }
        if self.TalentItem != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentItem);
        }
        for item in &self.UnlockRoleType {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EquippedLoot != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquippedLoot);
        }
        for item in &self.InsInfoList {
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
            1 => self.activityId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HistoryInsInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockRoleIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBlessRoleIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBlessIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockTalentIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockItemIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockLootIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ActiveTalentIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => self.TokenItem = r.read_varint_i32()?,
            12 => self.TalentItem = r.read_varint_i32()?,
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockRoleType, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => self.EquippedLoot = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.InsInfoList
                    .push(<RoverRogueInsEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueHistoryInsInfo {
    pub CurInsId: i32,
    pub CurLayer: i32,
    pub TotalLayer: i32,
    pub LootId: i32,
}

impl PbMessage for RoverRogueHistoryInsInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurInsId);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.TotalLayer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalLayer);
        }
        if self.LootId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LootId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurInsId = r.read_varint_i32()?,
            2 => self.CurLayer = r.read_varint_i32()?,
            3 => self.TotalLayer = r.read_varint_i32()?,
            4 => self.LootId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QingXiaoActivityInfo {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl PbMessage for QingXiaoActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlActivityInfo {
    pub ConditionTasks: Vec<ConditionTask>,
    pub UnlockedRoleIds: Vec<i32>,
    pub UnlockedItemIds: Vec<i32>,
    pub ChallengeInfos: Vec<DangoBrawlChallengeInfo>,
    pub SaveMatchBrief: Option<DangoBrawlSaveMatchBrief>,
    pub RewardScoreIds: Vec<i32>,
    pub LimitCoinCost: i32,
}

impl PbMessage for DangoBrawlActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.UnlockedRoleIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedItemIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ChallengeInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.SaveMatchBrief {
            crate::wire::write_submessage(out, 5, v);
        }
        for item in &self.RewardScoreIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LimitCoinCost != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitCoinCost);
        }
    }

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
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedRoleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedItemIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfos
                    .push(<DangoBrawlChallengeInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SaveMatchBrief
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardScoreIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            7 => self.LimitCoinCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlSaveMatchBrief {
    pub ChallengeId: i32,
    pub RoundNum: i32,
    pub RoleId: i32,
}

impl PbMessage for DangoBrawlSaveMatchBrief {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.RoundNum = r.read_varint_i32()?,
            3 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlChallengeInfo {
    pub ChallengeId: i32,
    pub State: Option<DangoBrawlChallengeState>,
    pub RetryCount: i32,
}

impl PbMessage for DangoBrawlChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RetryCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RetryCount);
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
            2 => {
                self.State = Some(
                    <DangoBrawlChallengeState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.RetryCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchThreeActivityInfo {
    pub MatchThreeLevelInfos: Vec<MatchThreeLevelInfo>,
}

impl PbMessage for MatchThreeActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MatchThreeLevelInfos {
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
                self.MatchThreeLevelInfos
                    .push(<MatchThreeLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchThreeLevelInfoHbyOneof {
    Score(i32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchThreeLevelInfo {
    pub Hby: Option<MatchThreeLevelInfoHbyOneof>,
    pub Id: i32,
    pub UnlockTime: i64,
    pub FirstPass: bool,
}

impl PbMessage for MatchThreeLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Hby {
            match oneof {
                MatchThreeLevelInfoHbyOneof::Score(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.FirstPass {
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
            4 => self.Hby = Some(MatchThreeLevelInfoHbyOneof::Score(r.read_varint_i32()?)),
            1 => self.Id = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => self.FirstPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolActivityPb {
    pub TaskType: Vec<SolarSchoolTaskTypePb>,
    pub Gallery: Vec<SolarSchoolGalleryPb>,
    pub SavePreview: Vec<SolarSchoolSavePreviewPb>,
    pub GalleryEntryUnlocked: bool,
    pub LimitTaskProgressRewarded: bool,
}

impl PbMessage for SolarSchoolActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskType {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Gallery {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.SavePreview {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.GalleryEntryUnlocked {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LimitTaskProgressRewarded {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TaskType
                    .push(<SolarSchoolTaskTypePb as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Gallery
                    .push(<SolarSchoolGalleryPb as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SavePreview
                    .push(<SolarSchoolSavePreviewPb as PbMessage>::decode(bytes)?);
            }
            5 => self.GalleryEntryUnlocked = r.read_bool()?,
            6 => self.LimitTaskProgressRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSavePreviewPb {
    pub SlotId: i32,
    pub CycleCount: i32,
    pub Attr: Vec<SolarSchoolAttrPb>,
    pub GameDay: i32,
    pub LastSaveTime: i64,
}

impl PbMessage for SolarSchoolSavePreviewPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.CycleCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleCount);
        }
        for item in &self.Attr {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.GameDay != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GameDay);
        }
        if self.LastSaveTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastSaveTime);
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
            2 => self.CycleCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Attr
                    .push(<SolarSchoolAttrPb as PbMessage>::decode(bytes)?);
            }
            4 => self.GameDay = r.read_varint_i32()?,
            5 => self.LastSaveTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolAttrPb {
    pub AttrType: i32,
    pub Value: i32,
    pub Important: bool,
}

impl PbMessage for SolarSchoolAttrPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttrType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrType);
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Value);
        }
        if self.Important {
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
            1 => self.AttrType = r.read_varint_i32()?,
            2 => self.Value = r.read_varint_i32()?,
            3 => self.Important = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolTaskTypePb {
    pub TaskType: i32,
    pub Task: Vec<ConditionTask>,
}

impl PbMessage for SolarSchoolTaskTypePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskType);
        }
        for item in &self.Task {
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
            1 => self.TaskType = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolGalleryPb {
    pub Id: i32,
    pub IsReward: bool,
}

impl PbMessage for SolarSchoolGalleryPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsReward {
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
            2 => self.IsReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerActivityData {
    pub RankRewardSettleTime: i64,
    pub ChallengeInfos: Vec<RoleTowerChallengeInfo>,
    pub UnlockTokens: Vec<i32>,
    pub ScoreTasks: Vec<i32>,
    pub LimitTasks: Vec<ConditionTask>,
    pub Score: i32,
}

impl PbMessage for RoleTowerActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RankRewardSettleTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RankRewardSettleTime);
        }
        for item in &self.ChallengeInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnlockTokens {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ScoreTasks {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LimitTasks {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            1 => self.RankRewardSettleTime = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfos
                    .push(<RoleTowerChallengeInfo as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockTokens, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTasks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.LimitTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            6 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleTowerChallengeInfoRiskHarvestActivityInfoOneof {
    SaveBrief(RoleTowerChallengeSaveBrief),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleTowerChallengeInfoUNSOneof {
    BestRecord(RoleTowerBestRecord),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeInfo {
    pub RiskHarvestActivityInfo: Option<RoleTowerChallengeInfoRiskHarvestActivityInfoOneof>,
    pub uNS: Option<RoleTowerChallengeInfoUNSOneof>,
    pub ChallengeId: i32,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub IsAllPassed: bool,
    pub HasSave: bool,
    pub StageInfos: Vec<RoleTowerStageInfo>,
    pub RoleEnergyInfos: Vec<RoleTowerRoleEnergyInfo>,
    pub InitialTokens: BTreeMap<i32, i32>,
}

impl PbMessage for RoleTowerChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.RiskHarvestActivityInfo {
            match oneof {
                RoleTowerChallengeInfoRiskHarvestActivityInfoOneof::SaveBrief(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
            }
        }
        if let Some(oneof) = &self.uNS {
            match oneof {
                RoleTowerChallengeInfoUNSOneof::BestRecord(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
            }
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsAllPassed {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HasSave {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.StageInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.RoleEnergyInfos {
            crate::wire::write_submessage(out, 9, item);
        }
        if !self.InitialTokens.is_empty() {
            crate::wire::write_map(out, 10, &self.InitialTokens);
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
                let val = <RoleTowerChallengeSaveBrief as PbMessage>::decode(bytes)?;
                self.RiskHarvestActivityInfo =
                    Some(RoleTowerChallengeInfoRiskHarvestActivityInfoOneof::SaveBrief(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTowerBestRecord as PbMessage>::decode(bytes)?;
                self.uNS = Some(RoleTowerChallengeInfoUNSOneof::BestRecord(val));
            }
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => self.IsAllPassed = r.read_bool()?,
            5 => self.HasSave = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.StageInfos
                    .push(<RoleTowerStageInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEnergyInfos
                    .push(<RoleTowerRoleEnergyInfo as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_map_entry(r, &mut self.InitialTokens)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRoleEnergyInfo {
    pub RoleIdOrTrialRoleId: i32,
    pub TrialRoleId: i32,
    pub CurEnergy: i32,
    pub MaxEnergy: i32,
    pub EnergyCost: i32,
}

impl PbMessage for RoleTowerRoleEnergyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIdOrTrialRoleId);
        }
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrialRoleId);
        }
        if self.CurEnergy != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurEnergy);
        }
        if self.MaxEnergy != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxEnergy);
        }
        if self.EnergyCost != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnergyCost);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            2 => self.TrialRoleId = r.read_varint_i32()?,
            3 => self.CurEnergy = r.read_varint_i32()?,
            4 => self.MaxEnergy = r.read_varint_i32()?,
            5 => self.EnergyCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPhantomInfo {
    pub Level: i32,
    pub CurExp: i32,
}

impl PbMessage for RoleTowerPhantomInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.CurExp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurExp);
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
            2 => self.CurExp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRoleSnapshot {
    pub RoleIdOrTrialRoleId: i32,
    pub RoleLevel: i32,
    pub WeaponIncId: i32,
    pub PhantomIncIds: Vec<i32>,
    pub SkillBranchId: i32,
}

impl PbMessage for RoleTowerRoleSnapshot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIdOrTrialRoleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.WeaponIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponIncId);
        }
        for item in &self.PhantomIncIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            3 => self.RoleLevel = r.read_varint_i32()?,
            5 => self.WeaponIncId = r.read_varint_i32()?,
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PhantomIncIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            7 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeSaveBrief {
    pub CurStageId: i32,
    pub PassedFloorCount: i32,
    pub RemainTimeMs: i64,
    pub SaveTime: i64,
    pub InitialTokens: BTreeMap<i32, i32>,
    pub RoleList: Vec<i32>,
    pub RoleEnergyInfos: Option<RoleTowerRoleEnergyInfo>,
    pub PhantomAddAttrs: BTreeMap<i32, i32>,
    pub PhantomAddAttrInfos: Vec<RoleTowerPhantomAddAttr>,
    pub RoleTowerRoleEnergyInfos: Vec<RoleTowerRoleEnergyInfo>,
    pub TeamStarInfo: BTreeMap<i32, i32>,
    pub PhantomInfo: Option<RoleTowerPhantomInfo>,
}

impl PbMessage for RoleTowerChallengeSaveBrief {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurStageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStageId);
        }
        if self.PassedFloorCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassedFloorCount);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        if self.SaveTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SaveTime);
        }
        if !self.InitialTokens.is_empty() {
            crate::wire::write_map(out, 5, &self.InitialTokens);
        }
        for item in &self.RoleList {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.RoleEnergyInfos {
            crate::wire::write_submessage(out, 9, v);
        }
        if !self.PhantomAddAttrs.is_empty() {
            crate::wire::write_map(out, 10, &self.PhantomAddAttrs);
        }
        for item in &self.PhantomAddAttrInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.RoleTowerRoleEnergyInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        if !self.TeamStarInfo.is_empty() {
            crate::wire::write_map(out, 13, &self.TeamStarInfo);
        }
        if let Some(v) = &self.PhantomInfo {
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
            1 => self.CurStageId = r.read_varint_i32()?,
            2 => self.PassedFloorCount = r.read_varint_i32()?,
            3 => self.RemainTimeMs = r.read_varint_i64()?,
            4 => self.SaveTime = r.read_varint_i64()?,
            5 => crate::wire::decode_map_entry(r, &mut self.InitialTokens)?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEnergyInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => crate::wire::decode_map_entry(r, &mut self.PhantomAddAttrs)?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomAddAttrInfos
                    .push(<RoleTowerPhantomAddAttr as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RoleTowerRoleEnergyInfos
                    .push(<RoleTowerRoleEnergyInfo as PbMessage>::decode(bytes)?);
            }
            13 => crate::wire::decode_map_entry(r, &mut self.TeamStarInfo)?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPhantomAddAttr {
    pub PhantomId: i32,
    pub SlotIndex: i32,
    pub Attrs: BTreeMap<i32, i32>,
    pub RoleIdOrTrialRoleId: i32,
}

impl PbMessage for RoleTowerPhantomAddAttr {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomId);
        }
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
        if !self.Attrs.is_empty() {
            crate::wire::write_map(out, 3, &self.Attrs);
        }
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.PhantomId = r.read_varint_i32()?,
            2 => self.SlotIndex = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.Attrs)?,
            4 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerStageInfo {
    pub StageId: i32,
    pub State: i32,
    pub EnergyCost: i32,
    pub PassRemainTimeMs: i64,
    pub PassRoles: Vec<RoleTowerRoleBrief>,
    pub FloorInfos: Vec<RoleTowerFloorInfo>,
    pub RoleSnapshots: Vec<RoleTowerRoleSnapshot>,
}

impl PbMessage for RoleTowerStageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StageId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if self.EnergyCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnergyCost);
        }
        if self.PassRemainTimeMs != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassRemainTimeMs);
        }
        for item in &self.PassRoles {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.FloorInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.RoleSnapshots {
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
            1 => self.StageId = r.read_varint_i32()?,
            2 => self.State = r.read_varint_i32()?,
            3 => self.EnergyCost = r.read_varint_i32()?,
            4 => self.PassRemainTimeMs = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PassRoles
                    .push(<RoleTowerRoleBrief as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FloorInfos
                    .push(<RoleTowerFloorInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RoleSnapshots
                    .push(<RoleTowerRoleSnapshot as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerFloorInfo {
    pub FloorId: i32,
    pub FloorType: i32,
    pub HasBranch: bool,
    pub BranchInfos: Vec<RoleTowerFloorBranchInfo>,
    pub PassRemainTimeMs: i64,
    pub IsPassed: bool,
}

impl PbMessage for RoleTowerFloorInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FloorId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FloorId);
        }
        if self.FloorType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FloorType);
        }
        if self.HasBranch {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.BranchInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.PassRemainTimeMs != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassRemainTimeMs);
        }
        if self.IsPassed {
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
            1 => self.FloorId = r.read_varint_i32()?,
            2 => self.FloorType = r.read_varint_i32()?,
            3 => self.HasBranch = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BranchInfos
                    .push(<RoleTowerFloorBranchInfo as PbMessage>::decode(bytes)?);
            }
            6 => self.PassRemainTimeMs = r.read_varint_i64()?,
            7 => self.IsPassed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerFloorBranchInfo {
    pub BranchIndex: i32,
    pub MonsterPreviewIds: Vec<i32>,
    pub TokenRewards: Vec<RoleTowerTokenCount>,
}

impl PbMessage for RoleTowerFloorBranchInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BranchIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchIndex);
        }
        for item in &self.MonsterPreviewIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TokenRewards {
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
            1 => self.BranchIndex = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MonsterPreviewIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TokenRewards
                    .push(<RoleTowerTokenCount as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRoleBrief {
    pub RoleIdOrTrialRoleId: i32,
    pub SkillBranchId: i32,
}

impl PbMessage for RoleTowerRoleBrief {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIdOrTrialRoleId);
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
            1 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            2 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTokenCount {
    pub TokenId: i32,
    pub Count: i32,
}

impl PbMessage for RoleTowerTokenCount {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TokenId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenId);
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
            1 => self.TokenId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerBestRecord {
    pub ChallengeId: i32,
    pub MainRoleId: i32,
    pub PassedFloorCount: i32,
    pub RemainTimeMs: i64,
    pub CostTimeMs: i64,
    pub SettleTime: i64,
    pub IsAllPassed: bool,
    pub StageRecords: Vec<RoleTowerStageRecord>,
    pub IsNewRecord: bool,
}

impl PbMessage for RoleTowerBestRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.MainRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MainRoleId);
        }
        if self.PassedFloorCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassedFloorCount);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        if self.CostTimeMs != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTimeMs);
        }
        if self.SettleTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SettleTime);
        }
        if self.IsAllPassed {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.StageRecords {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.IsNewRecord {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.MainRoleId = r.read_varint_i32()?,
            3 => self.PassedFloorCount = r.read_varint_i32()?,
            4 => self.RemainTimeMs = r.read_varint_i64()?,
            5 => self.CostTimeMs = r.read_varint_i64()?,
            6 => self.SettleTime = r.read_varint_i64()?,
            7 => self.IsAllPassed = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.StageRecords
                    .push(<RoleTowerStageRecord as PbMessage>::decode(bytes)?);
            }
            9 => self.IsNewRecord = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerStageRecord {
    pub StageId: i32,
    pub IsPassed: bool,
    pub GainTimeMs: i64,
    pub CostTimeMs: i64,
    pub RemainTimeMs: i64,
    pub Roles: Vec<RoleTowerRoleBrief>,
    pub FloorRecords: Vec<RoleTowerFloorRecord>,
}

impl PbMessage for RoleTowerStageRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StageId);
        }
        if self.IsPassed {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.GainTimeMs != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GainTimeMs);
        }
        if self.CostTimeMs != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTimeMs);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        for item in &self.Roles {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.FloorRecords {
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
            1 => self.StageId = r.read_varint_i32()?,
            2 => self.IsPassed = r.read_bool()?,
            3 => self.GainTimeMs = r.read_varint_i64()?,
            4 => self.CostTimeMs = r.read_varint_i64()?,
            5 => self.RemainTimeMs = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .push(<RoleTowerRoleBrief as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.FloorRecords
                    .push(<RoleTowerFloorRecord as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerFloorRecord {
    pub FloorId: i32,
    pub IsPassed: bool,
    pub BranchIndex: i32,
    pub RemainTimeMs: i64,
    pub CostTimeMs: i64,
    pub GainTimeMs: i64,
}

impl PbMessage for RoleTowerFloorRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FloorId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FloorId);
        }
        if self.IsPassed {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BranchIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchIndex);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        if self.CostTimeMs != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTimeMs);
        }
        if self.GainTimeMs != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GainTimeMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FloorId = r.read_varint_i32()?,
            2 => self.IsPassed = r.read_bool()?,
            3 => self.BranchIndex = r.read_varint_i32()?,
            4 => self.RemainTimeMs = r.read_varint_i64()?,
            5 => self.CostTimeMs = r.read_varint_i64()?,
            6 => self.GainTimeMs = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventreTask {
    pub Id: i32,
    pub State: Option<AdventreTaskState>,
    pub AdventreProgress: i32,
}

impl PbMessage for AdventreTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AdventreProgress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AdventreProgress);
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
                    <AdventreTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.AdventreProgress = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureManualData {
    pub AdventreTask: Vec<AdventreTask>,
    pub NowChapter: i32,
    pub ReceivedChapter: i32,
    pub UnlockChapters: Vec<i32>,
    pub RewardChapters: Vec<i32>,
}

impl PbMessage for AdventureManualData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AdventreTask {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.NowChapter != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NowChapter);
        }
        if self.ReceivedChapter != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReceivedChapter);
        }
        for item in &self.UnlockChapters {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardChapters {
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
                self.AdventreTask
                    .push(<AdventreTask as PbMessage>::decode(bytes)?);
            }
            2 => self.NowChapter = r.read_varint_i32()?,
            3 => self.ReceivedChapter = r.read_varint_i32()?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockChapters, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardChapters, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureItemData {
    pub ItemId: i32,
    pub ItemNum: i32,
}

impl PbMessage for AdventureItemData {
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
pub struct AdventureRewardData {
    pub DropId: i32,
    pub Items: Vec<AdventureItemData>,
}

impl PbMessage for AdventureRewardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropId);
        }
        for item in &self.Items {
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
            1 => self.DropId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AdventureItemData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionTarget {
    pub Id: i32,
    pub Type: Option<DetectionType>,
    pub UnlockState: bool,
    pub RefresherTime: i64,
    pub DetectionId: i32,
    pub IsTrace: i32,
}

impl PbMessage for DetectionTarget {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UnlockState {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RefresherTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RefresherTime);
        }
        if self.DetectionId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionId);
        }
        if self.IsTrace != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IsTrace);
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
                self.Type = Some(<DetectionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.UnlockState = r.read_bool()?,
            4 => self.RefresherTime = r.read_varint_i64()?,
            5 => self.DetectionId = r.read_varint_i32()?,
            6 => self.IsTrace = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionUnlock {
    pub MonsterDetectionIds: Vec<i32>,
    pub DungeonDetectionIds: Vec<i32>,
    pub SilentAreaDetectionIds: Vec<i32>,
}

impl PbMessage for DetectionUnlock {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MonsterDetectionIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DungeonDetectionIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SilentAreaDetectionIds {
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
                &mut self.MonsterDetectionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DungeonDetectionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SilentAreaDetectionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectDetectionTarget {
    pub DetectionId: i32,
    pub Type: Option<DetectionType>,
    pub Id: i32,
    pub IsTrace: i32,
}

impl PbMessage for SelectDetectionTarget {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsTrace != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IsTrace);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DetectionId = r.read_varint_i32()?,
            2 => {
                self.Type = Some(<DetectionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.Id = r.read_varint_i32()?,
            4 => self.IsTrace = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureManualDataRequest {
    pub PlayerId: i32,
}

impl AdventureManualDataRequest {
    pub const MSG_ID: u16 = 25198;
}

impl PbMessage for AdventureManualDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            3 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureManualDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AdventureManualData: Option<AdventureManualData>,
}

impl AdventureManualDataResponse {
    pub const MSG_ID: u16 = 24214;
}

impl PbMessage for AdventureManualDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.AdventureManualData {
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.AdventureManualData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureUpdateNotify {
    pub AdventureManualData: Vec<AdventureManualData>,
}

impl AdventureUpdateNotify {
    pub const MSG_ID: u16 = 22975;
}

impl PbMessage for AdventureUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AdventureManualData {
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
                self.AdventureManualData
                    .push(<AdventureManualData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureManualRequest {
    pub PlayerId: i32,
}

impl AdventureManualRequest {
    pub const MSG_ID: u16 = 17641;
}

impl PbMessage for AdventureManualRequest {
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
pub struct AdventureManualResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AdventureManualData: Option<AdventureManualData>,
    pub DetectionTarget: Vec<DetectionTarget>,
    pub AdventureRewardData: Vec<AdventureRewardData>,
    pub DetectionUnlocks: Option<DetectionUnlock>,
    pub NowSelectDetectionTarget: Option<SelectDetectionTarget>,
    pub SlientFirstAwardMap: BTreeMap<i32, SlientFirstAwardState>,
    pub SilenceAreaConfigs: Vec<AdventureDetectionConfig>,
    pub DungeonDetections: Vec<AdventureDetectionConfig>,
    pub PreOpeDungeonDetections: Vec<PreOpenDetections>,
    pub PreOpenSilenceAreaDetections: Vec<PreOpenDetections>,
}

impl AdventureManualResponse {
    pub const MSG_ID: u16 = 19686;
}

impl PbMessage for AdventureManualResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.AdventureManualData {
            crate::wire::write_submessage(out, 14, v);
        }
        for item in &self.DetectionTarget {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.AdventureRewardData {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.DetectionUnlocks {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.NowSelectDetectionTarget {
            crate::wire::write_submessage(out, 12, v);
        }
        if !self.SlientFirstAwardMap.is_empty() {
            crate::wire::write_map(out, 8, &self.SlientFirstAwardMap);
        }
        for item in &self.SilenceAreaConfigs {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.DungeonDetections {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PreOpeDungeonDetections {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.PreOpenSilenceAreaDetections {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.AdventureManualData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionTarget
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AdventureRewardData
                    .push(<AdventureRewardData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionUnlocks
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.NowSelectDetectionTarget
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => crate::wire::decode_map_entry(r, &mut self.SlientFirstAwardMap)?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SilenceAreaConfigs
                    .push(<AdventureDetectionConfig as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.DungeonDetections
                    .push(<AdventureDetectionConfig as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PreOpeDungeonDetections
                    .push(<PreOpenDetections as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PreOpenSilenceAreaDetections
                    .push(<PreOpenDetections as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreOpenDetections {
    pub Id: i32,
    pub PreOpenId: i32,
    pub PreOpenBeginTime: i64,
    pub PreOpenEndTIme: i64,
}

impl PbMessage for PreOpenDetections {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.PreOpenId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreOpenId);
        }
        if self.PreOpenBeginTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PreOpenBeginTime);
        }
        if self.PreOpenEndTIme != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PreOpenEndTIme);
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
            2 => self.PreOpenId = r.read_varint_i32()?,
            3 => self.PreOpenBeginTime = r.read_varint_i64()?,
            4 => self.PreOpenEndTIme = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureDetectionConfig {
    pub Id: i32,
    pub EffectBeginTime: i64,
    pub EffectEndTime: i64,
}

impl PbMessage for AdventureDetectionConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.EffectBeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EffectBeginTime);
        }
        if self.EffectEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EffectEndTime);
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
            2 => self.EffectBeginTime = r.read_varint_i64()?,
            3 => self.EffectEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockDetectionLabelInfo {
    pub UnlockedGuideIds: Vec<i32>,
    pub UnlockedDetectionTextIds: Vec<i32>,
}

impl PbMessage for UnlockDetectionLabelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedGuideIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, *item as i32);
        }
        for item in &self.UnlockedDetectionTextIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, *item as i32);
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
                &mut self.UnlockedGuideIds,
                |sr| Ok(sr.read_sint32()?),
            )?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedDetectionTextIds,
                |sr| Ok(sr.read_sint32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetDetectionLabelInfoRequest {}

impl GetDetectionLabelInfoRequest {
    pub const MSG_ID: u16 = 22348;
}

impl PbMessage for GetDetectionLabelInfoRequest {
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
pub struct GetDetectionLabelInfoResponse {
    pub UnlockLabelInfo: Option<UnlockDetectionLabelInfo>,
}

impl GetDetectionLabelInfoResponse {
    pub const MSG_ID: u16 = 21286;
}

impl PbMessage for GetDetectionLabelInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UnlockLabelInfo {
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
                self.UnlockLabelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceSettingNotify {
    pub IsShow: bool,
}

impl AdviceSettingNotify {
    pub const MSG_ID: u16 = 22745;
}

impl PbMessage for AdviceSettingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
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
            13 => self.IsShow = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceSetRequest {
    pub IsShow: bool,
}

impl AdviceSetRequest {
    pub const MSG_ID: u16 = 24164;
}

impl PbMessage for AdviceSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
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
            10 => self.IsShow = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceSetResponse {
    pub IsShow: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceSetResponse {
    pub const MSG_ID: u16 = 20925;
}

impl PbMessage for AdviceSetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => self.IsShow = r.read_bool()?,
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
pub struct PbAdvice {
    pub Id: i64,
    pub AreaId: i32,
    pub Contents: Vec<PbAdviceContent>,
    pub UpVote: i32,
}

impl PbMessage for PbAdvice {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        for item in &self.Contents {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.UpVote != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UpVote);
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
            2 => self.AreaId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Contents
                    .push(<PbAdviceContent as PbMessage>::decode(bytes)?);
            }
            4 => self.UpVote = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceComponentPb {
    pub Advice: Option<PbAdvice>,
    pub PlayerId: i32,
    pub PlayerName: Vec<u8>,
}

impl PbMessage for AdviceComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Advice {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Advice
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.PlayerId = r.read_varint_i32()?,
            3 => self.PlayerName = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbAdviceContent {
    pub Type: Option<PbAdviceContentType>,
    pub Id: i32,
    pub Word: i32,
}

impl PbMessage for PbAdviceContent {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Word != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Word);
        }
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
                    <PbAdviceContentType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Id = r.read_varint_i32()?,
            3 => self.Word = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceRequest {}

impl AdviceRequest {
    pub const MSG_ID: u16 = 23265;
}

impl PbMessage for AdviceRequest {
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
pub struct AdviceResponse {
    pub Advices: Vec<PbAdvice>,
    pub UpVoteIds: Vec<i64>,
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceResponse {
    pub const MSG_ID: u16 = 28955;
}

impl PbMessage for AdviceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Advices {
            crate::wire::write_submessage(out, 14, item);
        }
        for item in &self.UpVoteIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Advices.push(<PbAdvice as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UpVoteIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
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
pub struct AiHateEntity {
    pub EntityId: i64,
    pub HatredValue: i32,
}

impl PbMessage for AiHateEntity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HatredValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HatredValue);
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
            2 => self.HatredValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Int2Long {
    pub First: i32,
    pub Second: i64,
}

impl PbMessage for Int2Long {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.First != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.First);
        }
        if self.Second != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Second);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.First = r.read_varint_i32()?,
            2 => self.Second = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Int2Bool {
    pub First: i32,
    pub Second: bool,
}

impl PbMessage for Int2Bool {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.First != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.First);
        }
        if self.Second {
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
            1 => self.First = r.read_varint_i32()?,
            2 => self.Second = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiInformation {
    pub AiBlackboards: Vec<BlackboardParam>,
    pub HateList: Vec<AiHateEntity>,
    pub AiBlackboardCd: Vec<Int2Long>,
}

impl PbMessage for AiInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboards {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.HateList {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.AiBlackboardCd {
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
                self.AiBlackboards
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HateList
                    .push(<AiHateEntity as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCd
                    .push(<Int2Long as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiInformationRequest {
    pub AiInfo: Option<AiInformation>,
}

impl AiInformationRequest {
    pub const MSG_ID: u16 = 18681;
}

impl PbMessage for AiInformationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AiInfo {
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
pub struct AiInformationPush {
    pub AiInfo: Option<AiInformation>,
}

impl AiInformationPush {
    pub const MSG_ID: u16 = 22125;
}

impl PbMessage for AiInformationPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AiInfo {
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
pub struct AiInformationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AiInformationResponse {
    pub const MSG_ID: u16 = 20922;
}

impl PbMessage for AiInformationResponse {
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
pub struct AiInformationNotify {
    pub AiBlackboardCd: Vec<Int2Long>,
}

impl AiInformationNotify {
    pub const MSG_ID: u16 = 26510;
}

impl PbMessage for AiInformationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboardCd {
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
                self.AiBlackboardCd
                    .push(<Int2Long as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiBlackboardsRequest {
    pub AiBlackboards: Vec<BlackboardParam>,
}

impl AiBlackboardsRequest {
    pub const MSG_ID: u16 = 22558;
}

impl PbMessage for AiBlackboardsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboards {
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
                self.AiBlackboards
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiBlackboardsPush {
    pub AiBlackboards: Vec<BlackboardParam>,
}

impl AiBlackboardsPush {
    pub const MSG_ID: u16 = 16697;
}

impl PbMessage for AiBlackboardsPush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboards {
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
                self.AiBlackboards
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiBlackboardsResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AiBlackboardsResponse {
    pub const MSG_ID: u16 = 27598;
}

impl PbMessage for AiBlackboardsResponse {
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
pub struct AiBlackboardCdRequest {
    pub AiBlackboardCdModify: Vec<Int2Long>,
    pub AiBlackboardCdComplete: Vec<Int2Bool>,
}

impl AiBlackboardCdRequest {
    pub const MSG_ID: u16 = 15625;
}

impl PbMessage for AiBlackboardCdRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboardCdModify {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.AiBlackboardCdComplete {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdModify
                    .push(<Int2Long as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdComplete
                    .push(<Int2Bool as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiBlackboardCdPush {
    pub AiBlackboardCdModify: Vec<Int2Long>,
    pub AiBlackboardCdComplete: Vec<Int2Bool>,
}

impl AiBlackboardCdPush {
    pub const MSG_ID: u16 = 18641;
}

impl PbMessage for AiBlackboardCdPush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboardCdModify {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.AiBlackboardCdComplete {
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
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdModify
                    .push(<Int2Long as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdComplete
                    .push(<Int2Bool as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiBlackboardCdResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AiBlackboardCdResponse {
    pub const MSG_ID: u16 = 29807;
}

impl PbMessage for AiBlackboardCdResponse {
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
pub struct AiBlackboardCdNotify {
    pub AiBlackboardCdDel: Vec<i32>,
    pub AiBlackboardCdModify: Vec<Int2Long>,
    pub AiBlackboardCdComplete: Vec<Int2Bool>,
}

impl AiBlackboardCdNotify {
    pub const MSG_ID: u16 = 26019;
}

impl PbMessage for AiBlackboardCdNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiBlackboardCdDel {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AiBlackboardCdModify {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.AiBlackboardCdComplete {
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
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AiBlackboardCdDel,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdModify
                    .push(<Int2Long as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AiBlackboardCdComplete
                    .push(<Int2Bool as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiHateRequest {
    pub HateList: Vec<AiHateEntity>,
}

impl AiHateRequest {
    pub const MSG_ID: u16 = 27610;
}

impl PbMessage for AiHateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HateList {
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
                self.HateList
                    .push(<AiHateEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiHatePush {
    pub HateList: Vec<AiHateEntity>,
}

impl AiHatePush {
    pub const MSG_ID: u16 = 22440;
}

impl PbMessage for AiHatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HateList {
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
                self.HateList
                    .push(<AiHateEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiHateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AiHateResponse {
    pub const MSG_ID: u16 = 20048;
}

impl PbMessage for AiHateResponse {
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
pub struct AiHateNotify {
    pub HateList: Vec<AiHateEntity>,
}

impl AiHateNotify {
    pub const MSG_ID: u16 = 17752;
}

impl PbMessage for AiHateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HateList {
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
                self.HateList
                    .push(<AiHateEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IntArrayBlackboard {
    pub Values: Vec<i32>,
}

impl PbMessage for IntArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Values, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongArrayBlackboard {
    pub Values: Vec<i64>,
}

impl PbMessage for LongArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Values, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StringArrayBlackboard {
    pub Values: Vec<Vec<u8>>,
}

impl PbMessage for StringArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
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
            1 => self.Values.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloatArrayBlackboard {
    pub Values: Vec<f32>,
}

impl PbMessage for FloatArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(*item).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Values, |sr| {
                Ok(sr.read_f32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorArrayBlackboard {
    pub Values: Vec<Vector>,
}

impl PbMessage for VectorArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
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
                self.Values.push(<Vector as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RotatorArrayBlackboard {
    pub Values: Vec<Rotator>,
}

impl PbMessage for RotatorArrayBlackboard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Values {
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
                self.Values.push(<Rotator as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlackboardParamValueOneof {
    IntValue(i32),
    IntValues(IntArrayBlackboard),
    LongValue(i64),
    LongValues(LongArrayBlackboard),
    BooleanValue(bool),
    StringValue(Vec<u8>),
    StringValues(StringArrayBlackboard),
    FloatValue(f32),
    FloatValues(FloatArrayBlackboard),
    VectorValue(Vector),
    VectorValues(VectorArrayBlackboard),
    RotatorValue(Rotator),
    RotatorValues(RotatorArrayBlackboard),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackboardParam {
    pub Value: Option<BlackboardParamValueOneof>,
    pub Key: Vec<u8>,
    pub Type: Option<BlackboardParamType>,
}

impl PbMessage for BlackboardParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Value {
            match oneof {
                BlackboardParamValueOneof::IntValue(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                BlackboardParamValueOneof::IntValues(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                BlackboardParamValueOneof::LongValue(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                BlackboardParamValueOneof::LongValues(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                BlackboardParamValueOneof::BooleanValue(v) => {
                    crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
                BlackboardParamValueOneof::StringValue(v) => {
                    crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
                BlackboardParamValueOneof::StringValues(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                BlackboardParamValueOneof::FloatValue(v) => {
                    crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
                BlackboardParamValueOneof::FloatValues(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                BlackboardParamValueOneof::VectorValue(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                BlackboardParamValueOneof::VectorValues(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                BlackboardParamValueOneof::RotatorValue(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                BlackboardParamValueOneof::RotatorValues(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
            }
        }
        if !self.Key.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Key);
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
            3 => self.Value = Some(BlackboardParamValueOneof::IntValue(r.read_varint_i32()?)),
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <IntArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::IntValues(val));
            }
            5 => self.Value = Some(BlackboardParamValueOneof::LongValue(r.read_varint_i64()?)),
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <LongArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::LongValues(val));
            }
            7 => self.Value = Some(BlackboardParamValueOneof::BooleanValue(r.read_bool()?)),
            8 => {
                self.Value = Some(BlackboardParamValueOneof::StringValue(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <StringArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::StringValues(val));
            }
            10 => self.Value = Some(BlackboardParamValueOneof::FloatValue(r.read_f32()?)),
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <FloatArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::FloatValues(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::VectorValue(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <VectorArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::VectorValues(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <Rotator as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::RotatorValue(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <RotatorArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(BlackboardParamValueOneof::RotatorValues(val));
            }
            1 => self.Key = r.read_len_delimited()?.to_vec(),
            2 => {
                self.Type = Some(
                    <BlackboardParamType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EnterAreaRequest {
    pub Id: i32,
    pub LeaveId: i32,
}

impl EnterAreaRequest {
    pub const MSG_ID: u16 = 22737;
}

impl PbMessage for EnterAreaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.LeaveId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeaveId);
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
            7 => self.LeaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterAreaResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Id: i32,
}

impl EnterAreaResponse {
    pub const MSG_ID: u16 = 17162;
}

impl PbMessage for EnterAreaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioState {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub GroupType: Vec<u8>,
    pub State: Vec<u8>,
}

impl PbMessage for AudioState {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if !self.GroupType.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GroupType);
        }
        if !self.State.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
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
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            2 => self.TreeIncId = r.read_varint_i64()?,
            3 => self.GroupType = r.read_len_delimited()?.to_vec(),
            4 => self.State = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BanLogoutInfo {
    pub Reason: i32,
    pub BanEndTime: i64,
}

impl PbMessage for BanLogoutInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Reason != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.BanEndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Reason = r.read_varint_i32()?,
            2 => self.BanEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CardShowEntry {
    pub CardId: i32,
    pub IsRead: bool,
}

impl PbMessage for CardShowEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
        }
        if self.IsRead {
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
            1 => self.CardId = r.read_varint_i32()?,
            2 => self.IsRead = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BasicInfoNotify {
    pub Id: i32,
    pub Attributes: Vec<PlayerAttr>,
    pub MingSuGenInfos: Vec<MingSuGenInfo>,
    pub DragonPoolInfos: Vec<DragonPoolInfo>,
    pub RoleShowList: Vec<RoleShowEntry>,
    pub CurCardId: i32,
    pub Birthday: i32,
    pub CardUnlockList: Vec<CardShowEntry>,
    pub RandomSeed: i32,
    pub DisplayBirthDay: bool,
    pub LastModifyNameTime: i64,
    pub ModifyNameTime: Vec<u8>,
    pub BusinessCompliance: bool,
    pub NewbieGuideV2: bool,
}

impl BasicInfoNotify {
    pub const MSG_ID: u16 = 21993;
}

impl PbMessage for BasicInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Attributes {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.MingSuGenInfos {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.DragonPoolInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.RoleShowList {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.CurCardId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardId);
        }
        if self.Birthday != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Birthday);
        }
        for item in &self.CardUnlockList {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.RandomSeed != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RandomSeed);
        }
        if self.DisplayBirthDay {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LastModifyNameTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastModifyNameTime);
        }
        if !self.ModifyNameTime.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ModifyNameTime);
        }
        if self.BusinessCompliance {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NewbieGuideV2 {
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
            4 => self.Id = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Attributes
                    .push(<PlayerAttr as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.MingSuGenInfos
                    .push(<MingSuGenInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.DragonPoolInfos
                    .push(<DragonPoolInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.RoleShowList
                    .push(<RoleShowEntry as PbMessage>::decode(bytes)?);
            }
            2 => self.CurCardId = r.read_varint_i32()?,
            7 => self.Birthday = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CardUnlockList
                    .push(<CardShowEntry as PbMessage>::decode(bytes)?);
            }
            12 => self.RandomSeed = r.read_varint_i32()?,
            1 => self.DisplayBirthDay = r.read_bool()?,
            10 => self.LastModifyNameTime = r.read_varint_i64()?,
            14 => self.ModifyNameTime = r.read_len_delimited()?.to_vec(),
            5 => self.BusinessCompliance = r.read_bool()?,
            13 => self.NewbieGuideV2 = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyNameRequest {
    pub Name: Vec<u8>,
}

impl ModifyNameRequest {
    pub const MSG_ID: u16 = 18421;
}

impl PbMessage for ModifyNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
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
            4 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyNameResponse {
    pub Name: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
    pub LastModifyNameTime: i64,
    pub ModifyNameTime: Vec<u8>,
}

impl ModifyNameResponse {
    pub const MSG_ID: u16 = 16293;
}

impl PbMessage for ModifyNameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LastModifyNameTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastModifyNameTime);
        }
        if !self.ModifyNameTime.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ModifyNameTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.LastModifyNameTime = r.read_varint_i64()?,
            12 => self.ModifyNameTime = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifySignatureRequest {
    pub Signature: Vec<u8>,
}

impl ModifySignatureRequest {
    pub const MSG_ID: u16 = 16561;
}

impl PbMessage for ModifySignatureRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
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
            14 => self.Signature = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifySignatureResponse {
    pub Signature: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ModifySignatureResponse {
    pub const MSG_ID: u16 = 18895;
}

impl PbMessage for ModifySignatureResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Signature);
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
            10 => self.Signature = r.read_len_delimited()?.to_vec(),
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
pub struct ChangeHeadPhotoRequest {
    pub HeadPhotoId: i32,
}

impl ChangeHeadPhotoRequest {
    pub const MSG_ID: u16 = 26439;
}

impl PbMessage for ChangeHeadPhotoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HeadPhotoId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadPhotoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.HeadPhotoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeHeadPhotoResponse {
    pub HeadPhotoId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeHeadPhotoResponse {
    pub const MSG_ID: u16 = 28817;
}

impl PbMessage for ChangeHeadPhotoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HeadPhotoId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadPhotoId);
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
            8 => self.HeadPhotoId = r.read_varint_i32()?,
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
pub struct ClientBasicInfo {
    pub Platform: Vec<u8>,
    pub DeviceId: Vec<u8>,
    pub NetStatus: Option<NetStatusType>,
    pub Model: Vec<u8>,
    pub CPU: Vec<u8>,
    pub DeviceLevel: Option<ClientDeviceLevel>,
    pub Language: i32,
    pub DistinctId: Vec<u8>,
    pub MacAddress: Vec<u8>,
    pub PkgId: Vec<u8>,
    pub ServerTag: Vec<u8>,
    pub SystemLanguage: Vec<u8>,
    pub OS: Vec<u8>,
    pub DeviceId2ShuShu: Vec<u8>,
    pub ScreenHeight: i32,
    pub ScreenWidth: i32,
    pub DeviceInfo: Vec<u8>,
    pub DriverDate: Vec<u8>,
    pub ClientVersion: Vec<u8>,
    pub OSVersion: Vec<u8>,
    pub QualityVersion: i32,
}

impl PbMessage for ClientBasicInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Platform.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Platform);
        }
        if !self.DeviceId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceId);
        }
        if let Some(v) = &self.NetStatus {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Model.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Model);
        }
        if !self.CPU.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CPU);
        }
        if let Some(v) = &self.DeviceLevel {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Language != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Language);
        }
        if !self.DistinctId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DistinctId);
        }
        if !self.MacAddress.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MacAddress);
        }
        if !self.PkgId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PkgId);
        }
        if !self.ServerTag.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ServerTag);
        }
        if !self.SystemLanguage.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SystemLanguage);
        }
        if !self.OS.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OS);
        }
        if !self.DeviceId2ShuShu.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceId2ShuShu);
        }
        if self.ScreenHeight != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScreenHeight);
        }
        if self.ScreenWidth != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScreenWidth);
        }
        if !self.DeviceInfo.is_empty() {
            crate::wire::write_tag(out, 17, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceInfo);
        }
        if !self.DriverDate.is_empty() {
            crate::wire::write_tag(out, 18, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DriverDate);
        }
        if !self.ClientVersion.is_empty() {
            crate::wire::write_tag(out, 19, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientVersion);
        }
        if !self.OSVersion.is_empty() {
            crate::wire::write_tag(out, 20, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OSVersion);
        }
        if self.QualityVersion != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QualityVersion);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Platform = r.read_len_delimited()?.to_vec(),
            2 => self.DeviceId = r.read_len_delimited()?.to_vec(),
            3 => {
                self.NetStatus = Some(<NetStatusType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.Model = r.read_len_delimited()?.to_vec(),
            5 => self.CPU = r.read_len_delimited()?.to_vec(),
            6 => {
                self.DeviceLevel = Some(
                    <ClientDeviceLevel as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.Language = r.read_varint_i32()?,
            8 => self.DistinctId = r.read_len_delimited()?.to_vec(),
            9 => self.MacAddress = r.read_len_delimited()?.to_vec(),
            10 => self.PkgId = r.read_len_delimited()?.to_vec(),
            11 => self.ServerTag = r.read_len_delimited()?.to_vec(),
            12 => self.SystemLanguage = r.read_len_delimited()?.to_vec(),
            13 => self.OS = r.read_len_delimited()?.to_vec(),
            14 => self.DeviceId2ShuShu = r.read_len_delimited()?.to_vec(),
            15 => self.ScreenHeight = r.read_varint_i32()?,
            16 => self.ScreenWidth = r.read_varint_i32()?,
            17 => self.DeviceInfo = r.read_len_delimited()?.to_vec(),
            18 => self.DriverDate = r.read_len_delimited()?.to_vec(),
            19 => self.ClientVersion = r.read_len_delimited()?.to_vec(),
            20 => self.OSVersion = r.read_len_delimited()?.to_vec(),
            21 => self.QualityVersion = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerBasicInfoGetRequest {
    pub Id: i32,
}

impl PlayerBasicInfoGetRequest {
    pub const MSG_ID: u16 = 23383;
}

impl PbMessage for PlayerBasicInfoGetRequest {
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
pub struct PlayerBasicInfoGetResponse {
    pub Info: Option<PlayerDetails>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerBasicInfoGetResponse {
    pub const MSG_ID: u16 = 19022;
}

impl PbMessage for PlayerBasicInfoGetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 11, v);
        }
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct BirthdayInitRequest {
    pub Birthday: i32,
}

impl BirthdayInitRequest {
    pub const MSG_ID: u16 = 21136;
}

impl PbMessage for BirthdayInitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Birthday != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Birthday);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.Birthday = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthdayInitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BirthdayInitResponse {
    pub const MSG_ID: u16 = 29356;
}

impl PbMessage for BirthdayInitResponse {
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
pub struct RoleShowListUpdateRequest {
    pub RoleList: Vec<i32>,
}

impl RoleShowListUpdateRequest {
    pub const MSG_ID: u16 = 27889;
}

impl PbMessage for RoleShowListUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleShowListUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleShowListUpdateResponse {
    pub const MSG_ID: u16 = 23280;
}

impl PbMessage for RoleShowListUpdateResponse {
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
pub struct ChangeCardRequest {
    pub CardId: i32,
}

impl ChangeCardRequest {
    pub const MSG_ID: u16 = 23459;
}

impl PbMessage for ChangeCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeCardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeCardResponse {
    pub const MSG_ID: u16 = 29040;
}

impl PbMessage for ChangeCardResponse {
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
pub struct ReadCardRequest {
    pub CardId: i32,
}

impl ReadCardRequest {
    pub const MSG_ID: u16 = 15318;
}

impl PbMessage for ReadCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReadCardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ReadCardResponse {
    pub const MSG_ID: u16 = 19779;
}

impl PbMessage for ReadCardResponse {
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
pub struct BirthdayShowSetRequest {
    pub DisPlay: bool,
}

impl BirthdayShowSetRequest {
    pub const MSG_ID: u16 = 27980;
}

impl PbMessage for BirthdayShowSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DisPlay {
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
            7 => self.DisPlay = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthdayShowSetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BirthdayShowSetResponse {
    pub const MSG_ID: u16 = 29672;
}

impl PbMessage for BirthdayShowSetResponse {
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
pub struct PlayerNameUpdateNotify {
    pub Name: Vec<u8>,
    pub LastModifyNameTime: i64,
}

impl PlayerNameUpdateNotify {
    pub const MSG_ID: u16 = 26559;
}

impl PbMessage for PlayerNameUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.LastModifyNameTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastModifyNameTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.Name = r.read_len_delimited()?.to_vec(),
            4 => self.LastModifyNameTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayStationBlockAccountRequest {
    pub BlockedIds: Vec<Vec<u8>>,
}

impl UpdatePlayStationBlockAccountRequest {
    pub const MSG_ID: u16 = 16257;
}

impl PbMessage for UpdatePlayStationBlockAccountRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BlockedIds {
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
            15 => self.BlockedIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayStationBlockAccountResponse {}

impl UpdatePlayStationBlockAccountResponse {
    pub const MSG_ID: u16 = 17327;
}

impl PbMessage for UpdatePlayStationBlockAccountResponse {
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
pub struct PlayerHeadDataRequest {}

impl PlayerHeadDataRequest {
    pub const MSG_ID: u16 = 18199;
}

impl PbMessage for PlayerHeadDataRequest {
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
pub struct PlayerHeadDataResponse {
    pub PlayerHeadDataIds: Vec<i32>,
}

impl PlayerHeadDataResponse {
    pub const MSG_ID: u16 = 22413;
}

impl PbMessage for PlayerHeadDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerHeadDataIds {
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
                &mut self.PlayerHeadDataIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebSignRequest {}

impl WebSignRequest {
    pub const MSG_ID: u16 = 18839;
}

impl PbMessage for WebSignRequest {
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
pub struct WebSignResponse {
    pub NoticeSign: Vec<u8>,
}

impl WebSignResponse {
    pub const MSG_ID: u16 = 26225;
}

impl PbMessage for WebSignResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.NoticeSign.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.NoticeSign);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.NoticeSign = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StorageInfoNotify {
    pub Infos: Vec<ClientStorageInfo>,
}

impl StorageInfoNotify {
    pub const MSG_ID: u16 = 17245;
}

impl PbMessage for StorageInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
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
                self.Infos
                    .push(<ClientStorageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StorageInfoUpdateNotify {
    pub Adds: Vec<ClientStorageInfo>,
    pub Updates: Vec<ClientStorageInfo>,
    pub Removes: Vec<i32>,
}

impl StorageInfoUpdateNotify {
    pub const MSG_ID: u16 = 24810;
}

impl PbMessage for StorageInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Adds {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.Updates {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.Removes {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<ClientStorageInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<ClientStorageInfo as PbMessage>::decode(bytes)?);
            }
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StorageInfoRequest {}

impl StorageInfoRequest {
    pub const MSG_ID: u16 = 20672;
}

impl PbMessage for StorageInfoRequest {
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
pub struct StorageInfoResponse {
    pub Infos: Vec<ClientStorageInfo>,
}

impl StorageInfoResponse {
    pub const MSG_ID: u16 = 15837;
}

impl PbMessage for StorageInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
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
                self.Infos
                    .push(<ClientStorageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StorageInfoUpdateRequest {
    pub Infos: Vec<ClientStorageInfo>,
}

impl StorageInfoUpdateRequest {
    pub const MSG_ID: u16 = 22023;
}

impl PbMessage for StorageInfoUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
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
                self.Infos
                    .push(<ClientStorageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StorageInfoUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl StorageInfoUpdateResponse {
    pub const MSG_ID: u16 = 24640;
}

impl PbMessage for StorageInfoUpdateResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum ClientStorageInfoDataOneof {
    MapMapData(ClientStorageMapMapData),
    MapListData(ClientStorageMapListData),
    MapData(ClientStorageMapData),
    ListData(ClientStorageListData),
    SetData(ClientStorageSetData),
    BoolData(ClientStorageBoolData),
    IntData(ClientStorageIntData),
    LongData(ClientStorageLongData),
    StringData(ClientStorageStringData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageInfo {
    pub Data: Option<ClientStorageInfoDataOneof>,
    pub SystemId: i32,
}

impl PbMessage for ClientStorageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                ClientStorageInfoDataOneof::MapMapData(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                ClientStorageInfoDataOneof::MapListData(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                ClientStorageInfoDataOneof::MapData(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                ClientStorageInfoDataOneof::ListData(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                ClientStorageInfoDataOneof::SetData(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                ClientStorageInfoDataOneof::BoolData(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                ClientStorageInfoDataOneof::IntData(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                ClientStorageInfoDataOneof::LongData(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                ClientStorageInfoDataOneof::StringData(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
            }
        }
        if self.SystemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SystemId);
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
                let val = <ClientStorageMapMapData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::MapMapData(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageMapListData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::MapListData(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageMapData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::MapData(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageListData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::ListData(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageSetData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::SetData(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageBoolData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::BoolData(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageIntData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::IntData(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageLongData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::LongData(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientStorageStringData as PbMessage>::decode(bytes)?;
                self.Data = Some(ClientStorageInfoDataOneof::StringData(val));
            }
            1 => self.SystemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageMapData {
    pub Data: BTreeMap<i32, i32>,
}

impl PbMessage for ClientStorageMapData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Data.is_empty() {
            crate::wire::write_map(out, 1, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Data)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageMapMapData {
    pub Data: BTreeMap<i32, ClientStorageMapData>,
}

impl PbMessage for ClientStorageMapMapData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Data.is_empty() {
            crate::wire::write_map(out, 1, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Data)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageMapListData {
    pub Data: BTreeMap<i32, ClientStorageListData>,
}

impl PbMessage for ClientStorageMapListData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Data.is_empty() {
            crate::wire::write_map(out, 1, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Data)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageListData {
    pub Data: Vec<i32>,
}

impl PbMessage for ClientStorageListData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Data, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageSetData {
    pub Data: Vec<i32>,
}

impl PbMessage for ClientStorageSetData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Data, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageBoolData {
    pub Data: bool,
}

impl PbMessage for ClientStorageBoolData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Data {
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
            1 => self.Data = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageIntData {
    pub Data: i32,
}

impl PbMessage for ClientStorageIntData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Data != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Data = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageLongData {
    pub Data: i64,
}

impl PbMessage for ClientStorageLongData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Data != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Data = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientStorageStringData {
    pub Data: Vec<u8>,
}

impl PbMessage for ClientStorageStringData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Data.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Data = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerXboxBlockListRequest {
    pub XboxAccountIds: Vec<Vec<u8>>,
}

impl PlayerXboxBlockListRequest {
    pub const MSG_ID: u16 = 17705;
}

impl PbMessage for PlayerXboxBlockListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.XboxAccountIds {
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
            14 => self.XboxAccountIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerXboxBlockListResponse {}

impl PlayerXboxBlockListResponse {
    pub const MSG_ID: u16 = 26733;
}

impl PbMessage for PlayerXboxBlockListResponse {
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
pub struct RoleVoice {
    pub RoleId: i32,
    pub VoiceId: i32,
}

impl PbMessage for RoleVoice {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.VoiceId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VoiceId);
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
            2 => self.VoiceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerVoiceLanguageRequest {}

impl PlayerVoiceLanguageRequest {
    pub const MSG_ID: u16 = 22437;
}

impl PbMessage for PlayerVoiceLanguageRequest {
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
pub struct PlayerVoiceLanguageResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleVoices: Vec<RoleVoice>,
}

impl PlayerVoiceLanguageResponse {
    pub const MSG_ID: u16 = 27315;
}

impl PbMessage for PlayerVoiceLanguageResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RoleVoices {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RoleVoices
                    .push(<RoleVoice as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleVoiceSetting {
    pub RoleId: i32,
    pub VoiceLanguage: i32,
}

impl PbMessage for RoleVoiceSetting {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.VoiceLanguage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VoiceLanguage);
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
            2 => self.VoiceLanguage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerRoleVoiceSetRequest {
    pub RoleVoices: Vec<RoleVoiceSetting>,
}

impl PlayerRoleVoiceSetRequest {
    pub const MSG_ID: u16 = 22391;
}

impl PbMessage for PlayerRoleVoiceSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleVoices {
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
                self.RoleVoices
                    .push(<RoleVoiceSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerRoleVoiceSetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerRoleVoiceSetResponse {
    pub const MSG_ID: u16 = 22358;
}

impl PbMessage for PlayerRoleVoiceSetResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAttrValueOneof {
    Int32Value(i32),
    StringValue(Vec<u8>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAttr {
    pub Value: Option<PlayerAttrValueOneof>,
    pub Key: Option<PlayerAttrKey>,
    pub ValueType: Option<PlayerAttrType>,
}

impl PbMessage for PlayerAttr {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Value {
            match oneof {
                PlayerAttrValueOneof::Int32Value(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                PlayerAttrValueOneof::StringValue(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
            }
        }
        if let Some(v) = &self.Key {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ValueType {
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
            3 => self.Value = Some(PlayerAttrValueOneof::Int32Value(r.read_varint_i32()?)),
            4 => {
                self.Value = Some(PlayerAttrValueOneof::StringValue(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            1 => {
                self.Key = Some(<PlayerAttrKey as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.ValueType = Some(
                    <PlayerAttrType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PlayerAttrNotify {
    pub Attributes: Vec<PlayerAttr>,
}

impl PlayerAttrNotify {
    pub const MSG_ID: u16 = 17685;
}

impl PbMessage for PlayerAttrNotify {
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
                    .push(<PlayerAttr as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MingSuGenInfo {
    pub CreatureGenId: i64,
}

impl PbMessage for MingSuGenInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CreatureGenId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CreatureGenId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CreatureGenId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonPoolInfo {
    pub DragonPoolId: i32,
    pub Level: i32,
    pub InjectedCoreItemCount: i32,
    pub LevelGains: i32,
}

impl PbMessage for DragonPoolInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InjectedCoreItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InjectedCoreItemCount);
        }
        if self.LevelGains != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGains);
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
            3 => self.Level = r.read_varint_i32()?,
            4 => self.InjectedCoreItemCount = r.read_varint_i32()?,
            5 => self.LevelGains = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemEntry {
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for ItemEntry {
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
pub struct ItemDict {
    pub Items: Vec<ItemEntry>,
}

impl PbMessage for ItemDict {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
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
                self.Items.push(<ItemEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonPoolDropItems {
    pub DragonPoolId: i32,
    pub DropIds: Vec<i32>,
    pub DropItems: Vec<ItemDict>,
}

impl PbMessage for DragonPoolDropItems {
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
pub struct DarkCoastDeliveryRequest {
    pub DragonPoolId: i32,
}

impl DarkCoastDeliveryRequest {
    pub const MSG_ID: u16 = 20643;
}

impl PbMessage for DarkCoastDeliveryRequest {
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
pub struct DarkCoastDeliveryResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub DragonPoolDropItems: Option<DragonPoolDropItems>,
    pub DefeatedGuard: Vec<i32>,
    pub ReceivedGuardReward: Vec<i32>,
    pub LevelGain: i32,
}

impl DarkCoastDeliveryResponse {
    pub const MSG_ID: u16 = 20081;
}

impl PbMessage for DarkCoastDeliveryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.DragonPoolDropItems {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.DefeatedGuard {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ReceivedGuardReward {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LevelGain != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGain);
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
                self.DragonPoolDropItems
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DefeatedGuard, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ReceivedGuardReward,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => self.LevelGain = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransferContextId {
    pub BulletContextId: i64,
}

impl PbMessage for TransferContextId {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletContextId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletContextId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BulletContextId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbBattlePassReward {
    pub Level: i32,
    pub ItemId: i32,
    pub Type: Option<BattlePassType>,
}

impl PbMessage for PbBattlePassReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if let Some(v) = &self.Type {
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
            1 => self.Level = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => {
                self.Type = Some(
                    <BattlePassType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PbBattlePassRecurringReward {
    pub Type: Option<BattlePassType>,
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for PbBattlePassRecurringReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
            1 => {
                self.Type = Some(
                    <BattlePassType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbBattlePass {
    pub InTimeRange: bool,
    pub Id: i32,
    pub Level: i32,
    pub Exp: i32,
    pub WeeklyTotalExp: i32,
    pub PayStatus: Option<BattlePassPayStatus>,
    pub TakenRewards: Vec<PbBattlePassReward>,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub RecurringRewards: Vec<PbBattlePassRecurringReward>,
    pub HadEnter: bool,
}

impl PbMessage for PbBattlePass {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InTimeRange {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        if self.WeeklyTotalExp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeeklyTotalExp);
        }
        if let Some(v) = &self.PayStatus {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TakenRewards {
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
        for item in &self.RecurringRewards {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.HadEnter {
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
            1 => self.InTimeRange = r.read_bool()?,
            2 => self.Id = r.read_varint_i32()?,
            3 => self.Level = r.read_varint_i32()?,
            4 => self.Exp = r.read_varint_i32()?,
            5 => self.WeeklyTotalExp = r.read_varint_i32()?,
            6 => {
                self.PayStatus = Some(
                    <BattlePassPayStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TakenRewards
                    .push(<PbBattlePassReward as PbMessage>::decode(bytes)?);
            }
            8 => self.BeginTime = r.read_varint_i64()?,
            9 => self.EndTime = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RecurringRewards
                    .push(<PbBattlePassRecurringReward as PbMessage>::decode(bytes)?);
            }
            11 => self.HadEnter = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassRequest {}

impl BattlePassRequest {
    pub const MSG_ID: u16 = 22649;
}

impl PbMessage for BattlePassRequest {
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
pub struct BattlePassResponse {
    pub BattlePass: Option<PbBattlePass>,
    pub ErrorCode: Option<ErrorCode>,
}

impl BattlePassResponse {
    pub const MSG_ID: u16 = 26557;
}

impl PbMessage for BattlePassResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BattlePass {
            crate::wire::write_submessage(out, 2, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BattlePass
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

#[derive(Debug, Clone, PartialEq)]
pub enum NodeInfoExtraInfoOneof {
    ChildQuestNodeInfo(ChildQuestNodeInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeInfo {
    pub ExtraInfo: Option<NodeInfoExtraInfoOneof>,
    pub Status: Option<NodeStatus>,
}

impl PbMessage for NodeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ExtraInfo {
            match oneof {
                NodeInfoExtraInfoOneof::ChildQuestNodeInfo(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
            }
        }
        if let Some(v) = &self.Status {
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
                let val = <ChildQuestNodeInfo as PbMessage>::decode(bytes)?;
                self.ExtraInfo = Some(NodeInfoExtraInfoOneof::ChildQuestNodeInfo(val));
            }
            1 => {
                self.Status = Some(<NodeStatus as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeInfo {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub BtType: Option<BtType>,
    pub BlackboardId: i32,
    pub Nodes: BTreeMap<i32, NodeInfo>,
    pub Vars: BTreeMap<Vec<u8>, VarDefinePb>,
    pub TimerInfos: Vec<TimerInfoPb>,
    pub SuspendType: i32,
    pub OccupationInfo: Vec<OccupationPbInfo>,
    pub AudioState: BTreeMap<Vec<u8>, Vec<u8>>,
    pub IsScreenOccupy: bool,
    pub CharacterLookAtInfos: Vec<CharacterLookAtInfo>,
    pub SuspendNodeId: i32,
}

impl PbMessage for TreeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if let Some(v) = &self.BtType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BlackboardId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlackboardId);
        }
        if !self.Nodes.is_empty() {
            crate::wire::write_map(out, 5, &self.Nodes);
        }
        if !self.Vars.is_empty() {
            crate::wire::write_map(out, 6, &self.Vars);
        }
        for item in &self.TimerInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.SuspendType != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SuspendType);
        }
        for item in &self.OccupationInfo {
            crate::wire::write_submessage(out, 9, item);
        }
        if !self.AudioState.is_empty() {
            crate::wire::write_map(out, 10, &self.AudioState);
        }
        if self.IsScreenOccupy {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.CharacterLookAtInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        if self.SuspendNodeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SuspendNodeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            2 => self.TreeIncId = r.read_varint_i64()?,
            3 => {
                self.BtType = Some(<BtType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.BlackboardId = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.Nodes)?,
            6 => crate::wire::decode_map_entry(r, &mut self.Vars)?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TimerInfos
                    .push(<TimerInfoPb as PbMessage>::decode(bytes)?);
            }
            8 => self.SuspendType = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.OccupationInfo
                    .push(<OccupationPbInfo as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_map_entry(r, &mut self.AudioState)?,
            11 => self.IsScreenOccupy = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.CharacterLookAtInfos
                    .push(<CharacterLookAtInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.SuspendNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BehaviorTreeInfoNotify {
    pub TreeInfos: Vec<TreeInfo>,
}

impl BehaviorTreeInfoNotify {
    pub const MSG_ID: u16 = 16786;
}

impl PbMessage for BehaviorTreeInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TreeInfos {
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
                self.TreeInfos.push(<TreeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BehaviorTreeDeleteNotify {
    pub TreeIncIds: Vec<i64>,
}

impl BehaviorTreeDeleteNotify {
    pub const MSG_ID: u16 = 28831;
}

impl PbMessage for BehaviorTreeDeleteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TreeIncIds {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TreeIncIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChildQuestNodeProgressProgressOneof {
    Kill(KillProgress),
    GetItem(GetItemProgress),
    MonsterCreator(MonsterCreatorProgress),
    UseItem(UseItemProgress),
    levelPlayCount(i32),
    Interact(InteractProgress),
    CompleteInst(CompleteInstProgress),
    EntityStateList(EntityStateProgress),
    GpuMonster(GpuMonsterProgress),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChildQuestNodeProgress {
    pub Progress: Option<ChildQuestNodeProgressProgressOneof>,
}

impl PbMessage for ChildQuestNodeProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Progress {
            match oneof {
                ChildQuestNodeProgressProgressOneof::Kill(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                ChildQuestNodeProgressProgressOneof::GetItem(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                ChildQuestNodeProgressProgressOneof::MonsterCreator(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                ChildQuestNodeProgressProgressOneof::UseItem(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                ChildQuestNodeProgressProgressOneof::levelPlayCount(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                ChildQuestNodeProgressProgressOneof::Interact(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                ChildQuestNodeProgressProgressOneof::CompleteInst(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                ChildQuestNodeProgressProgressOneof::EntityStateList(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                ChildQuestNodeProgressProgressOneof::GpuMonster(v) => {
                    crate::wire::write_submessage(out, 9, v);
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
                let val = <KillProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::Kill(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <GetItemProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::GetItem(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterCreatorProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::MonsterCreator(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseItemProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::UseItem(val));
            }
            5 => {
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::levelPlayCount(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <InteractProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::Interact(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <CompleteInstProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::CompleteInst(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityStateProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::EntityStateList(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <GpuMonsterProgress as PbMessage>::decode(bytes)?;
                self.Progress = Some(ChildQuestNodeProgressProgressOneof::GpuMonster(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GpuMonsterProgress {
    pub CurKillNum: i32,
}

impl PbMessage for GpuMonsterProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurKillNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurKillNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurKillNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityStateProgress {
    pub EntityId: Vec<i32>,
}

impl PbMessage for EntityStateProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChildQuestNodeInfo {
    pub Status: Option<ChildQuestNodeStatus>,
    pub Progress: Option<ChildQuestNodeProgress>,
}

impl PbMessage for ChildQuestNodeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Progress {
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
                self.Status = Some(
                    <ChildQuestNodeStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
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
pub struct KillProgress {
    pub MonId: Vec<i32>,
    pub PrefabNum: i32,
    pub CurrNum: i32,
    pub TotalNum: i32,
}

impl PbMessage for KillProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MonId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PrefabNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PrefabNum);
        }
        if self.CurrNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrNum);
        }
        if self.TotalNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MonId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.PrefabNum = r.read_varint_i32()?,
            3 => self.CurrNum = r.read_varint_i32()?,
            4 => self.TotalNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InteractProgress {
    pub NpcId: Vec<i32>,
}

impl PbMessage for InteractProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NpcId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NpcId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetItemProgress {
    pub Info: Vec<GetItemCount>,
}

impl PbMessage for GetItemProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Info {
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
                self.Info.push(<GetItemCount as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetItemCount {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for GetItemCount {
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
pub struct SceneMonsterCreatedMonsterInfo {
    pub PrefabId: i32,
    pub MapId: i32,
    pub BaseLife: i64,
    pub State: i32,
}

impl PbMessage for SceneMonsterCreatedMonsterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PrefabId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PrefabId);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.BaseLife != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BaseLife);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.PrefabId = r.read_varint_i32()?,
            2 => self.MapId = r.read_varint_i32()?,
            3 => self.BaseLife = r.read_varint_i64()?,
            4 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterCreatorProgress {
    pub Slots: Vec<MonsterCreatorProgressSlot>,
    pub TotalNum: i32,
}

impl PbMessage for MonsterCreatorProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Slots {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.TotalNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalNum);
        }
    }

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
                    .push(<MonsterCreatorProgressSlot as PbMessage>::decode(bytes)?);
            }
            2 => self.TotalNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterCreatorProgressSlot {
    pub WaveId: i32,
    pub KillMonIds: Vec<i32>,
    pub CurrentWaveEndTime: i32,
    pub SpawnStepType: i32,
    pub CreatorEntityConfigId: i32,
    pub MonsterInfo: Vec<SceneMonsterCreatedMonsterInfo>,
}

impl PbMessage for MonsterCreatorProgressSlot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
        for item in &self.KillMonIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CurrentWaveEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentWaveEndTime);
        }
        if self.SpawnStepType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpawnStepType);
        }
        if self.CreatorEntityConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CreatorEntityConfigId);
        }
        for item in &self.MonsterInfo {
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
            1 => self.WaveId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.KillMonIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.CurrentWaveEndTime = r.read_varint_i32()?,
            4 => self.SpawnStepType = r.read_varint_i32()?,
            5 => self.CreatorEntityConfigId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MonsterInfo
                    .push(<SceneMonsterCreatedMonsterInfo as PbMessage>::decode(
                        bytes,
                    )?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseItemProgress {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for UseItemProgress {
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
pub struct CompleteInstProgress {
    pub InstId: i32,
    pub Count: i32,
}

impl PbMessage for CompleteInstProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateNodeProgressNotify {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub NodeId: i32,
    pub Progress: Option<ChildQuestNodeProgress>,
}

impl UpdateNodeProgressNotify {
    pub const MSG_ID: u16 = 21488;
}

impl PbMessage for UpdateNodeProgressNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if let Some(v) = &self.Progress {
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
            11 => self.TreeOwnerId = r.read_varint_i32()?,
            7 => self.TreeIncId = r.read_varint_i64()?,
            14 => self.NodeId = r.read_varint_i32()?,
            6 => {
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
pub struct UpdateChildQuestNodeStatusNotify {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub NodeId: i32,
    pub Status: Option<ChildQuestNodeStatus>,
}

impl UpdateChildQuestNodeStatusNotify {
    pub const MSG_ID: u16 = 22070;
}

impl PbMessage for UpdateChildQuestNodeStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if let Some(v) = &self.Status {
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
            10 => self.TreeOwnerId = r.read_varint_i32()?,
            6 => self.TreeIncId = r.read_varint_i64()?,
            11 => self.NodeId = r.read_varint_i32()?,
            14 => {
                self.Status = Some(
                    <ChildQuestNodeStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct UpdateNodeStatusNotify {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub NodeId: i32,
    pub Status: Option<NodeStatus>,
}

impl UpdateNodeStatusNotify {
    pub const MSG_ID: u16 = 23007;
}

impl PbMessage for UpdateNodeStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if let Some(v) = &self.Status {
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
            5 => self.TreeOwnerId = r.read_varint_i32()?,
            4 => self.TreeIncId = r.read_varint_i64()?,
            2 => self.NodeId = r.read_varint_i32()?,
            3 => {
                self.Status = Some(<NodeStatus as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OccupationPbInfo {
    pub ResourceName: Vec<u8>,
    pub NodeId: i32,
    pub IncId: i64,
}

impl PbMessage for OccupationPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ResourceName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourceName);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.ResourceName = r.read_len_delimited()?.to_vec(),
            2 => self.NodeId = r.read_varint_i32()?,
            3 => self.IncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimerInfoPb {
    pub TimerType: Vec<u8>,
    pub NodeId: i32,
    pub EndTime: i64,
    pub PauseTime: i64,
}

impl PbMessage for TimerInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TimerType.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TimerType);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.PauseTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PauseTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TimerType = r.read_len_delimited()?.to_vec(),
            2 => self.NodeId = r.read_varint_i32()?,
            3 => self.EndTime = r.read_varint_i64()?,
            4 => self.PauseTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterLookAtInfoTargetIdOneof {
    TargetEntityId(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterLookAtInfoTargetPossitionOneof {
    TargetPos(Vector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterLookAtInfo {
    pub TargetId: Option<CharacterLookAtInfoTargetIdOneof>,
    pub TargetPossition: Option<CharacterLookAtInfoTargetPossitionOneof>,
    pub EntityId: i32,
    pub TargetType: i32,
}

impl PbMessage for CharacterLookAtInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.TargetId {
            match oneof {
                CharacterLookAtInfoTargetIdOneof::TargetEntityId(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.TargetPossition {
            match oneof {
                CharacterLookAtInfoTargetPossitionOneof::TargetPos(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        if self.TargetType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetType);
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
                self.TargetId = Some(CharacterLookAtInfoTargetIdOneof::TargetEntityId(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.TargetPossition =
                    Some(CharacterLookAtInfoTargetPossitionOneof::TargetPos(val));
            }
            1 => self.EntityId = r.read_varint_i32()?,
            2 => self.TargetType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightBuffInformation {
    pub HandleId: i32,
    pub BuffId: i64,
    pub Level: i32,
    pub StackCount: i32,
    pub InstigatorId: i64,
    pub EntityId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub Duration: f32,
    pub LeftDuration: f32,
    pub Context: Vec<FightBuffEffectContext>,
    pub IsActive: bool,
    pub ServerId: i32,
    pub MessageId: i64,
    pub ConfBuffId: i64,
}

impl PbMessage for FightBuffInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
        }
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Duration != 0.0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Duration).to_bits().to_le_bytes());
        }
        if self.LeftDuration != 0.0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.LeftDuration).to_bits().to_le_bytes());
        }
        for item in &self.Context {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.IsActive {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.MessageId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MessageId);
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
            1 => self.HandleId = r.read_varint_i32()?,
            2 => self.BuffId = r.read_varint_i64()?,
            3 => self.Level = r.read_varint_i32()?,
            4 => self.StackCount = r.read_varint_i32()?,
            5 => self.InstigatorId = r.read_varint_i64()?,
            6 => self.EntityId = r.read_varint_i64()?,
            7 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.Duration = r.read_f32()?,
            9 => self.LeftDuration = r.read_f32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Context
                    .push(<FightBuffEffectContext as PbMessage>::decode(bytes)?);
            }
            11 => self.IsActive = r.read_bool()?,
            12 => self.ServerId = r.read_varint_i32()?,
            13 => self.MessageId = r.read_varint_i64()?,
            14 => self.ConfBuffId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityPbD3sOneof {
    Camp(i32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPb {
    pub d3s: Option<EntityPbD3sOneof>,
    pub Id: i64,
    pub ConfigId: i32,
    pub ConfigType: Option<EntityConfigType>,
    pub EntityType: Option<EEntityType>,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub InitPos: Option<Vector>,
    pub LivingStatus: Option<LivingStatus>,
    pub IsVisible: bool,
    pub PlayerId: i32,
    pub ComponentPbs: Vec<EntityComponentPb>,
    pub DurabilityValue: i32,
    pub EntityState: Option<EntityState>,
    pub InitLinearVelocity: Option<Vector>,
    pub IsPosAbnormal: bool,
    pub PrefabId: i32,
    pub PrefabIncId: i64,
    pub SubEntityType: i32,
    pub OwnerIncId: i64,
    pub Gravity: Option<Vector>,
    pub RoleSkinId: i32,
    pub IsActorVisible: bool,
    pub SoarWingSkinId: i32,
    pub ParaglidingSkinId: i32,
    pub IsSnapLocation: bool,
}

impl PbMessage for EntityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.d3s {
            match oneof {
                EntityPbD3sOneof::Camp(v) => {
                    crate::wire::write_tag(out, 20, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if let Some(v) = &self.ConfigType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.EntityType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.InitPos {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.LivingStatus {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.ComponentPbs {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.DurabilityValue != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DurabilityValue);
        }
        if let Some(v) = &self.EntityState {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InitLinearVelocity {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.IsPosAbnormal {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PrefabId != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PrefabId);
        }
        if self.PrefabIncId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PrefabIncId);
        }
        if self.SubEntityType != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubEntityType);
        }
        if self.OwnerIncId != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerIncId);
        }
        if let Some(v) = &self.Gravity {
            crate::wire::write_submessage(out, 22, v);
        }
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 23, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.IsActorVisible {
            crate::wire::write_tag(out, 24, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SoarWingSkinId != 0 {
            crate::wire::write_tag(out, 25, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SoarWingSkinId);
        }
        if self.ParaglidingSkinId != 0 {
            crate::wire::write_tag(out, 26, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ParaglidingSkinId);
        }
        if self.IsSnapLocation {
            crate::wire::write_tag(out, 27, WireType::Varint);
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
            20 => self.d3s = Some(EntityPbD3sOneof::Camp(r.read_varint_i32()?)),
            1 => self.Id = r.read_varint_i64()?,
            2 => self.ConfigId = r.read_varint_i32()?,
            3 => {
                self.ConfigType = Some(
                    <EntityConfigType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                self.EntityType = Some(<EEntityType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.InitPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            9 => self.IsVisible = r.read_bool()?,
            10 => self.PlayerId = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ComponentPbs
                    .push(<EntityComponentPb as PbMessage>::decode(bytes)?);
            }
            12 => self.DurabilityValue = r.read_varint_i32()?,
            13 => {
                self.EntityState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.InitLinearVelocity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.IsPosAbnormal = r.read_bool()?,
            17 => self.PrefabId = r.read_varint_i32()?,
            18 => self.PrefabIncId = r.read_varint_i64()?,
            19 => self.SubEntityType = r.read_varint_i32()?,
            21 => self.OwnerIncId = r.read_varint_i64()?,
            22 => {
                let bytes = r.read_len_delimited()?;
                self.Gravity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            23 => self.RoleSkinId = r.read_varint_i32()?,
            24 => self.IsActorVisible = r.read_bool()?,
            25 => self.SoarWingSkinId = r.read_varint_i32()?,
            26 => self.ParaglidingSkinId = r.read_varint_i32()?,
            27 => self.IsSnapLocation = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityComponentPbComponentPbOneof {
    AttributeComponent(AttributeComponentPb),
    TagComponent(TagComponentPb),
    TriggerComponent(TriggerComponentPb),
    SummonerComponent(SummonerComponentPb),
    PartComponent(PartComponentPb),
    VisionSkillComponent(VisionSkillComponentPb),
    AnimationStateComponent(AnimationStateComponentPb),
    BlackboardParamComponent(BlackboardParamComponentPb),
    SysBuffComponent(SysBuffComponentPb),
    ClientDataComponent(ClientDataComponentPb),
    MonsterWeaponComponentPb(MonsterWeaponComponentPb),
    MonsterAiComponentPb(MonsterAiComponentPb),
    FightBuffComponent(FightBuffComponentPb),
    NearbyTrackingComponentPb(NearbyTrackingComponentPb),
    DropComponentPb(DropComponentPb),
    MonsterCaptureComponent(MonsterCaptureComponentPb),
    LogicStateComponentPb(LogicStateComponentPb),
    AdviceComponentPb(AdviceComponentPb),
    LiftComponentPb(LiftComponentPb),
    InteractComponent(InteractComponentPb),
    EquipComponent(EquipComponentPb),
    BeControlledComponentPb(BeControlledComponentPb),
    ConcomitantsComponentPb(ConcomitantsComponentPb),
    TimelineTrackComponentPb(TimelineTrackComponentPb),
    SummonsComponentPb(SummonsComponentPb),
    EntityFsmComponentPb(EntityFsmComponentPb),
    BoardPb(BoardPb),
    PlacementItemPb(PlacementItemPb),
    StateTagComponentPb(StateTagComponentPb),
    MonsterGachaDataPb(MonsterGachaDataPb),
    FanComponentPb(FanComponentPb),
    NpcPb(NpcPb),
    BubbleComponent(BubbleComponentPb),
    PatrolComponent(PatrolComponentPb),
    RangeComponent(RangeComponentPb),
    PassiveSkillComponentPb(PassiveSkillComponentPb),
    PassiveGaSkillComponentPb(PassiveGaSkillComponentPb),
    DynAttachComponentPb(DynAttachComponentPb),
    EntityVarComponentPb(EntityVarComponentPb),
    FollowShooterComponentPb(FollowShooterComponentPb),
    StateComponentPb(StateComponentPb),
    BulletComponentPb(BulletComponentPb),
    BuffProducerComponentPb(BuffProducerComponentPb),
    BuffConsumerComponentPb(BuffConsumerComponentPb),
    SceneItemComponentPb(SceneItemComponentPb),
    ShieldComponentPb(ShieldComponentPb),
    NPCPerformGroupComponentPb(NPCPerformGroupComponentPb),
    PlayerSceneComponentPb(PlayerSceneComponentPb),
    JigsawBaseComponentPb(JigsawBaseComponentPb),
    RoleRecordComponentPb(RoleRecordComponentPb),
    FollowerComponentPb(FollowerComponentPb),
    AttributesIdsComponentPb(AttributesIdsComponentPb),
    PullingFoundationComponentPb(PullingFoundationComponentPb),
    BatchBulletCastComponentPb(BatchBulletCastComponentPb),
    WeaponSkinComponentPb(WeaponSkinComponentPb),
    CharacterAttachComponentPb(CharacterAttachComponentPb),
    PatrolInfoComponentPb(PatrolInfoComponentPb),
    AnimalPerformComponentPb(AnimalPerformComponentPb),
    NpcDriveVehicleComponentPb(NpcDriveVehicleComponentPb),
    GrapplingHookPointComponentPb(GrapplingHookPointComponentPb),
    HackingComponentPb(HackingComponentPb),
    HackTargetComponentPb(HackTargetComponentPb),
    GravityFlipComponent(GravityFlipComponent),
    EntityMoveSplineComponentPb(EntityMoveSplineComponentPb),
    EntityRewardItemPb(EntityRewardItemPb),
    TemplateEntitySpawnerComponentPb(TemplateEntitySpawnerComponentPb),
    GridObjectComponentPb(GridObjectComponentPb),
    SimpleCombatComponentPb(SimpleCombatComponentPb),
    TrapDefenseComponentPb(TrapDefenseComponentPb),
    HoldHandComponentPb(HoldHandComponentPb),
    SceneItemEventListenerComponentPb(SceneItemEventListenerComponentPb),
    ActivityComponentPb(ActivityComponentPb),
    CalabashSkinComponentPb(CalabashSkinComponentPb),
    HonamiStoryDropItemComponentPb(HonamiStoryDropItemComponentPb),
    HonamiStoryEnhanceLevelComponentPb(HonamiStoryEnhanceLevelComponentPb),
    MoveToPointComponentPb(MoveToPointComponentPb),
    RbBlockComponentPb(RbBlockComponentPb),
    SpiritGearComponentPb(SunSpiritGearComponentPb),
    VehiclePb(VehiclePb),
    RbFloorComponentPb(RbFloorComponentPb),
    RbItemComponentPb(RbItemComponentPb),
    RoadNetworkComponentPb(RoadNetworkComponentPb),
    FollowEntityComponentPb(FollowEntityComponentPb),
    MotorOutlookComponentPb(MotorDiyEquippedPb),
    MotorDaCtxComponentPb(MotorDaCtxComponentPb),
    ExhibitionComponentPb(ExhibitionComponentPb),
    FurnitureComponentPb(FurnitureComponentPb),
    OrnamentComponentPb(OrnamentComponentPb),
    FlowerPollutionComponentPb(FlowerPollutionComponentPb),
    DollGrabMachineComponentPb(DollGrabMachineComponentPb),
    DollGrabShowcaseComponentPb(DollGrabShowcaseComponentPb),
    GpuEntityComponentPb(GpuEntityComponentPb),
    RoverRoguePortalComponentPb(RoverRoguePortalComponentPb),
    ElementalHolderComponentPb(ElementalHolderComponentPb),
    SkillAvatarComponentPb(SkillAvatarComponentPb),
    EntityFoundationComponentPb(EntityFoundationComponentPb),
    SquadPropComponentPb(SquadPropComponentPb),
    ConnectorComponentPb(ConnectorComponentPb),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityComponentPb {
    pub ComponentPb: Option<EntityComponentPbComponentPbOneof>,
}

impl PbMessage for EntityComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ComponentPb {
            match oneof {
                EntityComponentPbComponentPbOneof::AttributeComponent(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                EntityComponentPbComponentPbOneof::TagComponent(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                EntityComponentPbComponentPbOneof::TriggerComponent(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                EntityComponentPbComponentPbOneof::SummonerComponent(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                EntityComponentPbComponentPbOneof::PartComponent(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                EntityComponentPbComponentPbOneof::VisionSkillComponent(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                EntityComponentPbComponentPbOneof::AnimationStateComponent(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                EntityComponentPbComponentPbOneof::BlackboardParamComponent(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                EntityComponentPbComponentPbOneof::SysBuffComponent(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                EntityComponentPbComponentPbOneof::ClientDataComponent(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                EntityComponentPbComponentPbOneof::MonsterWeaponComponentPb(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                EntityComponentPbComponentPbOneof::MonsterAiComponentPb(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                EntityComponentPbComponentPbOneof::FightBuffComponent(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                EntityComponentPbComponentPbOneof::NearbyTrackingComponentPb(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                EntityComponentPbComponentPbOneof::DropComponentPb(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                EntityComponentPbComponentPbOneof::MonsterCaptureComponent(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                EntityComponentPbComponentPbOneof::LogicStateComponentPb(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                EntityComponentPbComponentPbOneof::AdviceComponentPb(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                EntityComponentPbComponentPbOneof::LiftComponentPb(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                EntityComponentPbComponentPbOneof::InteractComponent(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                EntityComponentPbComponentPbOneof::EquipComponent(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                EntityComponentPbComponentPbOneof::BeControlledComponentPb(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                EntityComponentPbComponentPbOneof::ConcomitantsComponentPb(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                EntityComponentPbComponentPbOneof::TimelineTrackComponentPb(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                EntityComponentPbComponentPbOneof::SummonsComponentPb(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                EntityComponentPbComponentPbOneof::EntityFsmComponentPb(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                EntityComponentPbComponentPbOneof::BoardPb(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                EntityComponentPbComponentPbOneof::PlacementItemPb(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                EntityComponentPbComponentPbOneof::StateTagComponentPb(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                EntityComponentPbComponentPbOneof::MonsterGachaDataPb(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                EntityComponentPbComponentPbOneof::FanComponentPb(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                EntityComponentPbComponentPbOneof::NpcPb(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                EntityComponentPbComponentPbOneof::BubbleComponent(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                EntityComponentPbComponentPbOneof::PatrolComponent(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                EntityComponentPbComponentPbOneof::RangeComponent(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                EntityComponentPbComponentPbOneof::PassiveSkillComponentPb(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                EntityComponentPbComponentPbOneof::PassiveGaSkillComponentPb(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                EntityComponentPbComponentPbOneof::DynAttachComponentPb(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                EntityComponentPbComponentPbOneof::EntityVarComponentPb(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                EntityComponentPbComponentPbOneof::FollowShooterComponentPb(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                EntityComponentPbComponentPbOneof::StateComponentPb(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                EntityComponentPbComponentPbOneof::BulletComponentPb(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                EntityComponentPbComponentPbOneof::BuffProducerComponentPb(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                EntityComponentPbComponentPbOneof::BuffConsumerComponentPb(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                EntityComponentPbComponentPbOneof::SceneItemComponentPb(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                EntityComponentPbComponentPbOneof::ShieldComponentPb(v) => {
                    crate::wire::write_submessage(out, 48, v);
                }
                EntityComponentPbComponentPbOneof::NPCPerformGroupComponentPb(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                EntityComponentPbComponentPbOneof::PlayerSceneComponentPb(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                EntityComponentPbComponentPbOneof::JigsawBaseComponentPb(v) => {
                    crate::wire::write_submessage(out, 51, v);
                }
                EntityComponentPbComponentPbOneof::RoleRecordComponentPb(v) => {
                    crate::wire::write_submessage(out, 52, v);
                }
                EntityComponentPbComponentPbOneof::FollowerComponentPb(v) => {
                    crate::wire::write_submessage(out, 53, v);
                }
                EntityComponentPbComponentPbOneof::AttributesIdsComponentPb(v) => {
                    crate::wire::write_submessage(out, 54, v);
                }
                EntityComponentPbComponentPbOneof::PullingFoundationComponentPb(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                EntityComponentPbComponentPbOneof::BatchBulletCastComponentPb(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                EntityComponentPbComponentPbOneof::WeaponSkinComponentPb(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                EntityComponentPbComponentPbOneof::CharacterAttachComponentPb(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                EntityComponentPbComponentPbOneof::PatrolInfoComponentPb(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                EntityComponentPbComponentPbOneof::AnimalPerformComponentPb(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                EntityComponentPbComponentPbOneof::NpcDriveVehicleComponentPb(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                EntityComponentPbComponentPbOneof::GrapplingHookPointComponentPb(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                EntityComponentPbComponentPbOneof::HackingComponentPb(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                EntityComponentPbComponentPbOneof::HackTargetComponentPb(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                EntityComponentPbComponentPbOneof::GravityFlipComponent(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                EntityComponentPbComponentPbOneof::EntityMoveSplineComponentPb(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                EntityComponentPbComponentPbOneof::EntityRewardItemPb(v) => {
                    crate::wire::write_submessage(out, 67, v);
                }
                EntityComponentPbComponentPbOneof::TemplateEntitySpawnerComponentPb(v) => {
                    crate::wire::write_submessage(out, 68, v);
                }
                EntityComponentPbComponentPbOneof::GridObjectComponentPb(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                EntityComponentPbComponentPbOneof::SimpleCombatComponentPb(v) => {
                    crate::wire::write_submessage(out, 70, v);
                }
                EntityComponentPbComponentPbOneof::TrapDefenseComponentPb(v) => {
                    crate::wire::write_submessage(out, 71, v);
                }
                EntityComponentPbComponentPbOneof::HoldHandComponentPb(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
                EntityComponentPbComponentPbOneof::SceneItemEventListenerComponentPb(v) => {
                    crate::wire::write_submessage(out, 73, v);
                }
                EntityComponentPbComponentPbOneof::ActivityComponentPb(v) => {
                    crate::wire::write_submessage(out, 74, v);
                }
                EntityComponentPbComponentPbOneof::CalabashSkinComponentPb(v) => {
                    crate::wire::write_submessage(out, 75, v);
                }
                EntityComponentPbComponentPbOneof::HonamiStoryDropItemComponentPb(v) => {
                    crate::wire::write_submessage(out, 76, v);
                }
                EntityComponentPbComponentPbOneof::HonamiStoryEnhanceLevelComponentPb(v) => {
                    crate::wire::write_submessage(out, 77, v);
                }
                EntityComponentPbComponentPbOneof::MoveToPointComponentPb(v) => {
                    crate::wire::write_submessage(out, 78, v);
                }
                EntityComponentPbComponentPbOneof::RbBlockComponentPb(v) => {
                    crate::wire::write_submessage(out, 79, v);
                }
                EntityComponentPbComponentPbOneof::SpiritGearComponentPb(v) => {
                    crate::wire::write_submessage(out, 80, v);
                }
                EntityComponentPbComponentPbOneof::VehiclePb(v) => {
                    crate::wire::write_submessage(out, 81, v);
                }
                EntityComponentPbComponentPbOneof::RbFloorComponentPb(v) => {
                    crate::wire::write_submessage(out, 82, v);
                }
                EntityComponentPbComponentPbOneof::RbItemComponentPb(v) => {
                    crate::wire::write_submessage(out, 83, v);
                }
                EntityComponentPbComponentPbOneof::RoadNetworkComponentPb(v) => {
                    crate::wire::write_submessage(out, 84, v);
                }
                EntityComponentPbComponentPbOneof::FollowEntityComponentPb(v) => {
                    crate::wire::write_submessage(out, 85, v);
                }
                EntityComponentPbComponentPbOneof::MotorOutlookComponentPb(v) => {
                    crate::wire::write_submessage(out, 86, v);
                }
                EntityComponentPbComponentPbOneof::MotorDaCtxComponentPb(v) => {
                    crate::wire::write_submessage(out, 87, v);
                }
                EntityComponentPbComponentPbOneof::ExhibitionComponentPb(v) => {
                    crate::wire::write_submessage(out, 88, v);
                }
                EntityComponentPbComponentPbOneof::FurnitureComponentPb(v) => {
                    crate::wire::write_submessage(out, 89, v);
                }
                EntityComponentPbComponentPbOneof::OrnamentComponentPb(v) => {
                    crate::wire::write_submessage(out, 90, v);
                }
                EntityComponentPbComponentPbOneof::FlowerPollutionComponentPb(v) => {
                    crate::wire::write_submessage(out, 91, v);
                }
                EntityComponentPbComponentPbOneof::DollGrabMachineComponentPb(v) => {
                    crate::wire::write_submessage(out, 92, v);
                }
                EntityComponentPbComponentPbOneof::DollGrabShowcaseComponentPb(v) => {
                    crate::wire::write_submessage(out, 93, v);
                }
                EntityComponentPbComponentPbOneof::GpuEntityComponentPb(v) => {
                    crate::wire::write_submessage(out, 94, v);
                }
                EntityComponentPbComponentPbOneof::RoverRoguePortalComponentPb(v) => {
                    crate::wire::write_submessage(out, 95, v);
                }
                EntityComponentPbComponentPbOneof::ElementalHolderComponentPb(v) => {
                    crate::wire::write_submessage(out, 97, v);
                }
                EntityComponentPbComponentPbOneof::SkillAvatarComponentPb(v) => {
                    crate::wire::write_submessage(out, 98, v);
                }
                EntityComponentPbComponentPbOneof::EntityFoundationComponentPb(v) => {
                    crate::wire::write_submessage(out, 99, v);
                }
                EntityComponentPbComponentPbOneof::SquadPropComponentPb(v) => {
                    crate::wire::write_submessage(out, 100, v);
                }
                EntityComponentPbComponentPbOneof::ConnectorComponentPb(v) => {
                    crate::wire::write_submessage(out, 103, v);
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
                let val = <AttributeComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::AttributeComponent(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <TagComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::TagComponent(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <TriggerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::TriggerComponent(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <SummonerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::SummonerComponent(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::PartComponent(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <VisionSkillComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::VisionSkillComponent(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::AnimationStateComponent(val),
                );
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <BlackboardParamComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::BlackboardParamComponent(val),
                );
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <SysBuffComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::SysBuffComponent(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientDataComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::ClientDataComponent(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterWeaponComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::MonsterWeaponComponentPb(val),
                );
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterAiComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::MonsterAiComponentPb(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <FightBuffComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::FightBuffComponent(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <NearbyTrackingComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::NearbyTrackingComponentPb(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <DropComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::DropComponentPb(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterCaptureComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::MonsterCaptureComponent(val),
                );
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <LogicStateComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::LogicStateComponentPb(
                    val,
                ));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <AdviceComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::AdviceComponentPb(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <LiftComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::LiftComponentPb(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <InteractComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::InteractComponent(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <EquipComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::EquipComponent(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <BeControlledComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::BeControlledComponentPb(val),
                );
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <ConcomitantsComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::ConcomitantsComponentPb(val),
                );
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <TimelineTrackComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::TimelineTrackComponentPb(val),
                );
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <SummonsComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::SummonsComponentPb(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityFsmComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::EntityFsmComponentPb(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <BoardPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::BoardPb(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlacementItemPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::PlacementItemPb(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <StateTagComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::StateTagComponentPb(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterGachaDataPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::MonsterGachaDataPb(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <FanComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::FanComponentPb(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <NpcPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::NpcPb(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <BubbleComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::BubbleComponent(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <PatrolComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::PatrolComponent(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <RangeComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RangeComponent(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::PassiveSkillComponentPb(val),
                );
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveGaSkillComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::PassiveGaSkillComponentPb(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <DynAttachComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::DynAttachComponentPb(val));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityVarComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::EntityVarComponentPb(val));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <FollowShooterComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::FollowShooterComponentPb(val),
                );
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <StateComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::StateComponentPb(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <BulletComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::BulletComponentPb(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffProducerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::BuffProducerComponentPb(val),
                );
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffConsumerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::BuffConsumerComponentPb(val),
                );
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::SceneItemComponentPb(val));
            }
            48 => {
                let bytes = r.read_len_delimited()?;
                let val = <ShieldComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::ShieldComponentPb(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <NPCPerformGroupComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::NPCPerformGroupComponentPb(val));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlayerSceneComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::PlayerSceneComponentPb(
                    val,
                ));
            }
            51 => {
                let bytes = r.read_len_delimited()?;
                let val = <JigsawBaseComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::JigsawBaseComponentPb(
                    val,
                ));
            }
            52 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleRecordComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RoleRecordComponentPb(
                    val,
                ));
            }
            53 => {
                let bytes = r.read_len_delimited()?;
                let val = <FollowerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::FollowerComponentPb(val));
            }
            54 => {
                let bytes = r.read_len_delimited()?;
                let val = <AttributesIdsComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::AttributesIdsComponentPb(val),
                );
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <PullingFoundationComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::PullingFoundationComponentPb(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <BatchBulletCastComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::BatchBulletCastComponentPb(val));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <WeaponSkinComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::WeaponSkinComponentPb(
                    val,
                ));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterAttachComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::CharacterAttachComponentPb(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <PatrolInfoComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::PatrolInfoComponentPb(
                    val,
                ));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimalPerformComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::AnimalPerformComponentPb(val),
                );
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <NpcDriveVehicleComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::NpcDriveVehicleComponentPb(val));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <GrapplingHookPointComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::GrapplingHookPointComponentPb(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <HackingComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::HackingComponentPb(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <HackTargetComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::HackTargetComponentPb(
                    val,
                ));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <GravityFlipComponent as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::GravityFlipComponent(val));
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityMoveSplineComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::EntityMoveSplineComponentPb(val));
            }
            67 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityRewardItemPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::EntityRewardItemPb(val));
            }
            68 => {
                let bytes = r.read_len_delimited()?;
                let val = <TemplateEntitySpawnerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::TemplateEntitySpawnerComponentPb(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <GridObjectComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::GridObjectComponentPb(
                    val,
                ));
            }
            70 => {
                let bytes = r.read_len_delimited()?;
                let val = <SimpleCombatComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::SimpleCombatComponentPb(val),
                );
            }
            71 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::TrapDefenseComponentPb(
                    val,
                ));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <HoldHandComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::HoldHandComponentPb(val));
            }
            73 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemEventListenerComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::SceneItemEventListenerComponentPb(val));
            }
            74 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::ActivityComponentPb(val));
            }
            75 => {
                let bytes = r.read_len_delimited()?;
                let val = <CalabashSkinComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::CalabashSkinComponentPb(val),
                );
            }
            76 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryDropItemComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::HonamiStoryDropItemComponentPb(val));
            }
            77 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryEnhanceLevelComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::HonamiStoryEnhanceLevelComponentPb(val),
                );
            }
            78 => {
                let bytes = r.read_len_delimited()?;
                let val = <MoveToPointComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::MoveToPointComponentPb(
                    val,
                ));
            }
            79 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbBlockComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RbBlockComponentPb(val));
            }
            80 => {
                let bytes = r.read_len_delimited()?;
                let val = <SunSpiritGearComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::SpiritGearComponentPb(
                    val,
                ));
            }
            81 => {
                let bytes = r.read_len_delimited()?;
                let val = <VehiclePb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::VehiclePb(val));
            }
            82 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbFloorComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RbFloorComponentPb(val));
            }
            83 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbItemComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RbItemComponentPb(val));
            }
            84 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoadNetworkComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::RoadNetworkComponentPb(
                    val,
                ));
            }
            85 => {
                let bytes = r.read_len_delimited()?;
                let val = <FollowEntityComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::FollowEntityComponentPb(val),
                );
            }
            86 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorDiyEquippedPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(
                    EntityComponentPbComponentPbOneof::MotorOutlookComponentPb(val),
                );
            }
            87 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorDaCtxComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::MotorDaCtxComponentPb(
                    val,
                ));
            }
            88 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExhibitionComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::ExhibitionComponentPb(
                    val,
                ));
            }
            89 => {
                let bytes = r.read_len_delimited()?;
                let val = <FurnitureComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::FurnitureComponentPb(val));
            }
            90 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrnamentComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::OrnamentComponentPb(val));
            }
            91 => {
                let bytes = r.read_len_delimited()?;
                let val = <FlowerPollutionComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::FlowerPollutionComponentPb(val));
            }
            92 => {
                let bytes = r.read_len_delimited()?;
                let val = <DollGrabMachineComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::DollGrabMachineComponentPb(val));
            }
            93 => {
                let bytes = r.read_len_delimited()?;
                let val = <DollGrabShowcaseComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::DollGrabShowcaseComponentPb(val));
            }
            94 => {
                let bytes = r.read_len_delimited()?;
                let val = <GpuEntityComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::GpuEntityComponentPb(val));
            }
            95 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoverRoguePortalComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::RoverRoguePortalComponentPb(val));
            }
            97 => {
                let bytes = r.read_len_delimited()?;
                let val = <ElementalHolderComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::ElementalHolderComponentPb(val));
            }
            98 => {
                let bytes = r.read_len_delimited()?;
                let val = <SkillAvatarComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb = Some(EntityComponentPbComponentPbOneof::SkillAvatarComponentPb(
                    val,
                ));
            }
            99 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityFoundationComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::EntityFoundationComponentPb(val));
            }
            100 => {
                let bytes = r.read_len_delimited()?;
                let val = <SquadPropComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::SquadPropComponentPb(val));
            }
            103 => {
                let bytes = r.read_len_delimited()?;
                let val = <ConnectorComponentPb as PbMessage>::decode(bytes)?;
                self.ComponentPb =
                    Some(EntityComponentPbComponentPbOneof::ConnectorComponentPb(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillAvatarComponentPb {
    pub AvatarIds: Vec<i32>,
    pub AvatarEntityIds: Vec<i64>,
}

impl PbMessage for SkillAvatarComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AvatarIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AvatarEntityIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AvatarIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AvatarEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRoguePortalComponentPb {
    pub RewardType: i32,
    pub BlessRoleId: i32,
}

impl PbMessage for RoverRoguePortalComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardType);
        }
        if self.BlessRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlessRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RewardType = r.read_varint_i32()?,
            2 => self.BlessRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDaCtxComponentPb {
    pub MotorDaCtxId: i64,
}

impl PbMessage for MotorDaCtxComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorDaCtxId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MotorDaCtxId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MotorDaCtxId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterAiComponentPb {
    pub WeaponId: i32,
    pub HatredGroupId: i64,
    pub AiTeamInitId: i32,
    pub CombatMessageId: i64,
    pub BasicPerceptionIds: Vec<i32>,
    pub HatredId: i64,
}

impl PbMessage for MonsterAiComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.HatredGroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HatredGroupId);
        }
        if self.AiTeamInitId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AiTeamInitId);
        }
        if self.CombatMessageId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CombatMessageId);
        }
        for item in &self.BasicPerceptionIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.HatredId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HatredId);
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
            2 => self.HatredGroupId = r.read_varint_i64()?,
            3 => self.AiTeamInitId = r.read_varint_i32()?,
            4 => self.CombatMessageId = r.read_varint_i64()?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.BasicPerceptionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => self.HatredId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterWeaponComponentPb {
    pub WeaponId: i32,
}

impl PbMessage for MonsterWeaponComponentPb {
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
pub struct BatchBulletCastComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for BatchBulletCastComponentPb {
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
pub struct RangeComponentPb {
    pub InRangePlayers: Vec<i32>,
    pub InRangeEntities: Vec<i64>,
}

impl PbMessage for RangeComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InRangePlayers {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.InRangeEntities {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InRangePlayers, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.InRangeEntities,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientDataComponentPb {
    pub IsStaticInit: bool,
    pub OwnerId: i64,
    pub GroupId: i32,
}

impl PbMessage for ClientDataComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsStaticInit {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerId);
        }
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.IsStaticInit = r.read_bool()?,
            2 => self.OwnerId = r.read_varint_i64()?,
            3 => self.GroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttributeComponentPb {
    pub HardnessModeId: i32,
    pub RageModeId: i32,
    pub AttrData: Vec<AttrData>,
}

impl PbMessage for AttributeComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HardnessModeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HardnessModeId);
        }
        if self.RageModeId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RageModeId);
        }
        for item in &self.AttrData {
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
            2 => self.HardnessModeId = r.read_varint_i32()?,
            3 => self.RageModeId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AttrData.push(<AttrData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TagComponentPb {
    pub GameplayTags: Vec<GameplayTagData>,
    pub EntityCommonTags: Vec<i32>,
    pub InitGameplayTag: bool,
}

impl PbMessage for TagComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GameplayTags {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.EntityCommonTags {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InitGameplayTag {
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
                self.GameplayTags
                    .push(<GameplayTagData as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EntityCommonTags,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => self.InitGameplayTag = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TriggerComponentPb {
    pub TriggerCount: i32,
    pub ExitTriggerCount: i32,
    pub ConstateId: i64,
}

impl PbMessage for TriggerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TriggerCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TriggerCount);
        }
        if self.ExitTriggerCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExitTriggerCount);
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
            1 => self.TriggerCount = r.read_varint_i32()?,
            2 => self.ExitTriggerCount = r.read_varint_i32()?,
            3 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateTagComponentPb {
    pub StateTagId: i32,
}

impl PbMessage for StateTagComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StateTagId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.StateTagId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionSkillComponentPb {
    pub VisionSkillInfos: Vec<VisionSkillInformation>,
    pub PhantomSkillInfo: Option<VisionSkillInformation>,
}

impl PbMessage for VisionSkillComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.VisionSkillInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.PhantomSkillInfo {
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
                self.VisionSkillInfos
                    .push(<VisionSkillInformation as PbMessage>::decode(bytes)?);
            }
            2 => {
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
pub struct EntityVarComponentPb {
    pub Vars: BTreeMap<Vec<u8>, VarDefinePb>,
}

impl PbMessage for EntityVarComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Vars.is_empty() {
            crate::wire::write_map(out, 1, &self.Vars);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Vars)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoneVisibleData {
    pub BoneName: Vec<u8>,
    pub HideBone: bool,
}

impl PbMessage for BoneVisibleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BoneName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.BoneName);
        }
        if self.HideBone {
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
            1 => self.BoneName = r.read_len_delimited()?.to_vec(),
            2 => self.HideBone = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateComponentPb {
    pub AnimationStates: Vec<i32>,
    pub SpecialStates: Vec<i32>,
    pub BoneVisibleDatas: Vec<BoneVisibleData>,
    pub AnimationTags: Vec<i32>,
    pub ModelId: i32,
}

impl PbMessage for AnimationStateComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AnimationStates {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BoneVisibleDatas {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.AnimationTags {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AnimationStates,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BoneVisibleDatas
                    .push(<BoneVisibleData as PbMessage>::decode(bytes)?);
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AnimationTags, |sr| {
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
pub struct LogicStateComponentPb {
    pub PositionState: i32,
    pub MoveState: i32,
    pub DirectionState: i32,
    pub PositionSubState: i32,
}

impl PbMessage for LogicStateComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PositionState != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PositionState);
        }
        if self.MoveState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveState);
        }
        if self.DirectionState != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DirectionState);
        }
        if self.PositionSubState != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PositionSubState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PositionState = r.read_varint_i32()?,
            2 => self.MoveState = r.read_varint_i32()?,
            3 => self.DirectionState = r.read_varint_i32()?,
            4 => self.PositionSubState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiftComponentPb {
    pub Location: i32,
}

impl PbMessage for LiftComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Location != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Location);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Location = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackboardParamComponentPb {
    pub BlackboardParams: Vec<BlackboardParam>,
}

impl PbMessage for BlackboardParamComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BlackboardParams {
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
                self.BlackboardParams
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SysBuffComponentPb {
    pub SysBuffInfos: Vec<SysBuffInformation>,
}

impl PbMessage for SysBuffComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SysBuffInfos {
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
                self.SysBuffInfos
                    .push(<SysBuffInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SysBuffInformation {
    pub ServerId: i32,
    pub BuffId: i64,
    pub Level: i32,
    pub MessageId: i64,
    pub InstigatorId: i64,
    pub Duration: f32,
    pub StackCount: i32,
    pub ApplyType: Option<ApplyGEType>,
    pub IsIterable: bool,
}

impl PbMessage for SysBuffInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.MessageId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MessageId);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if self.Duration != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Duration).to_bits().to_le_bytes());
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => self.ServerId = r.read_varint_i32()?,
            2 => self.BuffId = r.read_varint_i64()?,
            3 => self.Level = r.read_varint_i32()?,
            4 => self.MessageId = r.read_varint_i64()?,
            5 => self.InstigatorId = r.read_varint_i64()?,
            6 => self.Duration = r.read_f32()?,
            7 => self.StackCount = r.read_varint_i32()?,
            8 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.IsIterable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightBuffComponentPb {
    pub FightBuffInfos: Vec<FightBuffInformation>,
    pub ListBuffEffectCd: Vec<BuffEffectCd>,
    pub ClientBornBuffIds: Vec<i64>,
    pub ClientBornMessageId: i64,
}

impl PbMessage for FightBuffComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FightBuffInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ListBuffEffectCd {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.ClientBornBuffIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.ClientBornMessageId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientBornMessageId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.FightBuffInfos
                    .push(<FightBuffInformation as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ListBuffEffectCd
                    .push(<BuffEffectCd as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ClientBornBuffIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            4 => self.ClientBornMessageId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NearbyTrackingComponentPb {
    pub IsEnable: bool,
}

impl PbMessage for NearbyTrackingComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnable {
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
            1 => self.IsEnable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropComponentPb {
    pub ItemId: i32,
    pub ShowPlanId: i32,
    pub ItemCount: i32,
    pub EntityConfigId: i32,
}

impl PbMessage for DropComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ShowPlanId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowPlanId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.ShowPlanId = r.read_varint_i32()?,
            3 => self.ItemCount = r.read_varint_i32()?,
            4 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterCaptureComponentPb {
    pub TemplateId: i32,
    pub EntityId: i32,
    pub MonsterId: i32,
}

impl PbMessage for MonsterCaptureComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TemplateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TemplateId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TemplateId = r.read_varint_i32()?,
            2 => self.EntityId = r.read_varint_i32()?,
            3 => self.MonsterId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BubbleInfo {
    pub ActionGuid: Vec<u8>,
    pub GameCtx: Option<GameCtxPb>,
}

impl PbMessage for BubbleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActionGuid.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActionGuid);
        }
        if let Some(v) = &self.GameCtx {
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
            1 => self.ActionGuid = r.read_len_delimited()?.to_vec(),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BubbleComponentPb {
    pub BubbleInfos: Vec<BubbleInfo>,
}

impl PbMessage for BubbleComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BubbleInfos {
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
                self.BubbleInfos
                    .push(<BubbleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleRecordComponentPb {
    pub IsAutoRole: bool,
    pub ConstateId: i64,
}

impl PbMessage for RoleRecordComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAutoRole {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => self.IsAutoRole = r.read_bool()?,
            2 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DynamicInteractInfo {
    pub OptionGuid: Vec<u8>,
    pub GameCtx: Option<GameCtxPb>,
    pub Text: Vec<u8>,
    pub DelayRemove: bool,
}

impl PbMessage for DynamicInteractInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OptionGuid);
        }
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if !self.Text.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Text);
        }
        if self.DelayRemove {
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
            1 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.Text = r.read_len_delimited()?.to_vec(),
            4 => self.DelayRemove = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InteractComponentPb {
    pub DynamicInteractInfos: Vec<DynamicInteractInfo>,
    pub RandomInteractIndex: Vec<i32>,
    pub Interacting: bool,
}

impl PbMessage for InteractComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DynamicInteractInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RandomInteractIndex {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Interacting {
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
                self.DynamicInteractInfos
                    .push(<DynamicInteractInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RandomInteractIndex,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => self.Interacting = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemComponentPb {
    pub PosSender: i32,
    pub BlackBoards: Vec<SceneItemBlackboardParam>,
}

impl PbMessage for SceneItemComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PosSender != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PosSender);
        }
        for item in &self.BlackBoards {
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
            1 => self.PosSender = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BlackBoards
                    .push(<SceneItemBlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledComponentPb {
    pub PlayerEntityId: i64,
    pub RelationId: i32,
    pub IsShow: bool,
    pub MatchIndex: i32,
    pub ConstateId: i64,
}

impl PbMessage for BeControlledComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlayerEntityId);
        }
        if self.RelationId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RelationId);
        }
        if self.IsShow {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MatchIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchIndex);
        }
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.PlayerEntityId = r.read_varint_i64()?,
            2 => self.RelationId = r.read_varint_i32()?,
            3 => self.IsShow = r.read_bool()?,
            4 => self.MatchIndex = r.read_varint_i32()?,
            5 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PullingFoundationComponentPb {
    pub RelationId: i32,
    pub MatchIndex: i32,
}

impl PbMessage for PullingFoundationComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RelationId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RelationId);
        }
        if self.MatchIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RelationId = r.read_varint_i32()?,
            2 => self.MatchIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DynAttachComponentPb {
    pub PbDynAttachEntityConfigId: i32,
    pub PbDynAttachEntityActorKey: Vec<u8>,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub PbDynAttachRefActorKey: Vec<u8>,
}

impl PbMessage for DynAttachComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PbDynAttachEntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PbDynAttachEntityConfigId);
        }
        if !self.PbDynAttachEntityActorKey.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PbDynAttachEntityActorKey);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 4, v);
        }
        if !self.PbDynAttachRefActorKey.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PbDynAttachRefActorKey);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PbDynAttachEntityConfigId = r.read_varint_i32()?,
            2 => self.PbDynAttachEntityActorKey = r.read_len_delimited()?.to_vec(),
            3 => {
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
            5 => self.PbDynAttachRefActorKey = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConcomitantsComponentPb {
    pub VisionEntityId: Vec<i64>,
    pub CustomEntityIds: Vec<i64>,
    pub PhantomRoleId: i64,
    pub BossRushId: i64,
}

impl PbMessage for ConcomitantsComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.VisionEntityId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.CustomEntityIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.PhantomRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PhantomRoleId);
        }
        if self.BossRushId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BossRushId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.VisionEntityId, |sr| {
                    Ok(sr.read_varint_i64()?)
                })?
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CustomEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            3 => self.PhantomRoleId = r.read_varint_i64()?,
            4 => self.BossRushId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FollowEntityComponentPb {
    pub EntityId: i64,
}

impl PbMessage for FollowEntityComponentPb {
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
pub struct TimelineTrackControlDataPb {
    pub ControlPoint: i32,
}

impl PbMessage for TimelineTrackControlDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ControlPoint != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ControlPoint);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ControlPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
