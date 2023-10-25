use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, state_setters::{random_state::RandomState, state_setter::StateSetter, default_state::DefaultState, wrappers::state_wrapper::StateWrapper}};

use super::state_modifier::StateModifier;

/// Used as a way to generate states for testing things like the observation builder, reward functions, etc.
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
        let mut wrapper = StateWrapper::new(Some(blue_count), Some(orange_count), None);
        self.state_setter.reset(&mut wrapper);
        
        let mut state_cars = Vec::<PlayerData>::new();
        for car_wrapper in wrapper.cars {
            let mut data = PlayerData::default();

            data.car_data.position = car_wrapper.position;
            data.car_data.linear_velocity = car_wrapper.linear_velocity;
            data.car_data.angular_velocity = car_wrapper.angular_velocity;
            data.car_data.euler_angles = car_wrapper.rotation;
            data.car_data.has_computed_euler_angles = true;
            // this is more of a formality just so that we have this data in case we want it
            data.car_data.rotation_mtx = data.car_data.euler_angles.euler_to_rotation();
            data.car_data.has_computed_rot_mtx = true;
            data.car_data.quaternion = data.car_data.rotation_mtx.rotation_to_quaternion();

            data.inverted_car_data.position = data.car_data.position.invert();
            data.inverted_car_data.linear_velocity = data.car_data.linear_velocity.invert();
            data.inverted_car_data.angular_velocity = data.car_data.angular_velocity.invert();
            data.inverted_car_data.rotation_mtx = data.car_data.rotation_mtx.invert();
            data.inverted_car_data.has_computed_rot_mtx = true;
            data.inverted_car_data.quaternion = data.car_data.quaternion.invert();
            data.inverted_car_data.euler_angles = data.inverted_car_data.quaternion.quat_to_euler();
            data.inverted_car_data.has_computed_euler_angles = true;

            state_cars.push(data);
        }
        
        let mut state = GameState { players: state_cars, ..Default::default()};
        for modifier in self.modifiers.iter_mut() {
            modifier.modify_state(&mut state);
        }

        // recalculate from the euler angles since that is what we expect to be updated when modifying the state
        for player_state in state.players.iter_mut() {
            player_state.car_data.rotation_mtx = player_state.car_data.euler_angles.euler_to_rotation();
            player_state.car_data.quaternion = player_state.car_data.rotation_mtx.rotation_to_quaternion();

            player_state.inverted_car_data.rotation_mtx = player_state.car_data.rotation_mtx.invert();
            player_state.inverted_car_data.quaternion = player_state.car_data.quaternion.invert();
            player_state.inverted_car_data.euler_angles = player_state.inverted_car_data.quaternion.quat_to_euler();
        }
        state
    }

    pub fn update_modifiers(&mut self, modifiers: Vec<Box<dyn StateModifier>>) {
        self.modifiers = modifiers;
    }
}