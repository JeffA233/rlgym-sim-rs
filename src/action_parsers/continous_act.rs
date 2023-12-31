use super::action_parser::ActionParser;
use crate::gamestates::game_state::GameState;
use crate::math::clip;

#[derive(Clone, Copy, Default)]
pub struct ContinuousAction;

impl ContinuousAction {
    pub fn new() -> Self {
        ContinuousAction
    }
}

impl ActionParser for ContinuousAction {
    fn parse_actions(&mut self, actions: Vec<Vec<f32>>, _state: &GameState) -> Vec<Vec<f32>> {
        let mut parsed_actions = Vec::<Vec<f32>>::new();
        for mut action_vec in actions {
            clip(&mut action_vec, 1., -1.);
            parsed_actions.push(action_vec);
        }
        parsed_actions
    }

    fn get_action_space(&mut self) -> Vec<usize> {
        vec![]
    }
}
