use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::notify_player_action_info::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::notify_player_action_info::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;
use crate::notify_player_action_info::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::notify_player_action_info::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::notify_player_action_info::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::notify_player_action_info::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::notify_player_action_info::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_field_unit_death_info::{PlayerFieldUnitDeathInfo};
use crate::notify_player_action_info::entity::player_hand_card_use_info::PlayerHandCardUseInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::{Opponent, You};
use crate::notify_player_action_info::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::notify_player_action_info::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::notify_player_action_info::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
use crate::notify_player_action_info::entity::player_search_card_list_info::PlayerSearchCardListInfo;
use crate::notify_player_action_info::entity::player_search_count_info::PlayerSearchCountInfo;
use crate::notify_player_action_info::entity::used_hand_card_info::UsedHandCardInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::response_generator::response_type::ResponseType::{NOTIFY_DECK_CARD_LOST_LIST, NOTIFY_DECK_CARD_USE_LIST, NOTIFY_DRAW_COUNT, NOTIFY_FIELD_ENERGY, NOTIFY_FIELD_UNIT_DAMAGE, NOTIFY_FIELD_UNIT_DEATH, NOTIFY_FIELD_UNIT_ENERGY, NOTIFY_FIELD_UNIT_HEALTH_POINT, NOTIFY_HAND_CARD_USE, NOTIFY_MAIN_CHARACTER_DAMAGE, NOTIFY_MAIN_CHARACTER_HEALTH_POINT, NOTIFY_MAIN_CHARACTER_SURVIVAL, NOTIFY_SEARCH_COUNT};

pub struct NotifyPlayerActionInfoRepositoryImpl;

impl NotifyPlayerActionInfoRepositoryImpl {
    pub fn new() -> Self {
        NotifyPlayerActionInfoRepositoryImpl { }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    fn get_player_hand_card_use_info(&self,
                                     notify_player_index: PlayerIndex,
                                     used_card_id: i32,
                                     used_card_type: KindsEnum
    ) -> PlayerHandCardUseInfo {

        if used_card_type == KindsEnum::Trap {
            let used_card_info = UsedHandCardInfo::new(-1, used_card_type as i32);
            let mut player_hand_use_map = HashMap::new();
            player_hand_use_map.insert(notify_player_index, used_card_info);

            return PlayerHandCardUseInfo::new(player_hand_use_map)
        }

        let used_card_info = UsedHandCardInfo::new(used_card_id, used_card_type as i32);
        let mut player_hand_use_map = HashMap::new();
        player_hand_use_map.insert(notify_player_index, used_card_info);

        PlayerHandCardUseInfo::new(player_hand_use_map)
    }

    fn get_player_deck_card_list_use_info(&self,
                                          notify_player_index: PlayerIndex,
                                          used_deck_card_list: Vec<i32>
    ) -> PlayerDeckCardUseListInfo {

        let mut player_deck_card_list_use_map = HashMap::new();
        player_deck_card_list_use_map.insert(notify_player_index, used_deck_card_list);

        PlayerDeckCardUseListInfo::new(player_deck_card_list_use_map)
    }

    fn get_player_field_unit_energy_info(&self,
                                         notify_player_index: PlayerIndex,
                                         field_unit_energy_info: FieldUnitEnergyInfo
    ) -> PlayerFieldUnitEnergyInfo {

        let mut player_field_unit_energy_map = HashMap::new();
        player_field_unit_energy_map.insert(notify_player_index, field_unit_energy_info);

        PlayerFieldUnitEnergyInfo::new(player_field_unit_energy_map)
    }

    fn get_player_drawn_card_list_info(&self,
                                       notify_player_index: PlayerIndex,
                                       drawn_card_list: Vec<i32>
    ) -> PlayerDrawnCardListInfo {

        let mut player_drawn_card_list_map = HashMap::new();
        player_drawn_card_list_map.insert(notify_player_index, drawn_card_list);

        PlayerDrawnCardListInfo::new(player_drawn_card_list_map)
    }

    fn get_player_draw_count_info(&self,
                                  notify_player_index: PlayerIndex,
                                  drawn_card_list: Vec<i32>
    ) -> PlayerDrawCountInfo {

        let mut player_draw_count_map = HashMap::new();
        player_draw_count_map.insert(notify_player_index, drawn_card_list.len() as i32);

        PlayerDrawCountInfo::new(player_draw_count_map)
    }

    fn get_player_search_card_list_info(&self,
                                        notify_player_index: PlayerIndex,
                                        found_card_list: Vec<i32>
    ) -> PlayerSearchCardListInfo {

        let mut player_search_card_list_map = HashMap::new();
        player_search_card_list_map.insert(notify_player_index, found_card_list);

        PlayerSearchCardListInfo::new(player_search_card_list_map)
    }

    fn get_player_search_count_info(&self,
                                    notify_player_index: PlayerIndex,
                                    found_card_list: Vec<i32>
    ) -> PlayerSearchCountInfo {

        let mut player_search_count_map = HashMap::new();
        player_search_count_map.insert(notify_player_index, found_card_list.len() as i32);

        PlayerSearchCountInfo::new(player_search_count_map)
    }

    fn get_player_field_energy_info(&self,
                                    notify_player_index: PlayerIndex,
                                    field_energy_count: i32
    ) -> PlayerFieldEnergyInfo {

        let mut player_field_energy_map = HashMap::new();
        player_field_energy_map.insert(notify_player_index, field_energy_count);

        PlayerFieldEnergyInfo::new(player_field_energy_map)
    }

    fn get_player_field_unit_damage_info(&self,
                                         notify_player_index: PlayerIndex,
                                         field_unit_damage_info: FieldUnitDamageInfo
    ) -> PlayerFieldUnitDamageInfo {

        let mut player_field_unit_damage_map = HashMap::new();
        player_field_unit_damage_map.insert(notify_player_index, field_unit_damage_info);

        PlayerFieldUnitDamageInfo::new(player_field_unit_damage_map)
    }

    fn get_player_field_unit_health_point_info(&self,
                                               notify_player_index: PlayerIndex,
                                               field_unit_health_point_info: FieldUnitHealthPointInfo
    ) -> PlayerFieldUnitHealthPointInfo {

        let mut player_field_unit_health_point_map = HashMap::new();
        player_field_unit_health_point_map.insert(notify_player_index, field_unit_health_point_info);

        PlayerFieldUnitHealthPointInfo::new(player_field_unit_health_point_map)
    }

    fn get_player_field_unit_death_info(&self,
                                        notify_player_index: PlayerIndex,
                                        field_unit_death_info: FieldUnitDeathInfo
    ) -> PlayerFieldUnitDeathInfo {

        let mut player_field_unit_death_map = HashMap::new();
        player_field_unit_death_map.insert(notify_player_index, field_unit_death_info);

        PlayerFieldUnitDeathInfo::new(player_field_unit_death_map)
    }

    fn get_player_deck_card_lost_list_info(&self,
                                           notify_player_index: PlayerIndex,
                                           lost_deck_card_list: Vec<i32>
    ) -> PlayerDeckCardLostListInfo {

        let mut player_deck_card_lost_list_map = HashMap::new();
        player_deck_card_lost_list_map.insert(notify_player_index, lost_deck_card_list);

        PlayerDeckCardLostListInfo::new(player_deck_card_lost_list_map)
    }

    fn get_player_main_character_damage_info(&self,
                                             notify_player_index: PlayerIndex,
                                             damage: i32
    ) -> PlayerMainCharacterDamageInfo {

        let mut player_main_character_damage_map = HashMap::new();
        player_main_character_damage_map.insert(notify_player_index, damage);

        PlayerMainCharacterDamageInfo::new(player_main_character_damage_map)
    }

    fn get_player_main_character_health_point_info(&self,
                                                   notify_player_index: PlayerIndex,
                                                   health_point: i32
    ) -> PlayerMainCharacterHealthPointInfo {

        let mut player_main_character_health_point_map = HashMap::new();
        player_main_character_health_point_map.insert(notify_player_index, health_point);

        PlayerMainCharacterHealthPointInfo::new(player_main_character_health_point_map)
    }

    fn get_player_main_character_survival_info(&self,
                                                   notify_player_index: PlayerIndex,
                                                   survival_status: StatusMainCharacterEnum
    ) -> PlayerMainCharacterSurvivalInfo {

        let mut player_main_character_survival_map = HashMap::new();
        player_main_character_survival_map.insert(notify_player_index, survival_status);

        PlayerMainCharacterSurvivalInfo::new(player_main_character_survival_map)
    }
}

#[async_trait]
impl NotifyPlayerActionInfoRepository for NotifyPlayerActionInfoRepositoryImpl {
    async fn notify_player_use_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_use_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        return true
    }

    async fn notify_player_use_deck_card_list(
        &mut self,
        opponent_unique_id: i32,
        found_card_id_list_form_deck: Vec<i32>) -> PlayerDeckCardUseListInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_use_deck_card_list()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_deck_card_use_list_info =
            self.get_player_deck_card_list_use_info(Opponent, found_card_id_list_form_deck.clone());

        // 상대에게 덱에서 추가적으로 사용한 카드 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DECK_CARD_USE_LIST(player_deck_card_use_list_info)))).await;

        return self.get_player_deck_card_list_use_info(You, found_card_id_list_form_deck.clone())
    }

    async fn notify_player_energy_of_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo) -> PlayerFieldUnitEnergyInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_energy_of_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_unit_energy_info =
            self.get_player_field_unit_energy_info(Opponent, field_unit_energy_info.clone());

        // 상대에게 내 필드 유닛의 에너지 정보 업데이트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info)))).await;

        return self.get_player_field_unit_energy_info(You, field_unit_energy_info.clone())
    }

    async fn notify_player_draw_card(
        &mut self,
        opponent_unique_id: i32,
        drawn_card_list: Vec<i32>) -> PlayerDrawnCardListInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_draw_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게는 내가 몇 장을 드로우 했는지 공지
        let player_draw_count_info =
            self.get_player_draw_count_info(Opponent, drawn_card_list.clone());

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DRAW_COUNT(player_draw_count_info)))).await;

        return self.get_player_drawn_card_list_info(You, drawn_card_list.clone());
    }

    async fn notify_player_search_card(
        &mut self,
        opponent_unique_id: i32,
        searched_card_list_from_deck: Vec<i32>) -> PlayerSearchCardListInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_search_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 몇 장을 검색하여 가져왔는지 공지
        let player_search_count_info =
            self.get_player_search_count_info(Opponent, searched_card_list_from_deck.clone());

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_SEARCH_COUNT(player_search_count_info)))).await;

        return self.get_player_search_card_list_info(You, searched_card_list_from_deck.clone())
    }

    async fn notify_player_opponent_field_energy(
        &mut self,
        opponent_unique_id: i32,
        remaining_field_energy_count: i32) -> PlayerFieldEnergyInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_opponent_field_energy()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_energy_info =
            self.get_player_field_energy_info(You, remaining_field_energy_count);

        // 상대에게 갱신된 필드 에너지 정보 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_ENERGY(player_field_energy_info)))).await;

        return self.get_player_field_energy_info(Opponent, remaining_field_energy_count)
    }

    async fn notify_player_remove_energy_of_specific_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo) -> PlayerFieldUnitEnergyInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_remove_energy_of_specific_opponent_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_unit_energy_info =
            self.get_player_field_unit_energy_info(You, field_unit_energy_info.clone());

        // 상대에게 내 필드 유닛의 에너지 정보 업데이트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info)))).await;

        return self.get_player_field_unit_energy_info(Opponent, field_unit_energy_info.clone())
    }

    async fn notify_player_apply_damage_to_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_damage_info: FieldUnitDamageInfo,
        field_unit_health_point_info: FieldUnitHealthPointInfo,
        field_unit_death_info: FieldUnitDeathInfo
    ) -> (PlayerFieldUnitDamageInfo, PlayerFieldUnitHealthPointInfo, PlayerFieldUnitDeathInfo) {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_apply_damage_to_specific_opponent_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_unit_damage_info =
            self.get_player_field_unit_damage_info(You, field_unit_damage_info.clone());

        // 상대에게 유닛이 입은 데미지 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_DAMAGE(player_field_unit_damage_info)))).await;

        let player_field_unit_health_point_info =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());

        // 상대에게 데미지 적용 후 남은 체력 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_HEALTH_POINT(player_field_unit_health_point_info)))).await;

        let player_field_unit_death_info =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());

        // 상대가 죽은 유닛들을 삭제할 수 있도록 생사 여부 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_DEATH(player_field_unit_death_info)))).await;

        return (self.get_player_field_unit_damage_info(Opponent, field_unit_damage_info.clone()),
                self.get_player_field_unit_health_point_info(Opponent, field_unit_health_point_info.clone()),
                self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone()))

    }

    async fn notify_player_apply_damage_to_opponent_main_character(
        &mut self,
        opponent_unique_id: i32,
        opponent_main_character_damage: i32,
        opponent_health_point: i32,
        opponent_survival: StatusMainCharacterEnum) -> (PlayerMainCharacterDamageInfo, PlayerMainCharacterHealthPointInfo, PlayerMainCharacterSurvivalInfo) {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_apply_damage_to_opponent_main_character()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_main_character_damage_info =
            self.get_player_main_character_damage_info(You, opponent_main_character_damage);

        // 상대 본체가 입은 데미지 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_MAIN_CHARACTER_DAMAGE(player_main_character_damage_info)))).await;

        let player_main_character_health_point_info =
            self.get_player_main_character_health_point_info(You, opponent_health_point);

        // 상대 본체에게 데미지 적용 후 남은 체력 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_MAIN_CHARACTER_HEALTH_POINT(player_main_character_health_point_info)))).await;

        let player_main_character_survival_info =
            self.get_player_main_character_survival_info(You, opponent_survival.clone());

        // 상대가 죽은 유닛들을 삭제할 수 있도록 생사 여부 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_MAIN_CHARACTER_SURVIVAL(player_main_character_survival_info)))).await;

        return (self.get_player_main_character_damage_info(You, opponent_main_character_damage),
                self.get_player_main_character_health_point_info(You, opponent_health_point),
                self.get_player_main_character_survival_info(You, opponent_survival.clone()))
    }

    async fn notify_player_attach_energy_to_specific_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo)-> PlayerFieldUnitEnergyInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_attach_energy_to_specific_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_unit_energy_info =
            self.get_player_field_unit_energy_info(Opponent, field_unit_energy_info.clone());

        // 상대에게 내 필드 유닛의 에너지 정보 업데이트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info)))).await;

        return self.get_player_field_unit_energy_info(You, field_unit_energy_info.clone())
    }

    async fn notify_player_death_of_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_death_info: FieldUnitDeathInfo) -> PlayerFieldUnitDeathInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_instant_death_of_specific_opponent_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_field_unit_death_info =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());

        // 상대에게 즉사 유닛의 생존 정보 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_DEATH(player_field_unit_death_info)))).await;

        return self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone())
    }

    async fn notify_player_lost_deck_card(
        &mut self,
        opponent_unique_id: i32,
        lost_deck_card_list: Vec<i32>) -> PlayerDeckCardLostListInfo {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_lost_deck_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let player_deck_card_lost_list_info =
            self.get_player_deck_card_lost_list_info(You, lost_deck_card_list.clone());

        // 상대에게 로스트 존으로 이동시킬 덱 카드 리스트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DECK_CARD_LOST_LIST(player_deck_card_lost_list_info)))).await;

        return self.get_player_deck_card_lost_list_info(Opponent, lost_deck_card_list.clone());
    }
}