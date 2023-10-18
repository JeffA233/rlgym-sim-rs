use crate::{
    common_values::CAR_MAX_SPEED,
    gamestates::{game_state::GameState, player_data::PlayerData},
    reward_functions::reward_fn::RewardFn,
};

pub struct VelocityPlayerToBallReward {
    use_scalar_projection: bool,
}

impl VelocityPlayerToBallReward {
    pub fn new(use_scalar_projection: Option<bool>) -> Self {
        let use_scalar_projection = use_scalar_projection.unwrap_or(false);
        VelocityPlayerToBallReward { use_scalar_projection }
    }
}

impl RewardFn for VelocityPlayerToBallReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, _previous_action: &[f32]) -> f32 {
        let vel = &player.car_data.linear_velocity;

        let pos_diff = state.ball.position - player.car_data.position;

        if self.use_scalar_projection {
            vel.scalar_projection(&pos_diff)
        } else {
            let partial = pos_diff.norm();
            let norm_pos_diff = pos_diff.divide_by_var(partial);
            let norm_vel = vel.divide_by_var(CAR_MAX_SPEED);
            norm_pos_diff.multiply_by_vel(&norm_vel).into_array().iter().sum()
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &[f32]) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}
