use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, state_setters::{random_state::RandomState, state_setter::StateSetter, default_state::DefaultState, wrappers::state_wrapper::StateWrapper}};

use super::state_modifier::StateModifier;

pub struct CombinedStateGenerator {
    modifiers: Vec<Box<dyn StateModifier>>,
    state_setter: Box<dyn StateSetter>,
}

impl CombinedStateGenerator {
    pub fn new(modifiers: Vec<Box<dyn StateModifier>>, random_state_init_op: Option<bool>) -> Self {
        let random_state_init = random_state_init_op.unwrap_or(false);
        let state_setter: Box<dyn StateSetter> = if random_state_init {
            Box::new(RandomState::new(Some(true), Some(true), Some(false), None))
        } else {
            Box::new(DefaultState::new(None))
        };
        CombinedStateGenerator {
            modifiers,
            state_setter,
        }
    }

    pub fn generate_state(&mut self, blue_count: usize, orange_count: usize) -> GameState {
        // let mut game_state = GameState::default();
        let wrapper = StateWrapper::new(Some(blue_count), Some(orange_count), None);
        self.state_setter.reset(&mut wrapper);
        
        let state_cars = Vec::<PlayerData>::new();
        for car_wrapper in wrapper.cars {

        }
        
        for modifier in self.modifiers.iter_mut() {
            modifier.modify_state(state)
        }
    }
}