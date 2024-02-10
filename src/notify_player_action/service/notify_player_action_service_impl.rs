use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_card_unit::repository::game_card_unit_repository_impl::GameCardUnitRepositoryImpl;
use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::notify_player_action::repository::notify_player_action_repository_impl::NotifyPlayerActionRepositoryImpl;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::request::notify_to_opponent_you_deploy_unit_request::NotifyToOpponentYouDeployUnitRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_boost_card_request::NotifyToOpponentYouUseEnergyBoostCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_card_request::NotifyToOpponentYouUseEnergyCardRequest;
use crate::notify_player_action::service::response::notify_to_opponent_you_deploy_unit_response::NotifyToOpponentYouDeployUnitResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_boost_card_response::NotifyToOpponentYouUseEnergyBoostCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_card_response::NotifyToOpponentYouUseEnergyCardResponse;

pub struct NotifyPlayerActionServiceImpl {
    notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
}

impl NotifyPlayerActionServiceImpl {
    pub fn new(
        notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
    ) -> Self {

        NotifyPlayerActionServiceImpl {
            notify_player_action_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionServiceImpl::new(
                            NotifyPlayerActionRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionService for NotifyPlayerActionServiceImpl {
    async fn notify_to_opponent_what_you_do(&mut self, notify_to_opponent_what_you_do_request: NotifyToOpponentYouDeployUnitRequest) -> NotifyToOpponentYouDeployUnitResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_what_you_do()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_what_you_do(
            notify_to_opponent_what_you_do_request.get_opponent_unique_id(),
            notify_to_opponent_what_you_do_request.get_usage_hand_card_id()).await;

        NotifyToOpponentYouDeployUnitResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_energy_card(&mut self, notify_to_opponent_you_use_energy_card_request: NotifyToOpponentYouUseEnergyCardRequest) -> NotifyToOpponentYouUseEnergyCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_energy_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_you_use_energy_card(
            notify_to_opponent_you_use_energy_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_energy_card_request.get_unit_card_index(),
            notify_to_opponent_you_use_energy_card_request.get_usage_hand_card_id()).await;

        NotifyToOpponentYouUseEnergyCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_energy_boost(&mut self, notify_to_opponent_you_use_energy_boost_card_request: NotifyToOpponentYouUseEnergyBoostCardRequest) -> NotifyToOpponentYouUseEnergyBoostCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_energy_boost()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_you_use_energy_boost_card_response = notify_player_action_repository_guard.notify_to_opponent_you_use_energy_boost_card(
            notify_to_opponent_you_use_energy_boost_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_energy_boost_card_request.get_unit_card_index(),
            notify_to_opponent_you_use_energy_boost_card_request.get_usage_hand_card_id(),
            notify_to_opponent_you_use_energy_boost_card_request.get_boosting_energy_count(),
            notify_to_opponent_you_use_energy_boost_card_request.get_boosting_energy_card_id()).await;

        NotifyToOpponentYouUseEnergyBoostCardResponse::new(notify_to_opponent_you_use_energy_boost_card_response)
    }
}