use crate::{
    common_values::{BALL_MAX_SPEED, BLUE_GOAL_BACK, BLUE_TEAM, ORANGE_GOAL_BACK, ORANGE_TEAM},
    gamestates::{game_state::GameState, physics_object::Position, player_data::PlayerData},
    reward_functions::default_reward::RewardFn,
};

pub struct VelocityBallToGoalReward {
    own_goal: bool,
    use_scalar_projection: bool,
}

impl VelocityBallToGoalReward {
    /// default: own_goal=false, use_scalar_projection=false
    pub fn new(own_goal: Option<bool>, use_scalar_projection: Option<bool>) -> Self {
        let own_goal = own_goal.unwrap_or(false);
        let use_scalar_projection = use_scalar_projection.unwrap_or(false);
        VelocityBallToGoalReward { own_goal, use_scalar_projection }
    }
}

impl RewardFn for VelocityBallToGoalReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, _previous_action: &[f32]) -> f32 {
        let objective: Position = if (player.team_num == BLUE_TEAM && !self.own_goal) || (player.team_num == ORANGE_TEAM && self.own_goal) {
            ORANGE_GOAL_BACK
        } else {
            BLUE_GOAL_BACK
        };

        // let pos_diff = element_sub_vec(&objective, &state.ball.position);
        let pos_diff = objective - state.ball.position;

        if self.use_scalar_projection {
            // return scalar_projection(&state.ball.linear_velocity, &pos_diff)
            state.ball.linear_velocity.scalar_projection(&pos_diff)
        } else {
            // let pos_diff_normed = norm_func(&pos_diff);
            let pos_diff_norm = pos_diff.norm();
            // let norm_pos_diff = vec_div_variable(&pos_diff, &pos_diff_normed);
            let norm_pos_diff = pos_diff.divide_by_var(pos_diff_norm);
            // let norm_vel = vec_div_variable(&state.ball.linear_velocity, &BALL_MAX_SPEED);
            let norm_vel = state.ball.linear_velocity.divide_by_var(BALL_MAX_SPEED);
            // return element_mult_vec(&norm_pos_diff, &norm_vel).iter().sum()
            norm_pos_diff.multiply_by_vel(&norm_vel).into_array().iter().sum()
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &[f32]) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}
