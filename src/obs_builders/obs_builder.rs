use crate::{
    envs::game_match::GameConfig,
    gamestates::{game_state::GameState, player_data::PlayerData},
};

pub trait ObsBuilder {
    fn reset(&mut self, initial_state: &GameState);
    fn get_obs_space(&mut self) -> Vec<usize>;
    fn pre_step(&mut self, _state: &GameState, _config: &GameConfig) {}
    fn build_obs(&mut self, player: &PlayerData, state: &GameState, config: &GameConfig, previous_action: &[f32]) -> Vec<f32>;
}
