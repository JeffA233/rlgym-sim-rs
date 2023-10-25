// use crate::gamestates::game_state::GameState;

// use super::state_modifier::StateModifier;



// pub struct AlignCarsToBall {

// }

// impl Default for AlignCarsToBall {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl AlignCarsToBall {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// impl StateModifier for AlignCarsToBall {
//     fn modify_state(&mut self, state: &mut GameState) {
//         for player_data in state.players.iter_mut() {
//             let dist_diff = state.ball.position - player_data.car_data.position;
//             let mut angles = player_data.car_data.euler_angles;
//             angles.pitch = dist_diff.z.atan2(dist_diff.x);
//             angles.yaw = dist_diff.y.atan2(dist_diff.x);
//             angles.roll = 0f32.atan2(dist_diff.z);
//         }
//     }
// }