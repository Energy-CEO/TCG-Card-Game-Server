use async_trait::async_trait;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_unit_by_using_hand_card_request::NoticeApplyDamageToEveryUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_request::{NoticeBoostEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_specific_unit_by_using_hand_card_request::NoticeApplyDamageToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_attach_energy_to_specific_unit_by_using_hand_card_request::NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_draw_card_request::{NoticeDrawCardRequest};
use crate::notify_player_action_info::service::request::notice_instant_death_of_specific_unit_by_using_hand_card_request::NoticeInstantDeathOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_unit_by_using_hand_card_request::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_field_energy_by_using_hand_card_request::NoticeRemoveFieldEnergyByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_search_card_by_using_hand_card_request::NoticeSearchCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_unit_by_using_hand_card_response::NoticeApplyDamageToEveryUnitByUsingHandCardResponse;

use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_response::{NoticeBoostEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_unit_by_using_hand_card_response::NoticeApplyDamageToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_attach_energy_to_specific_unit_by_using_hand_card_response::NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_draw_card_response::{NoticeDrawCardResponse};
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_unit_by_using_hand_card_response::NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_unit_by_using_hand_card_response::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_field_energy_by_using_hand_card_response::NoticeRemoveFieldEnergyByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_search_card_by_using_hand_card_response::NoticeSearchCardByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[async_trait]
pub trait NotifyPlayerActionInfoService {
    async fn notice_use_hand_card(
        &mut self, notice_use_hand_card_request: NoticeUseHandCardRequest)
        -> NoticeUseHandCardResponse;
    async fn notice_boost_energy_to_specific_unit(
        &mut self,
        notice_boost_energy_to_specific_unit_request: NoticeBoostEnergyToSpecificUnitRequest)
        -> NoticeBoostEnergyToSpecificUnitResponse;
    async fn notice_draw_card(
        &mut self,
        notice_draw_card_request: NoticeDrawCardRequest)
        -> NoticeDrawCardResponse;
    async fn notice_search_card_by_using_hand_card(
        &mut self,
        notice_search_card_by_using_hand_card_request: NoticeSearchCardByUsingHandCardRequest)
        -> NoticeSearchCardByUsingHandCardResponse;
    async fn notice_remove_field_energy_by_using_hand_card(
        &mut self,
        notice_remove_field_energy_by_using_hand_card_request: NoticeRemoveFieldEnergyByUsingHandCardRequest)
        -> NoticeRemoveFieldEnergyByUsingHandCardResponse;
    async fn notice_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self,
        notice_remove_energy_of_specific_unit_by_using_hand_card_request: NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest)
        -> NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
    async fn notice_apply_damage_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_apply_damage_to_specific_unit_by_using_hand_card_request: NoticeApplyDamageToSpecificUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToSpecificUnitByUsingHandCardResponse;
    async fn notice_apply_damage_to_every_unit_by_using_hand_card(
        &mut self,
        notice_apply_damage_to_every_unit_by_using_hand_card_request: NoticeApplyDamageToEveryUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToEveryUnitByUsingHandCardResponse;
    async fn notice_attach_energy_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_attach_energy_to_specific_unit_by_using_hand_card_request: NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse;
    async fn notice_instant_death_of_specific_unit_by_using_hand_card(
        &mut self,
        notice_instant_death_of_specific_unit_by_using_hand_card_request: NoticeInstantDeathOfSpecificUnitByUsingHandCardRequest)
        -> NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse;
}