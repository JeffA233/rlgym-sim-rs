use crate::gamestates::game_state::GameState;

use super::action_parser::ActionParser;

/// Necto parser for Matrix
pub struct TestAction {}

impl Default for TestAction {
    fn default() -> Self {
        Self::new()
    }
}

impl TestAction {
    pub fn new() -> Self {
        TestAction {}
    }
}

impl ActionParser for TestAction {
    fn get_action_space(&mut self) -> Vec<f32> {
        vec![8.]
    }

    fn parse_actions(&mut self, actions: Vec<Vec<f32>>, _state: &GameState) -> Vec<Vec<f32>> {
        actions
    }
}
