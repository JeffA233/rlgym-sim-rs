use crate::{
    common_values::{BLUE_TEAM, CAR_MAX_SPEED},
    gamestates::{game_state::GameState, player_data::PlayerData},
    math::{element_mult_vec, element_sub_vec},
    reward_functions::reward_fn::RewardFn,
};
use std::collections::HashMap;

pub struct EventReward {
    weights: Vec<f32>,
    last_registered_values: HashMap<i32, Vec<f32>>,
}

impl EventReward {
    pub fn new(
        goal: Option<f32>,
        team_goal: Option<f32>,
        concede: Option<f32>,
        touch: Option<f32>,
        shot: Option<f32>,
        save: Option<f32>,
        demo: Option<f32>,
        boost_pickup: Option<f32>,
    ) -> Self {
        let goal = goal.unwrap_or(0.);
        let team_goal = team_goal.unwrap_or(0.);
        let concede = concede.unwrap_or(0.);
        let touch = touch.unwrap_or(0.);
        let shot = shot.unwrap_or(0.);
        let save = save.unwrap_or(0.);
        let demo = demo.unwrap_or(0.);
        let boost_pickup = boost_pickup.unwrap_or(0.);

        EventReward {
            weights: vec![goal, team_goal, concede, touch, shot, save, demo, boost_pickup],
            last_registered_values: HashMap::new(),
        }
    }

    fn _extract_values(player: &PlayerData, state: &GameState) -> Vec<f32> {
        let team: i32;
        let opponent: i32;
        if player.team_num == BLUE_TEAM {
            team = state.blue_score;
            opponent = state.orange_score;
        } else {
            team = state.orange_score;
            opponent = state.blue_score;
        }

        vec![
            player.match_goals as f32,
            team as f32,
            opponent as f32,
            player.ball_touched as i64 as f32,
            player.match_shots as f32,
            player.match_saves as f32,
            player.match_demolishes as f32,
            player.boost_amount,
        ]
    }
}

impl RewardFn for EventReward {
    fn reset(&mut self, initial_state: &GameState) {
        self.last_registered_values.clear();
        for player in &initial_state.players {
            let id = player.car_id;
            self.last_registered_values.insert(id, EventReward::_extract_values(player, initial_state));
        }
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState) -> f32 {
        let id = player.car_id;
        let new_values = EventReward::_extract_values(player, state);
        let old_values = self.last_registered_values.insert(id, new_values.clone());
        let old_values = match old_values {
            Some(old_values) => old_values,
            None => new_values.clone(),
        };
        let diff_values = element_sub_vec(&new_values, &old_values);

        let is_value_positive: Vec<f32> = diff_values.iter().map(|x| if *x > 0. { *x } else { 0. }).collect();
        element_mult_vec(&is_value_positive, &self.weights).iter().sum()
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState) -> f32 {
        self.get_reward(player, state)
    }
}

pub struct VelocityReward {
    negative: bool,
}

impl VelocityReward {
    pub fn new(negative: Option<bool>) -> Self {
        let negative = negative.unwrap_or(false);
        VelocityReward { negative }
    }
}

impl RewardFn for VelocityReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState) -> f32 {
        // let norm = norm_func(&player.car_data.linear_velocity);
        let norm = player.car_data.linear_velocity.norm();
        norm / CAR_MAX_SPEED * (1 - 2 * self.negative as i32) as f32
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState) -> f32 {
        self.get_reward(player, state)
    }
}

pub struct SaveBoostReward {}

impl SaveBoostReward {
    pub fn new() -> Self {
        SaveBoostReward {}
    }
}

impl Default for SaveBoostReward {
    fn default() -> Self {
        Self::new()
    }
}

impl RewardFn for SaveBoostReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState) -> f32 {
        player.boost_amount.sqrt()
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState) -> f32 {
        self.get_reward(player, state)
    }
}
