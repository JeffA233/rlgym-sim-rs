// use ndarray::*;
// use std::collections::VecDeque;
use std::f32::consts::PI;

use crate::{common_values, IntoArray};
use crate::gamestates::game_state::GameState;
use crate::gamestates::physics_object::PhysicsObject;
use crate::gamestates::player_data::PlayerData;

use super::obs_builder::ObsBuilder;

/// Matrix's observation builder, holds a stack of previous ball positions and shows the stack in the observation
pub struct AdvancedObs {
    pos_std: f32,
    ang_std: f32,
}

impl Default for AdvancedObs {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedObs {
    // pub fn new(team_size: Option<usize>, expanding: Option<bool>, stack_size: Option<usize>) -> Self {
    pub fn new() -> Self {
        // let expanding = match expanding {
        //     Some(expanding) => expanding,
        //     None => false
        // };

        AdvancedObs { pos_std: 2300., ang_std: PI }
    }

    fn _add_player_to_obs(&self, obs: &mut Vec<f32>, car: &PlayerData, ball: &PhysicsObject, inverted: bool, player: Option<&PhysicsObject>) -> PhysicsObject {
        let player_car: PhysicsObject = if inverted {
            car.inverted_car_data
        } else {
            car.car_data
        };

        let mut rel_pos = ball.position - player_car.position;
        rel_pos = rel_pos.divide_by_var(self.pos_std);
        let mut rel_vel = ball.linear_velocity - player_car.linear_velocity;
        rel_vel = rel_vel.divide_by_var(self.pos_std);

        obs.extend(rel_pos);
        obs.extend(rel_vel);
        obs.extend(player_car.position.divide_by_var(self.pos_std));
        obs.extend(player_car.forward());
        obs.extend(player_car.up());
        obs.extend(player_car.linear_velocity.divide_by_var(self.pos_std));
        obs.extend(player_car.angular_velocity.divide_by_var(self.ang_std));
        obs.extend(vec![car.boost_amount, car.on_ground as i32 as f32, car.has_flip as i32 as f32, car.is_demoed as i32 as f32]);

        if let Some(player) = player {
            obs.extend((player_car.position - player.position).divide_by_var(self.pos_std));
            obs.extend((player_car.linear_velocity - player.linear_velocity).divide_by_var(self.pos_std));
        }

        player_car
    }
}

impl ObsBuilder for AdvancedObs {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_obs_space(&mut self) -> Vec<usize> {
        vec![276]
    }

    fn build_obs(&mut self, player: &PlayerData, state: &GameState, _config: &crate::envs::game_match::GameConfig) -> Vec<f32> {
        let inverted: bool;
        let ball: &PhysicsObject;
        let pads;
        if player.team_num == common_values::ORANGE_TEAM {
            inverted = true;
            ball = &state.inverted_ball;
            pads = state.inverted_boost_pads;
        } else {
            inverted = false;
            ball = &state.ball;
            pads = state.boost_pads;
        }

        let pos = &ball.position;
        let lin = &ball.linear_velocity;
        let ang = &ball.angular_velocity;

        // let pos_std = vec_div_variable(pos, &self.pos_std);
        // let lin_std = vec_div_variable(lin, &self.pos_std);
        // let ang_std = vec_div_variable(ang, &self.ang_std);
        let pos_std = pos.divide_by_var(self.pos_std);
        let lin_std = lin.divide_by_var(self.pos_std);
        let ang_std = ang.divide_by_var(self.ang_std);

        let mut obs = Vec::<f32>::with_capacity(276);

        obs.extend(pos_std);
        obs.extend(lin_std);
        obs.extend(ang_std);
        obs.extend(player.last_actions.into_array());
        obs.extend(pads.iter().map(|pad| pad.state.is_active as i32 as f32));

        // self.add_ball_to_stack(pos_std, lin_std, ang_std, player.car_id as usize);

        // let ball_stack = self.ball_stack[player.car_id as usize].make_contiguous().as_ref();
        // for ball_vec in self.ball_stack[player.car_id as usize].make_contiguous().as_ref() {

        let player_car = self._add_player_to_obs(&mut obs, player, ball, inverted, None);

        for other in &state.players {
            if other.car_id == player.car_id {
                continue;
            }

            self._add_player_to_obs(&mut obs, other, ball, inverted, Some(&player_car));
        }

        obs
    }
}
