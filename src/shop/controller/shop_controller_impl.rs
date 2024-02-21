use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::{Mutex as AsyncMutex, Mutex};

use crate::shop::controller::shop_controller::ShopController;
use crate::shop::controller::request_form::execute_shop_gacha_request_form::ExecuteShopGachaRequestForm;
use crate::shop::controller::request_form::execute_free_gacha_request_form::ExecuteFreeGachaRequestForm;
use crate::shop::controller::response_form::execute_shop_gacha_response_form::ExecuteShopGachaResponseForm;
use crate::shop::controller::response_form::execute_free_gacha_response_form::ExecuteFreeGachaResponseForm;


use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::account_point_service_impl::AccountPointServiceImpl;

use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::shop::controller::request_form::show_me_the_money_request::ShowMeTheMoneyRequest;
use crate::shop::controller::response_form::show_me_the_money_response::ShowMeTheMoneyResponse;
use crate::shop::service::request::data_to_display_in_shop_request::DataToDisplayInShopRequest;
use crate::shop::service::shop_service_impl::ShopServiceImpl;

use crate::shop_gacha::service::shop_gacha_service::ShopGachaService;
use crate::shop_gacha::service::shop_gacha_service_impl::ShopGachaServiceImpl;

pub struct ShopControllerImpl {
    account_point_service: Arc<AsyncMutex<AccountPointServiceImpl>>,
    shop_gacha_service: Arc<AsyncMutex<ShopGachaServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
}

impl ShopControllerImpl {
    pub fn new(account_point_service: Arc<AsyncMutex<AccountPointServiceImpl>>,
               shop_gacha_service: Arc<AsyncMutex<ShopGachaServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    ) -> Self {
        ShopControllerImpl {
            account_point_service,
            shop_gacha_service,
            redis_in_memory_service,
        }
    }
    pub fn get_instance() -> Arc<Mutex<ShopControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopControllerImpl::new(
                            AccountPointServiceImpl::get_instance(),
                            ShopGachaServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // async fn get_account_unique_id(&self, session_id: &str) -> i32 {
    //     let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
    //     let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
    //     let account_unique_id_string = account_unique_id_option_string.unwrap();
    //     let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
    //     account_unique_id
    // }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl ShopController for ShopControllerImpl {
    async fn execute_shop_gacha(&self, execute_shop_gacha_request_form: ExecuteShopGachaRequestForm) -> ExecuteShopGachaResponseForm {
        let account_unique_id = self.is_valid_session(execute_shop_gacha_request_form.to_session_validation_request()).await;
        //1. 재화 확인
        //2. 재화 사용
        let account_point_service_guard = self.account_point_service.lock().await;
        let check_pay_gold_response = account_point_service_guard.pay_gold(
            execute_shop_gacha_request_form.to_pay_gold_request(account_unique_id, 100)
        ).await;
        if !check_pay_gold_response.get_is_success() {
            return ExecuteShopGachaResponseForm::new(vec![0], false);
        }
        //3. 카드 뽑기
        let shop_gacha_service_guard = self.shop_gacha_service.lock().await;
        let get_specific_race_card_response = shop_gacha_service_guard.get_specific_race_card_default(
            execute_shop_gacha_request_form.to_get_specific_race_card_request(
                account_unique_id,
                execute_shop_gacha_request_form.get_race_enum(),
                execute_shop_gacha_request_form.is_confirmed_upper_legend())).await;

        ExecuteShopGachaResponseForm::new(get_specific_race_card_response.get_card_id_list(), true)
    }
    async fn execute_free_gacha(&self, execute_free_gacha_request_form: ExecuteFreeGachaRequestForm) -> ExecuteFreeGachaResponseForm {
        //1. 무료 뽑기 가능한지 확인
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(
            execute_free_gacha_request_form.to_session_validation_request()).await;
        let value_string = session_validation_response.get_value();
        let save_daily_token_redis_response = redis_in_memory_service_guard.save_daily_key_and_value(
            execute_free_gacha_request_form.to_save_daily_key_and_value_request(
                session_validation_response.get_value())).await;

        if !save_daily_token_redis_response.is_success() {
            return ExecuteFreeGachaResponseForm::new(vec![0], false);
        }

        //2. 카드 뽑기
        let shop_gacha_service_guard = self.shop_gacha_service.lock().await;
        let get_specific_race_card_response = shop_gacha_service_guard.get_specific_race_card_default(
            execute_free_gacha_request_form.to_get_specific_race_card_request(
                value_string.parse::<i32>().unwrap_or_else(|_| { -1 }),
                execute_free_gacha_request_form.get_race_enum(),
                execute_free_gacha_request_form.is_confirmed_upper_legend())).await;

        ExecuteFreeGachaResponseForm::new(get_specific_race_card_response.get_card_id_list(), true)
    }

    async fn show_me_the_money(&self, show_me_the_money_request: ShowMeTheMoneyRequest) -> ShowMeTheMoneyResponse {
        let account_unique_id = self.is_valid_session(show_me_the_money_request.to_session_validation_request()).await;

        let account_point_service = self.account_point_service.lock().await;
        let gain_account_point = account_point_service.gain_gold(
            show_me_the_money_request.to_gain_gold_request(account_unique_id, 100)).await;

        if !gain_account_point.get_is_success() {
            return ShowMeTheMoneyResponse::new(false);
        }
        ShowMeTheMoneyResponse::new(true)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_add_free_cards() {
        let shop_controller_impl_mutex = ShopControllerImpl::get_instance();
        let shop_controller_impl_mutex_guard = shop_controller_impl_mutex.lock().await;
        //
        // let request = GetCardDefaultRequest::new("qwer".to_string(), "Human".to_string(), true);
        //
        // let result = shop_service_impl_mutex_guard.get_specific_race_card_default(request).await;
        //
        // println!("result: {:?}", result);
        let request = ExecuteFreeGachaRequestForm::new("qwer".to_string(), "Human".to_string(), true);
        let result = shop_controller_impl_mutex_guard.execute_free_gacha(request).await;

        println!("{:?}", result);
    }
    #[test]
    async fn test_show_me_the_money() {
        let shop_controller_impl_mutex = ShopControllerImpl::get_instance();
        let shop_controller_impl_mutex_guard = shop_controller_impl_mutex.lock().await;
        let request = ShowMeTheMoneyRequest::new("qwer".to_string());
        let result = shop_controller_impl_mutex_guard.show_me_the_money(request).await;

        println!("{:?}", result);
    }
}