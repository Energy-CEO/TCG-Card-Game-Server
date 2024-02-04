use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;

#[derive(Debug, Clone)]
pub struct GameFieldUnitCardList {
    game_field_unit_card_list: Vec<GameFieldUnitCard>,
}

impl GameFieldUnitCardList {
    pub fn new() -> GameFieldUnitCardList {
        GameFieldUnitCardList { game_field_unit_card_list: Vec::new() }
    }

    pub fn add_field_unit(&mut self, card: GameFieldUnitCard) {
        self.game_field_unit_card_list.push(card);
    }

    pub fn get_all_field_unit_list(&self) -> &Vec<GameFieldUnitCard> {
        &self.game_field_unit_card_list
    }

    pub fn add_energy_to_unit(&mut self, unit_id: i32) {
        if let Some(unit) = self.game_field_unit_card_list.iter_mut().find(|card| card.get_card() == unit_id) {
            unit.attach_energy();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_unit_list() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(3);
        let game_field_unit_card2 = GameFieldUnitCard::new(7);
        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let field_unit_list = game_field_unit_card_list.get_all_field_unit_list();
        assert_eq!(field_unit_list.len(), 2);
        assert_eq!(field_unit_list[0].get_card(), 3);
        assert_eq!(field_unit_list[1].get_card(), 7);

        println!("{:?}", field_unit_list);
    }

    #[test]
    fn test_attach_energy_to_unit() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(3);
        let game_field_unit_card2 = GameFieldUnitCard::new(7);
        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let field_unit_list = game_field_unit_card_list.get_all_field_unit_list();
        assert_eq!(field_unit_list.len(), 2);
        assert_eq!(field_unit_list[0].get_card(), 3);
        assert_eq!(field_unit_list[1].get_card(), 7);

        let mut cloned_list = game_field_unit_card_list.clone();
        cloned_list.add_energy_to_unit(3);

        println!("{:?}", cloned_list.get_all_field_unit_list());
    }
}
