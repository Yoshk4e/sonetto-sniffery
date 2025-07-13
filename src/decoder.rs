use crate::error::{AppError, PacketError};
use byteorder::{BE, ByteOrder};
use crate::sonettoproto;
use crate::sonettoproto::Message;
use crate::sonettoproto::*;

#[derive(Debug)]
pub struct ServerPacket {
    pub cmd_id: i16,
    pub result_code: i16,
    pub up_tag: u8,
    pub down_tag: u8,
    pub data: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ClientPacket {
    pub sequence: i32,
    pub cmd_id: i16,
    pub up_tag: u8,
    pub data: Vec<u8>,
}

pub fn decode_cs_packet(cmd_name: &str, data: &[u8]) -> Option<Box<dyn std::fmt::Debug>> {
    match cmd_name {
        "GetAchievementInfoRequest" => Some(Box::new(sonettoproto::GetAchievementInfoRequest::decode(data).ok()?)),
        "ShowAchievementRequest" => Some(Box::new(sonettoproto::ShowAchievementRequest::decode(data).ok()?)),
        "ReadNewAchievementRequest" => Some(Box::new(sonettoproto::ReadNewAchievementRequest::decode(data).ok()?)),
        "Get101InfosRequest" => Some(Box::new(sonettoproto::Get101InfosRequest::decode(data).ok()?)),
        "Get101BonusRequest" => Some(Box::new(sonettoproto::Get101BonusRequest::decode(data).ok()?)),
        "Get101SpBonusRequest" => Some(Box::new(sonettoproto::Get101SpBonusRequest::decode(data).ok()?)),
        "Get101OnceBonusRequest" => Some(Box::new(sonettoproto::Get101OnceBonusRequest::decode(data).ok()?)),
        "GetAct186SpBonusInfoRequest" => Some(Box::new(sonettoproto::GetAct186SpBonusInfoRequest::decode(data).ok()?)),
        "AcceptAct186SpBonusRequest" => Some(Box::new(sonettoproto::AcceptAct186SpBonusRequest::decode(data).ok()?)),
        "Act160GetInfoRequest" => Some(Box::new(sonettoproto::Act160GetInfoRequest::decode(data).ok()?)),
        "Act160FinishMissionRequest" => Some(Box::new(sonettoproto::Act160FinishMissionRequest::decode(data).ok()?)),
        "GetAct172InfoRequest" => Some(Box::new(sonettoproto::GetAct172InfoRequest::decode(data).ok()?)),
        "GetAct189InfoRequest" => Some(Box::new(sonettoproto::GetAct189InfoRequest::decode(data).ok()?)),
        "GetAct189OnceBonusRequest" => Some(Box::new(sonettoproto::GetAct189OnceBonusRequest::decode(data).ok()?)),
        "GetActivityInfosRequest" => Some(Box::new(sonettoproto::GetActivityInfosRequest::decode(data).ok()?)),
        "GetActivityInfosWithParamRequest" => Some(Box::new(sonettoproto::GetActivityInfosWithParamRequest::decode(data).ok()?)),
        "ActivityNewStageReadRequest" => Some(Box::new(sonettoproto::ActivityNewStageReadRequest::decode(data).ok()?)),
        "UnlockPermanentRequest" => Some(Box::new(sonettoproto::UnlockPermanentRequest::decode(data).ok()?)),
        "GetAntiqueInfoRequest" => Some(Box::new(sonettoproto::GetAntiqueInfoRequest::decode(data).ok()?)),
        "GetBpInfoRequest" => Some(Box::new(sonettoproto::GetBpInfoRequest::decode(data).ok()?)),
        "GetBpBonusRequest" => Some(Box::new(sonettoproto::GetBpBonusRequest::decode(data).ok()?)),
        "BpMarkFirstShowRequest" => Some(Box::new(sonettoproto::BpMarkFirstShowRequest::decode(data).ok()?)),
        "GetSelfSelectBonusRequest" => Some(Box::new(sonettoproto::GetSelfSelectBonusRequest::decode(data).ok()?)),
        "GetChargeInfoRequest" => Some(Box::new(sonettoproto::GetChargeInfoRequest::decode(data).ok()?)),
        "NewOrderRequest" => Some(Box::new(sonettoproto::NewOrderRequest::decode(data).ok()?)),
        "GetMonthCardInfoRequest" => Some(Box::new(sonettoproto::GetMonthCardInfoRequest::decode(data).ok()?)),
        "GetMonthCardBonusRequest" => Some(Box::new(sonettoproto::GetMonthCardBonusRequest::decode(data).ok()?)),
        "ReadChargeNewRequest" => Some(Box::new(sonettoproto::ReadChargeNewRequest::decode(data).ok()?)),
        "GetServerTimeRequest" => Some(Box::new(sonettoproto::GetServerTimeRequest::decode(data).ok()?)),
        "GetCurrencyListRequest" => Some(Box::new(sonettoproto::GetCurrencyListRequest::decode(data).ok()?)),
        "GetBuyPowerInfoRequest" => Some(Box::new(sonettoproto::GetBuyPowerInfoRequest::decode(data).ok()?)),
        "BuyPowerRequest" => Some(Box::new(sonettoproto::BuyPowerRequest::decode(data).ok()?)),
        "ExchangeDiamondRequest" => Some(Box::new(sonettoproto::ExchangeDiamondRequest::decode(data).ok()?)),
        "GetDialogInfoRequest" => Some(Box::new(sonettoproto::GetDialogInfoRequest::decode(data).ok()?)),
        "RecordDialogInfoRequest" => Some(Box::new(sonettoproto::RecordDialogInfoRequest::decode(data).ok()?)),
        "GetDungeonRequest" => Some(Box::new(sonettoproto::GetDungeonRequest::decode(data).ok()?)),
        "StartDungeonRequest" => Some(Box::new(sonettoproto::StartDungeonRequest::decode(data).ok()?)),
        "EndDungeonRequest" => Some(Box::new(sonettoproto::EndDungeonRequest::decode(data).ok()?)),
        "MapElementRequest" => Some(Box::new(sonettoproto::MapElementRequest::decode(data).ok()?)),
        "GetPointRewardRequest" => Some(Box::new(sonettoproto::GetPointRewardRequest::decode(data).ok()?)),
        "GetEpisodeHeroRecommendRequest" => Some(Box::new(sonettoproto::GetEpisodeHeroRecommendRequest::decode(data).ok()?)),
        "InstructionDungeonRewardRequest" => Some(Box::new(sonettoproto::InstructionDungeonRewardRequest::decode(data).ok()?)),
        "InstructionDungeonFinalRewardRequest" => Some(Box::new(sonettoproto::InstructionDungeonFinalRewardRequest::decode(data).ok()?)),
        "InstructionDungeonInfoRequest" => Some(Box::new(sonettoproto::InstructionDungeonInfoRequest::decode(data).ok()?)),
        "InstructionDungeonOpenRequest" => Some(Box::new(sonettoproto::InstructionDungeonOpenRequest::decode(data).ok()?)),
        "CoverDungeonRecordRequest" => Some(Box::new(sonettoproto::CoverDungeonRecordRequest::decode(data).ok()?)),
        "PuzzleFinishRequest" => Some(Box::new(sonettoproto::PuzzleFinishRequest::decode(data).ok()?)),
        "SavePuzzleProgressRequest" => Some(Box::new(sonettoproto::SavePuzzleProgressRequest::decode(data).ok()?)),
        "GetPuzzleProgressRequest" => Some(Box::new(sonettoproto::GetPuzzleProgressRequest::decode(data).ok()?)),
        "RefreshAssistRequest" => Some(Box::new(sonettoproto::RefreshAssistRequest::decode(data).ok()?)),
        "GetMainDramaRewardRequest" => Some(Box::new(sonettoproto::GetMainDramaRewardRequest::decode(data).ok()?)),
        "GetEquipInfoRequest" => Some(Box::new(sonettoproto::GetEquipInfoRequest::decode(data).ok()?)),
        "EquipStrengthenRequest" => Some(Box::new(sonettoproto::EquipStrengthenRequest::decode(data).ok()?)),
        "EquipBreakRequest" => Some(Box::new(sonettoproto::EquipBreakRequest::decode(data).ok()?)),
        "EquipLockRequest" => Some(Box::new(sonettoproto::EquipLockRequest::decode(data).ok()?)),
        "EquipComposeRequest" => Some(Box::new(sonettoproto::EquipComposeRequest::decode(data).ok()?)),
        "EquipDecomposeRequest" => Some(Box::new(sonettoproto::EquipDecomposeRequest::decode(data).ok()?)),
        "EquipRefineRequest" => Some(Box::new(sonettoproto::EquipRefineRequest::decode(data).ok()?)),
        "BeginFightRequest" => Some(Box::new(sonettoproto::BeginFightRequest::decode(data).ok()?)),
        "TestFightRequest" => Some(Box::new(sonettoproto::TestFightRequest::decode(data).ok()?)),
        "TestFightIdRequest" => Some(Box::new(sonettoproto::TestFightIdRequest::decode(data).ok()?)),
        "MoveCardRequest" => Some(Box::new(sonettoproto::MoveCardRequest::decode(data).ok()?)),
        "ResetRoundRequest" => Some(Box::new(sonettoproto::ResetRoundRequest::decode(data).ok()?)),
        "BeginRoundRequest" => Some(Box::new(sonettoproto::BeginRoundRequest::decode(data).ok()?)),
        "EndRoundRequest" => Some(Box::new(sonettoproto::EndRoundRequest::decode(data).ok()?)),
        "ChangeSubHeroRequest" => Some(Box::new(sonettoproto::ChangeSubHeroRequest::decode(data).ok()?)),
        "ChangeSubHeroExSkillRequest" => Some(Box::new(sonettoproto::ChangeSubHeroExSkillRequest::decode(data).ok()?)),
        "ReconnectFightRequest" => Some(Box::new(sonettoproto::ReconnectFightRequest::decode(data).ok()?)),
        "EndFightRequest" => Some(Box::new(sonettoproto::EndFightRequest::decode(data).ok()?)),
        "UseClothSkillRequest" => Some(Box::new(sonettoproto::UseClothSkillRequest::decode(data).ok()?)),
        "AutoRoundRequest" => Some(Box::new(sonettoproto::AutoRoundRequest::decode(data).ok()?)),
        "GetFightOperRequest" => Some(Box::new(sonettoproto::GetFightOperRequest::decode(data).ok()?)),
        "GetFightRecordGroupRequest" => Some(Box::new(sonettoproto::GetFightRecordGroupRequest::decode(data).ok()?)),
        "EntityInfoRequest" => Some(Box::new(sonettoproto::EntityInfoRequest::decode(data).ok()?)),
        "GetFightCardDeckInfoRequest" => Some(Box::new(sonettoproto::GetFightCardDeckInfoRequest::decode(data).ok()?)),
        "GetEntityDetailInfosRequest" => Some(Box::new(sonettoproto::GetEntityDetailInfosRequest::decode(data).ok()?)),
        "GetFightRecordAllRequest" => Some(Box::new(sonettoproto::GetFightRecordAllRequest::decode(data).ok()?)),
        "FightWithRecordAllRequest" => Some(Box::new(sonettoproto::FightWithRecordAllRequest::decode(data).ok()?)),
        "GetFightCardDeckDetailInfoRequest" => Some(Box::new(sonettoproto::GetFightCardDeckDetailInfoRequest::decode(data).ok()?)),
        "FinishGuideRequest" => Some(Box::new(sonettoproto::FinishGuideRequest::decode(data).ok()?)),
        "GetGuideInfoRequest" => Some(Box::new(sonettoproto::GetGuideInfoRequest::decode(data).ok()?)),
        "GetHandbookInfoRequest" => Some(Box::new(sonettoproto::GetHandbookInfoRequest::decode(data).ok()?)),
        "HandbookReadRequest" => Some(Box::new(sonettoproto::HandbookReadRequest::decode(data).ok()?)),
        "GetHeroGroupListRequest" => Some(Box::new(sonettoproto::GetHeroGroupListRequest::decode(data).ok()?)),
        "UpdateHeroGroupRequest" => Some(Box::new(sonettoproto::UpdateHeroGroupRequest::decode(data).ok()?)),
        "SetHeroGroupEquipRequest" => Some(Box::new(sonettoproto::SetHeroGroupEquipRequest::decode(data).ok()?)),
        "SetHeroGroupSnapshotRequest" => Some(Box::new(sonettoproto::SetHeroGroupSnapshotRequest::decode(data).ok()?)),
        "GetHeroGroupCommonListRequest" => Some(Box::new(sonettoproto::GetHeroGroupCommonListRequest::decode(data).ok()?)),
        "ChangeHeroGroupSelectRequest" => Some(Box::new(sonettoproto::ChangeHeroGroupSelectRequest::decode(data).ok()?)),
        "UpdateHeroGroupNameRequest" => Some(Box::new(sonettoproto::UpdateHeroGroupNameRequest::decode(data).ok()?)),
        "GetHeroGroupSnapshotListRequest" => Some(Box::new(sonettoproto::GetHeroGroupSnapshotListRequest::decode(data).ok()?)),
        "HeroInfoListRequest" => Some(Box::new(sonettoproto::HeroInfoListRequest::decode(data).ok()?)),
        "HeroUpgradeSkillRequest" => Some(Box::new(sonettoproto::HeroUpgradeSkillRequest::decode(data).ok()?)),
        "HeroLevelUpRequest" => Some(Box::new(sonettoproto::HeroLevelUpRequest::decode(data).ok()?)),
        "HeroRankUpRequest" => Some(Box::new(sonettoproto::HeroRankUpRequest::decode(data).ok()?)),
        "UseSkinRequest" => Some(Box::new(sonettoproto::UseSkinRequest::decode(data).ok()?)),
        "UnMarkIsNewRequest" => Some(Box::new(sonettoproto::UnMarkIsNewRequest::decode(data).ok()?)),
        "UnlockVoiceRequest" => Some(Box::new(sonettoproto::UnlockVoiceRequest::decode(data).ok()?)),
        "ItemUnlockRequest" => Some(Box::new(sonettoproto::ItemUnlockRequest::decode(data).ok()?)),
        "HeroTouchRequest" => Some(Box::new(sonettoproto::HeroTouchRequest::decode(data).ok()?)),
        "HeroTalentUpRequest" => Some(Box::new(sonettoproto::HeroTalentUpRequest::decode(data).ok()?)),
        "PutTalentCubeRequest" => Some(Box::new(sonettoproto::PutTalentCubeRequest::decode(data).ok()?)),
        "TakeoffAllTalentCubeRequest" => Some(Box::new(sonettoproto::TakeoffAllTalentCubeRequest::decode(data).ok()?)),
        "PutTalentSchemeRequest" => Some(Box::new(sonettoproto::PutTalentSchemeRequest::decode(data).ok()?)),
        "HeroDefaultEquipRequest" => Some(Box::new(sonettoproto::HeroDefaultEquipRequest::decode(data).ok()?)),
        "RenameTalentTemplateRequest" => Some(Box::new(sonettoproto::RenameTalentTemplateRequest::decode(data).ok()?)),
        "UseTalentTemplateRequest" => Some(Box::new(sonettoproto::UseTalentTemplateRequest::decode(data).ok()?)),
        "UnlockTalentStyleRequest" => Some(Box::new(sonettoproto::UnlockTalentStyleRequest::decode(data).ok()?)),
        "UseTalentStyleRequest" => Some(Box::new(sonettoproto::UseTalentStyleRequest::decode(data).ok()?)),
        "TalentStyleReadRequest" => Some(Box::new(sonettoproto::TalentStyleReadRequest::decode(data).ok()?)),
        "MarkHeroFavorRequest" => Some(Box::new(sonettoproto::MarkHeroFavorRequest::decode(data).ok()?)),
        "HeroTalentStyleStatRequest" => Some(Box::new(sonettoproto::HeroTalentStyleStatRequest::decode(data).ok()?)),
        "DestinyLevelUpRequest" => Some(Box::new(sonettoproto::DestinyLevelUpRequest::decode(data).ok()?)),
        "DestinyRankUpRequest" => Some(Box::new(sonettoproto::DestinyRankUpRequest::decode(data).ok()?)),
        "DestinyStoneUnlockRequest" => Some(Box::new(sonettoproto::DestinyStoneUnlockRequest::decode(data).ok()?)),
        "DestinyStoneUseRequest" => Some(Box::new(sonettoproto::DestinyStoneUseRequest::decode(data).ok()?)),
        "HeroRedDotReadRequest" => Some(Box::new(sonettoproto::HeroRedDotReadRequest::decode(data).ok()?)),
        "PutTalentCubeBatchRequest" => Some(Box::new(sonettoproto::PutTalentCubeBatchRequest::decode(data).ok()?)),
        "GetItemListRequest" => Some(Box::new(sonettoproto::GetItemListRequest::decode(data).ok()?)),
        "UseItemRequest" => Some(Box::new(sonettoproto::UseItemRequest::decode(data).ok()?)),
        "UsePowerItemRequest" => Some(Box::new(sonettoproto::UsePowerItemRequest::decode(data).ok()?)),
        "UsePowerItemListRequest" => Some(Box::new(sonettoproto::UsePowerItemListRequest::decode(data).ok()?)),
        "AutoUseExpirePowerItemRequest" => Some(Box::new(sonettoproto::AutoUseExpirePowerItemRequest::decode(data).ok()?)),
        "MarkReadSubType21Request" => Some(Box::new(sonettoproto::MarkReadSubType21Request::decode(data).ok()?)),
        "UseInsightItemRequest" => Some(Box::new(sonettoproto::UseInsightItemRequest::decode(data).ok()?)),
        "GetAllMailsRequest" => Some(Box::new(sonettoproto::GetAllMailsRequest::decode(data).ok()?)),
        "ReadMailRequest" => Some(Box::new(sonettoproto::ReadMailRequest::decode(data).ok()?)),
        "ReadMailBatchRequest" => Some(Box::new(sonettoproto::ReadMailBatchRequest::decode(data).ok()?)),
        "DeleteMailBatchRequest" => Some(Box::new(sonettoproto::DeleteMailBatchRequest::decode(data).ok()?)),
        "MarkMailJumpRequest" => Some(Box::new(sonettoproto::MarkMailJumpRequest::decode(data).ok()?)),
        "GetPlayerInfoRequest" => Some(Box::new(sonettoproto::GetPlayerInfoRequest::decode(data).ok()?)),
        "CreatePlayerRequest" => Some(Box::new(sonettoproto::CreatePlayerRequest::decode(data).ok()?)),
        "RenameRequest" => Some(Box::new(sonettoproto::RenameRequest::decode(data).ok()?)),
        "SetSignatureRequest" => Some(Box::new(sonettoproto::SetSignatureRequest::decode(data).ok()?)),
        "SetBirthdayRequest" => Some(Box::new(sonettoproto::SetBirthdayRequest::decode(data).ok()?)),
        "SetPortraitRequest" => Some(Box::new(sonettoproto::SetPortraitRequest::decode(data).ok()?)),
        "SetShowHeroUniqueIdsRequest" => Some(Box::new(sonettoproto::SetShowHeroUniqueIdsRequest::decode(data).ok()?)),
        "GetSimplePropertyRequest" => Some(Box::new(sonettoproto::GetSimplePropertyRequest::decode(data).ok()?)),
        "SetSimplePropertyRequest" => Some(Box::new(sonettoproto::SetSimplePropertyRequest::decode(data).ok()?)),
        "GetClothInfoRequest" => Some(Box::new(sonettoproto::GetClothInfoRequest::decode(data).ok()?)),
        "GetOtherPlayerInfoRequest" => Some(Box::new(sonettoproto::GetOtherPlayerInfoRequest::decode(data).ok()?)),
        "SetCharacterAgeRequest" => Some(Box::new(sonettoproto::SetCharacterAgeRequest::decode(data).ok()?)),
        "MarkMainThumbnailRequest" => Some(Box::new(sonettoproto::MarkMainThumbnailRequest::decode(data).ok()?)),
        "SetPlayerBgRequest" => Some(Box::new(sonettoproto::SetPlayerBgRequest::decode(data).ok()?)),
        "GetAssistBonusRequest" => Some(Box::new(sonettoproto::GetAssistBonusRequest::decode(data).ok()?)),
        "ReceiveAssistBonusRequest" => Some(Box::new(sonettoproto::ReceiveAssistBonusRequest::decode(data).ok()?)),
        "SetMainSceneSkinRequest" => Some(Box::new(sonettoproto::SetMainSceneSkinRequest::decode(data).ok()?)),
        "GetRedDotInfosRequest" => Some(Box::new(sonettoproto::GetRedDotInfosRequest::decode(data).ok()?)),
        "ShowRedDotRequest" => Some(Box::new(sonettoproto::ShowRedDotRequest::decode(data).ok()?)),
        "GetRoomInfoRequest" => Some(Box::new(sonettoproto::GetRoomInfoRequest::decode(data).ok()?)),
        "UseBlockRequest" => Some(Box::new(sonettoproto::UseBlockRequest::decode(data).ok()?)),
        "ResetRoomRequest" => Some(Box::new(sonettoproto::ResetRoomRequest::decode(data).ok()?)),
        "UseBuildingRequest" => Some(Box::new(sonettoproto::UseBuildingRequest::decode(data).ok()?)),
        "UnUseBuildingRequest" => Some(Box::new(sonettoproto::UnUseBuildingRequest::decode(data).ok()?)),
        "GetRoomObInfoRequest" => Some(Box::new(sonettoproto::GetRoomObInfoRequest::decode(data).ok()?)),
        "RoomConfirmRequest" => Some(Box::new(sonettoproto::RoomConfirmRequest::decode(data).ok()?)),
        "RoomRevertRequest" => Some(Box::new(sonettoproto::RoomRevertRequest::decode(data).ok()?)),
        "StartProductionLineRequest" => Some(Box::new(sonettoproto::StartProductionLineRequest::decode(data).ok()?)),
        "GainProductionLineRequest" => Some(Box::new(sonettoproto::GainProductionLineRequest::decode(data).ok()?)),
        "ProductionLineLvUpRequest" => Some(Box::new(sonettoproto::ProductionLineLvUpRequest::decode(data).ok()?)),
        "ProductionLineAccelerateRequest" => Some(Box::new(sonettoproto::ProductionLineAccelerateRequest::decode(data).ok()?)),
        "GetOtherRoomObInfoRequest" => Some(Box::new(sonettoproto::GetOtherRoomObInfoRequest::decode(data).ok()?)),
        "RoomLevelUpRequest" => Some(Box::new(sonettoproto::RoomLevelUpRequest::decode(data).ok()?)),
        "ProductionLineInfoRequest" => Some(Box::new(sonettoproto::ProductionLineInfoRequest::decode(data).ok()?)),
        "GetBuildingInfoRequest" => Some(Box::new(sonettoproto::GetBuildingInfoRequest::decode(data).ok()?)),
        "UnUseBlockRequest" => Some(Box::new(sonettoproto::UnUseBlockRequest::decode(data).ok()?)),
        "UpdateRoomHeroDataRequest" => Some(Box::new(sonettoproto::UpdateRoomHeroDataRequest::decode(data).ok()?)),
        "HideBlockPackageReddotRequest" => Some(Box::new(sonettoproto::HideBlockPackageReddotRequest::decode(data).ok()?)),
        "GainRoomHeroFaithRequest" => Some(Box::new(sonettoproto::GainRoomHeroFaithRequest::decode(data).ok()?)),
        "GetCharacterInteractionBonusRequest" => Some(Box::new(sonettoproto::GetCharacterInteractionBonusRequest::decode(data).ok()?)),
        "GetCharacterInteractionInfoRequest" => Some(Box::new(sonettoproto::GetCharacterInteractionInfoRequest::decode(data).ok()?)),
        "StartCharacterInteractionRequest" => Some(Box::new(sonettoproto::StartCharacterInteractionRequest::decode(data).ok()?)),
        "GetRoomThemeCollectionBonusRequest" => Some(Box::new(sonettoproto::GetRoomThemeCollectionBonusRequest::decode(data).ok()?)),
        "GetRoomPlanInfoRequest" => Some(Box::new(sonettoproto::GetRoomPlanInfoRequest::decode(data).ok()?)),
        "GetRoomPlanDetailsRequest" => Some(Box::new(sonettoproto::GetRoomPlanDetailsRequest::decode(data).ok()?)),
        "SetRoomPlanRequest" => Some(Box::new(sonettoproto::SetRoomPlanRequest::decode(data).ok()?)),
        "SetRoomPlanNameRequest" => Some(Box::new(sonettoproto::SetRoomPlanNameRequest::decode(data).ok()?)),
        "SetRoomPlanCoverRequest" => Some(Box::new(sonettoproto::SetRoomPlanCoverRequest::decode(data).ok()?)),
        "UseRoomPlanRequest" => Some(Box::new(sonettoproto::UseRoomPlanRequest::decode(data).ok()?)),
        "SwitchRoomPlanRequest" => Some(Box::new(sonettoproto::SwitchRoomPlanRequest::decode(data).ok()?)),
        "DeleteRoomPlanRequest" => Some(Box::new(sonettoproto::DeleteRoomPlanRequest::decode(data).ok()?)),
        "CopyOtherRoomPlanRequest" => Some(Box::new(sonettoproto::CopyOtherRoomPlanRequest::decode(data).ok()?)),
        "GetRoomShareRequest" => Some(Box::new(sonettoproto::GetRoomShareRequest::decode(data).ok()?)),
        "UseRoomShareRequest" => Some(Box::new(sonettoproto::UseRoomShareRequest::decode(data).ok()?)),
        "ShareRoomPlanRequest" => Some(Box::new(sonettoproto::ShareRoomPlanRequest::decode(data).ok()?)),
        "DeleteRoomShareRequest" => Some(Box::new(sonettoproto::DeleteRoomShareRequest::decode(data).ok()?)),
        "ReportRoomRequest" => Some(Box::new(sonettoproto::ReportRoomRequest::decode(data).ok()?)),
        "SetWaterTypeRequest" => Some(Box::new(sonettoproto::SetWaterTypeRequest::decode(data).ok()?)),
        "SetRoomSkinRequest" => Some(Box::new(sonettoproto::SetRoomSkinRequest::decode(data).ok()?)),
        "ReadRoomSkinRequest" => Some(Box::new(sonettoproto::ReadRoomSkinRequest::decode(data).ok()?)),
        "GenerateRoadRequest" => Some(Box::new(sonettoproto::GenerateRoadRequest::decode(data).ok()?)),
        "DeleteRoadRequest" => Some(Box::new(sonettoproto::DeleteRoadRequest::decode(data).ok()?)),
        "AllotCritterRequest" => Some(Box::new(sonettoproto::AllotCritterRequest::decode(data).ok()?)),
        "AllotVehicleRequest" => Some(Box::new(sonettoproto::AllotVehicleRequest::decode(data).ok()?)),
        "GetManufactureInfoRequest" => Some(Box::new(sonettoproto::GetManufactureInfoRequest::decode(data).ok()?)),
        "GetFrozenItemInfoRequest" => Some(Box::new(sonettoproto::GetFrozenItemInfoRequest::decode(data).ok()?)),
        "BuyManufactureBuildingRequest" => Some(Box::new(sonettoproto::BuyManufactureBuildingRequest::decode(data).ok()?)),
        "DispatchCritterRequest" => Some(Box::new(sonettoproto::DispatchCritterRequest::decode(data).ok()?)),
        "ManuBuildingUpgradeRequest" => Some(Box::new(sonettoproto::ManuBuildingUpgradeRequest::decode(data).ok()?)),
        "SelectSlotProductionPlanRequest" => Some(Box::new(sonettoproto::SelectSlotProductionPlanRequest::decode(data).ok()?)),
        "ManufactureAccelerateRequest" => Some(Box::new(sonettoproto::ManufactureAccelerateRequest::decode(data).ok()?)),
        "ReapFinishSlotRequest" => Some(Box::new(sonettoproto::ReapFinishSlotRequest::decode(data).ok()?)),
        "BatchDispatchCrittersRequest" => Some(Box::new(sonettoproto::BatchDispatchCrittersRequest::decode(data).ok()?)),
        "RouseCrittersRequest" => Some(Box::new(sonettoproto::RouseCrittersRequest::decode(data).ok()?)),
        "BatchAddProctionsRequest" => Some(Box::new(sonettoproto::BatchAddProctionsRequest::decode(data).ok()?)),
        "GainGuideBuildingRequest" => Some(Box::new(sonettoproto::GainGuideBuildingRequest::decode(data).ok()?)),
        "AccelerateGuidePlanRequest" => Some(Box::new(sonettoproto::AccelerateGuidePlanRequest::decode(data).ok()?)),
        "BuyRestSlotRequest" => Some(Box::new(sonettoproto::BuyRestSlotRequest::decode(data).ok()?)),
        "ChangeRestCritterRequest" => Some(Box::new(sonettoproto::ChangeRestCritterRequest::decode(data).ok()?)),
        "UnloadRestBuildingCrittersRequest" => Some(Box::new(sonettoproto::UnloadRestBuildingCrittersRequest::decode(data).ok()?)),
        "ReplaceRestBuildingCrittersRequest" => Some(Box::new(sonettoproto::ReplaceRestBuildingCrittersRequest::decode(data).ok()?)),
        "FeedCritterRequest" => Some(Box::new(sonettoproto::FeedCritterRequest::decode(data).ok()?)),
        "GetOrderInfoRequest" => Some(Box::new(sonettoproto::GetOrderInfoRequest::decode(data).ok()?)),
        "FinishOrderRequest" => Some(Box::new(sonettoproto::FinishOrderRequest::decode(data).ok()?)),
        "LockOrderRequest" => Some(Box::new(sonettoproto::LockOrderRequest::decode(data).ok()?)),
        "RefreshPurchaseOrderRequest" => Some(Box::new(sonettoproto::RefreshPurchaseOrderRequest::decode(data).ok()?)),
        "ChangePurchaseOrderTraceStateRequest" => Some(Box::new(sonettoproto::ChangePurchaseOrderTraceStateRequest::decode(data).ok()?)),
        "GetTradeTaskInfoRequest" => Some(Box::new(sonettoproto::GetTradeTaskInfoRequest::decode(data).ok()?)),
        "ReadNewTradeTaskRequest" => Some(Box::new(sonettoproto::ReadNewTradeTaskRequest::decode(data).ok()?)),
        "GetTradeSupportBonusRequest" => Some(Box::new(sonettoproto::GetTradeSupportBonusRequest::decode(data).ok()?)),
        "TradeLevelUpRequest" => Some(Box::new(sonettoproto::TradeLevelUpRequest::decode(data).ok()?)),
        "GetTradeTaskExtraBonusRequest" => Some(Box::new(sonettoproto::GetTradeTaskExtraBonusRequest::decode(data).ok()?)),
        "GetRoomLogRequest" => Some(Box::new(sonettoproto::GetRoomLogRequest::decode(data).ok()?)),
        "ReadRoomLogNewRequest" => Some(Box::new(sonettoproto::ReadRoomLogNewRequest::decode(data).ok()?)),
        "GetRougeOutsideInfoRequest" => Some(Box::new(sonettoproto::GetRougeOutsideInfoRequest::decode(data).ok()?)),
        "RougeActiveGeniusRequest" => Some(Box::new(sonettoproto::RougeActiveGeniusRequest::decode(data).ok()?)),
        "RougeReceivePointBonusRequest" => Some(Box::new(sonettoproto::RougeReceivePointBonusRequest::decode(data).ok()?)),
        "RougeMarkGeniusNewStageRequest" => Some(Box::new(sonettoproto::RougeMarkGeniusNewStageRequest::decode(data).ok()?)),
        "RougeMarkBonusNewStageRequest" => Some(Box::new(sonettoproto::RougeMarkBonusNewStageRequest::decode(data).ok()?)),
        "RougeGetUnlockCollectionsRequest" => Some(Box::new(sonettoproto::RougeGetUnlockCollectionsRequest::decode(data).ok()?)),
        "RougeGetNewReddotInfoRequest" => Some(Box::new(sonettoproto::RougeGetNewReddotInfoRequest::decode(data).ok()?)),
        "RougeMarkNewReddotRequest" => Some(Box::new(sonettoproto::RougeMarkNewReddotRequest::decode(data).ok()?)),
        "RougeUnlockStoryRequest" => Some(Box::new(sonettoproto::RougeUnlockStoryRequest::decode(data).ok()?)),
        "RougeLimiterSettingSaveRequest" => Some(Box::new(sonettoproto::RougeLimiterSettingSaveRequest::decode(data).ok()?)),
        "RougeLimiterUnlockBuffRequest" => Some(Box::new(sonettoproto::RougeLimiterUnlockBuffRequest::decode(data).ok()?)),
        "RougeLimiterSpeedUpBuffCdRequest" => Some(Box::new(sonettoproto::RougeLimiterSpeedUpBuffCdRequest::decode(data).ok()?)),
        "GetSignInInfoRequest" => Some(Box::new(sonettoproto::GetSignInInfoRequest::decode(data).ok()?)),
        "SignInRequest" => Some(Box::new(sonettoproto::SignInRequest::decode(data).ok()?)),
        "SignInAddupRequest" => Some(Box::new(sonettoproto::SignInAddupRequest::decode(data).ok()?)),
        "SignInHistoryRequest" => Some(Box::new(sonettoproto::SignInHistoryRequest::decode(data).ok()?)),
        "GetHeroBirthdayRequest" => Some(Box::new(sonettoproto::GetHeroBirthdayRequest::decode(data).ok()?)),
        "SignInTotalRewardRequest" => Some(Box::new(sonettoproto::SignInTotalRewardRequest::decode(data).ok()?)),
        "SignInTotalRewardAllRequest" => Some(Box::new(sonettoproto::SignInTotalRewardAllRequest::decode(data).ok()?)),
        "ClientStatBaseInfoRequest" => Some(Box::new(sonettoproto::ClientStatBaseInfoRequest::decode(data).ok()?)),
        "UpdateClientStatBaseInfoRequest" => Some(Box::new(sonettoproto::UpdateClientStatBaseInfoRequest::decode(data).ok()?)),
        "GetStoreInfosRequest" => Some(Box::new(sonettoproto::GetStoreInfosRequest::decode(data).ok()?)),
        "BuyGoodsRequest" => Some(Box::new(sonettoproto::BuyGoodsRequest::decode(data).ok()?)),
        "ReadStoreNewRequest" => Some(Box::new(sonettoproto::ReadStoreNewRequest::decode(data).ok()?)),
        "GetStoryRequest" => Some(Box::new(sonettoproto::GetStoryRequest::decode(data).ok()?)),
        "UpdateStoryRequest" => Some(Box::new(sonettoproto::UpdateStoryRequest::decode(data).ok()?)),
        "GetStoryFinishRequest" => Some(Box::new(sonettoproto::GetStoryFinishRequest::decode(data).ok()?)),
        "SummonRequest" => Some(Box::new(sonettoproto::SummonRequest::decode(data).ok()?)),
        "GetSummonInfoRequest" => Some(Box::new(sonettoproto::GetSummonInfoRequest::decode(data).ok()?)),
        "SummonQueryTokenRequest" => Some(Box::new(sonettoproto::SummonQueryTokenRequest::decode(data).ok()?)),
        "OpenLuckyBagRequest" => Some(Box::new(sonettoproto::OpenLuckyBagRequest::decode(data).ok()?)),
        "ChooseMultiUpHeroRequest" => Some(Box::new(sonettoproto::ChooseMultiUpHeroRequest::decode(data).ok()?)),
        "ChooseEnhancedPoolHeroRequest" => Some(Box::new(sonettoproto::ChooseEnhancedPoolHeroRequest::decode(data).ok()?)),
        "GetTaskInfoRequest" => Some(Box::new(sonettoproto::GetTaskInfoRequest::decode(data).ok()?)),
        "FinishTaskRequest" => Some(Box::new(sonettoproto::FinishTaskRequest::decode(data).ok()?)),
        "GetTaskActivityBonusRequest" => Some(Box::new(sonettoproto::GetTaskActivityBonusRequest::decode(data).ok()?)),
        "FinishAllTaskRequest" => Some(Box::new(sonettoproto::FinishAllTaskRequest::decode(data).ok()?)),
        "FinishReadTaskRequest" => Some(Box::new(sonettoproto::FinishReadTaskRequest::decode(data).ok()?)),
        "GetTurnbackInfoRequest" => Some(Box::new(sonettoproto::GetTurnbackInfoRequest::decode(data).ok()?)),
        "TurnbackSignInRequest" => Some(Box::new(sonettoproto::TurnbackSignInRequest::decode(data).ok()?)),
        "TurnbackOnceBonusRequest" => Some(Box::new(sonettoproto::TurnbackOnceBonusRequest::decode(data).ok()?)),
        "TurnbackFirstShowRequest" => Some(Box::new(sonettoproto::TurnbackFirstShowRequest::decode(data).ok()?)),
        "TurnbackBonusPointRequest" => Some(Box::new(sonettoproto::TurnbackBonusPointRequest::decode(data).ok()?)),
        "BuyDoubleBonusRequest" => Some(Box::new(sonettoproto::BuyDoubleBonusRequest::decode(data).ok()?)),
        "RefreshOnlineTaskRequest" => Some(Box::new(sonettoproto::RefreshOnlineTaskRequest::decode(data).ok()?)),
        "GetSettingInfosRequest" => Some(Box::new(sonettoproto::GetSettingInfosRequest::decode(data).ok()?)),
        "UpdateSettingInfoRequest" => Some(Box::new(sonettoproto::UpdateSettingInfoRequest::decode(data).ok()?)),
        _ => None,
    }
}

impl ServerPacket {
    const PACKET_HEADER: usize = 10;

    pub fn encode(&self) -> Vec<u8> {
        let total_len = Self::PACKET_HEADER + self.data.len();
        let mut buffer = vec![0u8; total_len];

        BE::write_u32(&mut buffer[0..4], (total_len - 4) as u32);
        BE::write_i16(&mut buffer[4..6], self.cmd_id);
        BE::write_i16(&mut buffer[6..8], self.result_code);
        buffer[8] = self.up_tag;
        buffer[9] = self.down_tag;
        (&mut buffer[Self::PACKET_HEADER..]).copy_from_slice(&self.data);

        buffer
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, AppError> {
        println!("[raw SC buffer] {:02X?}", &buffer[..buffer.len().min(32)]);

        if buffer.len() < Self::PACKET_HEADER {
            return Err(AppError::Packet(PacketError::LengthLessThanHeader(
                Self::PACKET_HEADER,
                buffer.len(),
            )));
        }

        let packet_size = BE::read_u32(&buffer[0..4]) as usize;
        if buffer.len() != packet_size + 4 {
            println!(
                "[!] packet_size mismatch: header says {}, actual = {}",
                packet_size + 4,
                buffer.len()
            );
            return Err(AppError::Packet(PacketError::LengthMismatch(
                packet_size + 4,
                buffer.len(),
            )));
        }

        let cmd_id = BE::read_i16(&buffer[4..6]);
        let result_code = BE::read_i16(&buffer[6..8]);
        let up_tag = buffer[8];
        let down_tag = buffer[9];
        let data = buffer[Self::PACKET_HEADER..].to_vec();

        println!(
            "[decode check] cmd_id={:#06X} result={} tags={}/{} data[0..8]={:02X?}",
            cmd_id, result_code, up_tag, down_tag, &data[..data.len().min(8)]
        );

        Ok(Self {
            cmd_id,
            result_code,
            up_tag,
            down_tag,
            data,
        })
    }

}

impl ClientPacket {
    const PACKET_HEADER: usize = 11;

    pub fn decode(buffer: &[u8]) -> Result<Self, AppError> {
        if buffer.len() < Self::PACKET_HEADER {
            return Err(AppError::Packet(PacketError::LengthLessThanHeader(
                Self::PACKET_HEADER,
                buffer.len(),
            )));
        }

        let packet_size = BE::read_i32(&buffer[0..4]) as usize;

        if buffer.len() != packet_size + 4 {
            return Err(AppError::Packet(PacketError::LengthMismatch(
                packet_size + 4,
                buffer.len(),
            )));
        }

        let sequence = BE::read_i32(&buffer[4..8]);
        let cmd_id = BE::read_i16(&buffer[8..10]);
        let up_tag = buffer[10];
        let data = buffer[Self::PACKET_HEADER..].to_vec();

        Ok(Self {
            sequence,
            cmd_id,
            up_tag,
            data,
        })
    }

    pub fn decode_message<T: Message + Default>(&self) -> Result<T, AppError> {
        let data = &*self.data;
        let decoded = T::decode(data)
            .map_err(|e| AppError::Packet(PacketError::ClientPacketDataDecodeFail(e)))?;
        Ok(decoded)
    }
}