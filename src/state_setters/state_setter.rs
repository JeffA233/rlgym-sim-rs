use crate::gamestates::game_state::GameState;

use super::wrappers::state_wrapper::StateWrapper;

pub trait StateSetter {
    fn build_wrapper(&mut self, max_team_size: usize, spawn_opponents: bool, game_state: Option<&GameState>) -> StateWrapper {
        StateWrapper::new(Some(max_team_size), if spawn_opponents { Some(max_team_size) } else { Some(0) }, game_state)
    }
    fn reset(&mut self, state_wrapper: &mut StateWrapper);
    fn set_seed(&mut self, _seed: u64) {}
}
