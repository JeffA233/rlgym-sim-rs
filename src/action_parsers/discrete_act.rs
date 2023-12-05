use super::action_parser::ActionParser;

pub struct DiscreteAction {
    n_bins: usize,
}

impl DiscreteAction {
    pub fn new() -> Self {
        DiscreteAction { n_bins: 3 }
    }
}

impl Default for DiscreteAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionParser for DiscreteAction {
    fn get_action_space(&mut self) -> Vec<usize> {
        let mut act_space = vec![self.n_bins; 5];
        act_space.extend([2; 3]);
        act_space
    }

    fn parse_actions(&mut self, actions: Vec<Vec<f32>>, _state: &crate::gamestates::game_state::GameState) -> Vec<Vec<f32>> {
        let mut parsed_actions = Vec::<Vec<f32>>::new();
        // [[self.n_bins; 5], bool, bool, bool]
        for mut action_vec in actions {
            // let act = &mut action_vec[0];
            for act in &mut action_vec {
                *act = *act / (self.n_bins / 2) as f32 - 1.;
            }
            parsed_actions.push(action_vec);
        }

        parsed_actions
    }
}
