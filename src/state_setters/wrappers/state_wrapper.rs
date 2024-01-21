use rocketsim_rs::sim::BoostPadState;

use crate::gamestates::game_state::GameState;

use super::{car_wrapper::CarWrapper, physics_wrapper::PhysicsWrapper};

// const BLUE_ID1: i32 = 1;
// const ORANGE_ID1: i32 = 5;

/// State wrapper that allows for easy modification of the state via itself
pub struct StateWrapper {
    pub ball: PhysicsWrapper,
    pub cars: Vec<CarWrapper>,
    pub pads: [BoostPadState; 34],
}

impl StateWrapper {
    pub fn new(blue_count: Option<usize>, orange_count: Option<usize>, game_state: Option<&GameState>) -> Self {
        let blue_count = blue_count.unwrap_or(0);
        let orange_count = orange_count.unwrap_or(0);
        match game_state {
            Some(game_state) => StateWrapper::_read_from_gamestate(game_state),
            None => {
                let mut cars = Vec::<CarWrapper>::new();
                let mut i = 0;
                for _ in 0..blue_count {
                    i += 1;
                    cars.push(CarWrapper::new(Some(0), Some(i), None))
                }
                for _ in 0..orange_count {
                    i += 1;
                    cars.push(CarWrapper::new(Some(1), Some(i), None))
                }
                
                StateWrapper {
                    ball: PhysicsWrapper::new(None),
                    cars,
                    pads: [BoostPadState { is_active: true,..Default::default() }; 34],
                }
            }
        }
    }

    fn _read_from_gamestate(game_state: &GameState) -> StateWrapper {
        let mut cars = Vec::<CarWrapper>::new();
        // let players = &mut game_state.players;
        for player in &game_state.players {
            cars.push(CarWrapper::new(None, None, Some(player)))
        }

        let mut pads = [BoostPadState::default(); 34];
        for (i, boost_pad) in game_state.boost_pads.iter().enumerate() {
            pads[i] = boost_pad.state;
        }
        
        StateWrapper {
            ball: PhysicsWrapper::new(Some(&game_state.ball)),
            cars,
            pads,
        }
    }

    // pub fn format_state(&self) -> Vec<f64> {
    //     let mut ball_vec = self.ball.encode();
    //     let mut full_vec = Vec::<f64>::new();
    //     full_vec.append(&mut ball_vec);
    //     for c in &self.cars {
    //         full_vec.append(&mut c.encode());
    //     }
    //     // let car_str = car_str_vec.join(" ");
    //     // format!("{ball_str} {car_str}")
    //     return full_vec
    // }
}
