//! Wuthering Waves 3.8.11 protobuf messages, enums, and message registry.

pub mod enums;
pub mod fail_response;
pub mod pb;
pub mod registry;
pub mod wire;

pub use enums::*;
pub use fail_response::FailResponse;
pub use pb::*;
pub use wire::{DecodeError, PbEnum, PbMessage, WireKind, WireType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_battlepass_and_map_messages() {
        let mut orig = BattlePassResponse {
            ErrorCode: Some(ErrorCode::Success),
            ..Default::default()
        };
        orig.BattlePass = Some(PbBattlePass {
            InTimeRange: true,
            Id: 2,
            Level: 70,
            PayStatus: Some(BattlePassPayStatus::Advanced),
            EndTime: 4070908800,
            HadEnter: true,
            TakenRewards: vec![PbBattlePassReward {
                Level: 1,
                Type: Some(BattlePassType::Free),
                ItemId: 3,
            }],
            ..Default::default()
        });
        let bytes = orig.encode_to_vec();
        let decoded = BattlePassResponse::decode(&bytes).expect("decode BattlePassResponse");
        assert_eq!(decoded, orig);

        let mut ach = AchievementReceiveResponse {
            ErrorCode: Some(ErrorCode::Success),
            ..Default::default()
        };
        ach.ItemMap.insert(3, 20);
        ach.ItemMap.insert(2, 5000);
        let ach_bytes = ach.encode_to_vec();
        let ach_dec = AchievementReceiveResponse::decode(&ach_bytes).expect("decode map");
        assert_eq!(ach_dec, ach);
    }

    #[test]
    fn decodes_packed_and_zigzag_fields() {
        // Test packed repeated i32 on BattlePassTaskTakeResponse.Ids (field 11? let's encode & decode roundtrip)
        let req = BattlePassTaskTakeRequest {
            Ids: vec![101, 102, 103],
        };
        let bytes = req.encode_to_vec();
        let dec = BattlePassTaskTakeRequest::decode(&bytes).expect("decode req");
        assert_eq!(dec.Ids, vec![101, 102, 103]);

        // Zigzag sint32 roundtrip via BattleResultRequest/similar or wire helpers
        let mut buf = Vec::new();
        wire::write_sint32(&mut buf, -42);
        let mut r = wire::Reader::new(&buf);
        assert_eq!(r.read_sint32().expect("sint32"), -42);
    }

    #[test]
    fn roundtrips_batch_1_to_3_and_negative_varints() {
        let act = ActionFinishRequest::decode(
            &ActionFinishRequest {
                PlayerId: 1001,
                IncId: 55,
                StartIndex: 2,
                ErrorMsg: b"ok".to_vec(),
            }
            .encode_to_vec(),
        )
        .expect("decode ActionFinishRequest");
        assert_eq!(act.PlayerId, 1001);
        assert_eq!(act.ErrorMsg, b"ok");

        let tp = AddTemporaryTeleportRequest::decode(
            &AddTemporaryTeleportRequest {
                Area: 12,
                Pos: Some(Vector {
                    X: 1.5,
                    Y: 2.5,
                    Z: 3.5,
                }),
                Rot: Some(Rotator {
                    Roll: 0.0,
                    Pitch: 0.0,
                    Yaw: 90.0,
                }),
            }
            .encode_to_vec(),
        )
        .expect("decode AddTemporaryTeleportRequest");
        assert_eq!(tp.Area, 12);
        assert_eq!(tp.Rot.unwrap().Yaw, 90.0);

        let forge = ForgeItemRequest::decode(
            &ForgeItemRequest {
                Id: 201,
                Count: 2,
                RoleId: 1104,
                InteractEntityId: 999,
            }
            .encode_to_vec(),
        )
        .expect("decode ForgeItemRequest");
        assert_eq!(forge.Id, 201);
        assert_eq!(forge.InteractEntityId, 999);

        let cook = CookFoodRequest::decode(
            &CookFoodRequest {
                Id: 10000,
                CookCount: 3,
                RoleId: 1104,
                InteractEntityId: 1234,
            }
            .encode_to_vec(),
        )
        .expect("decode CookFoodRequest");
        assert_eq!(cook.CookCount, 3);

        let ach = AchievementReceiveRequest::decode(
            &AchievementReceiveRequest {
                Id: 300101,
                IsGroupId: false,
            }
            .encode_to_vec(),
        )
        .expect("decode AchievementReceiveRequest");
        assert_eq!(ach.Id, 300101);

        let fish = FishingCabinPutRequest::decode(
            &FishingCabinPutRequest {
                CabinType: Some(CabinType::TempCabin),
                Id: 1,
                RemoveIncrId: 77,
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .expect("decode FishingCabinPutRequest");
        assert_eq!(fish.CabinType, Some(CabinType::TempCabin));
        assert_eq!(fish.RemoveIncrId, 77);

        let ch = ChannelChatRequest::decode(
            &ChannelChatRequest {
                SubChannelType: Some(SubChatChannelType::WorldTeam),
                ChannelType: Some(ChatChannelType::Team),
                ChatContentType: Some(ChatContentType::Text),
                Content: b"hello".to_vec(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .expect("decode ChannelChatRequest");
        assert_eq!(ch.Content, b"hello");

        let attr = GameplayAttributeData::decode(
            &GameplayAttributeData {
                BaseValue: 120,
                CurrentValue: 95,
                AttributeType: Some(EAttributeType::Life),
            }
            .encode_to_vec(),
        )
        .expect("decode GameplayAttributeData");
        assert_eq!(attr.BaseValue, 120);
        assert_eq!(attr.CurrentValue, 95);
        assert_eq!(attr.AttributeType, Some(EAttributeType::Life));

        let tc = TimeCheckNotify::decode(
            &TimeCheckNotify {
                ServerStopTime: 111,
                ServerTime: 222,
                ServerCombatTime: 333,
                ClientTime: 444,
                ServerFlowTimestamp: 555,
            }
            .encode_to_vec(),
        )
        .expect("decode TimeCheckNotify");
        assert_eq!(tc.ServerStopTime, 111);
        assert_eq!(tc.ServerCombatTime, 333);
        assert_eq!(tc.ClientTime, 444);

        let sub = SubmitNodeRequest::decode(
            &SubmitNodeRequest {
                TreeOwnerId: 139_000_025,
                TreeIncId: 139_000_025,
                NodeId: 19,
            }
            .encode_to_vec(),
        )
        .expect("decode SubmitNodeRequest");
        assert_eq!(sub.TreeOwnerId, 139_000_025);
        assert_eq!(sub.NodeId, 19);

        let rd = IllustratedRedDotResponse::decode(
            &IllustratedRedDotResponse {
                RedDotList: vec![IllustratedType::Monster, IllustratedType::Weapon],
            }
            .encode_to_vec(),
        )
        .expect("decode IllustratedRedDotResponse");
        assert_eq!(rd.RedDotList, vec![IllustratedType::Monster, IllustratedType::Weapon]);

        let dmg = DamageExecuteNotify::decode(
            &DamageExecuteNotify {
                DamageId: 1001,
                AttackerEntityId: 5,
                TargetEntityId: 7,
                Damage: 420,
                PartIndex: -1,
                ChangeLife: -420,
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .expect("decode DamageExecuteNotify");
        assert_eq!(dmg.PartIndex, -1);
        assert_eq!(dmg.ChangeLife, -420);

        let tag_out = TagComponentPb::decode(
            &TagComponentPb {
                EntityCommonTags: vec![-1_152_559_349, -3_775_711],
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .expect("decode TagComponentPb");
        assert_eq!(tag_out.EntityCommonTags, vec![-1_152_559_349, -3_775_711]);

        let state_out = StateTagComponentPb::decode(
            &StateTagComponentPb {
                StateTagId: -1_152_559_349,
            }
            .encode_to_vec(),
        )
        .expect("decode StateTagComponentPb");
        assert_eq!(state_out.StateTagId, -1_152_559_349);
    }
}
