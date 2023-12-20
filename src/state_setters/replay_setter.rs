use std::fs::File;
use memmap2::Mmap;
use ndarray::ArrayView2;
use ndarray::Axis;
use ndarray_npy::ViewNpyExt;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use crate::state_setters::state_setter::StateSetter;
use crate::state_setters::wrappers::state_wrapper::StateWrapper;

pub struct ReplaySetter<'a> {
    states: ArrayView2<'a, f32>,
    rng: SmallRng,
    random_boost: bool,
    random_pads: bool,
}

impl<'a> ReplaySetter<'a> {
    pub fn new(file_str: &str, random_boost: Option<bool>, random_pads: Option<bool>) -> Self {
        let rng = SmallRng::from_entropy();
        let file = File::open(file_str).expect("Make sure your file exists");
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        let mmap_leaked: &'static Mmap = Box::leak(Box::new(mmap));
        let states = ArrayView2::<f32>::view_npy(mmap_leaked)
            .expect("Data types of npy file must be f32 and save a numpy array and be 2d");
        let random_boost = random_boost.unwrap_or(false);
        let random_pads = random_pads.unwrap_or(false);
        Self { states, rng, random_boost, random_pads}
    }

    fn set_cars(&mut self, state_wrapper: &mut StateWrapper, state: Vec<f32>) {
        let data = &state[9..state_wrapper.cars.len() * 13 + 9];
        let mut i = 0;
        for car in state_wrapper.cars.iter_mut() {
            car.set_pos(Some(data[i]), Some(data[i + 1]), Some(data[i + 2]));
            car.set_rot(Some(data[i + 3]), Some(data[i + 4]), Some(data[i + 5]));
            car.set_lin_vel(Some(data[i + 6]), Some(data[i + 7]), Some(data[i + 8]));
            car.set_ang_vel(Some(data[i + 9]), Some(data[i + 10]), Some(data[i + 11]));
            if self.random_boost{
                car.boost = self.rng.gen_range(0.0..=1.);
            } 
            else{
                car.boost = data[i + 12];
            }
            i += 13;
        }
    }

    fn set_ball(state_wrapper: &mut StateWrapper, data: &[f32]) {
        state_wrapper.ball.set_pos(Some(data[0]), Some(data[1]), Some(data[2]));
        state_wrapper.ball.set_lin_vel(Some(data[3]), Some(data[4]), Some(data[5]));
        state_wrapper.ball.set_ang_vel(Some(data[6]), Some(data[7]), Some(data[8]));
    }

    fn set_pads(&mut self, state_wrapper: &mut StateWrapper){
        let big_pads = [3, 4, 15, 18, 29, 30];
        for (i, pad) in state_wrapper.pads.iter_mut().enumerate(){
            pad.is_active = self.rng.gen_bool(0.5);
            if !pad.is_active {
                if big_pads.contains(&i){
                    pad.cooldown = 10.;
                }
                else {
                    pad.cooldown = 4.;
                }
            }
        }
    }
}

impl<'a> StateSetter for ReplaySetter<'a> {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let index = self.rng.gen_range(0..self.states.dim().0);
        let binding = self.states.index_axis(Axis(0), index);
        let state = binding.as_slice().unwrap();
        Self::set_ball(state_wrapper, state);
        self.set_cars(state_wrapper, state.to_owned());
        if self.random_pads{
            self.set_pads(state_wrapper);
        }
    }
}


#[cfg(test)]
mod tests{
    use ndarray::Array2;
    use ndarray_npy::write_npy;
    use crate::sim_wrapper::wrapper::RocketsimWrapper;
    use super::*;
    
    #[test]
    fn replay_setter_load_threes(){
        rocketsim_rs::init(None);
        let gameconfig = crate::envs::game_match::GameConfig{team_size: 3, spawn_opponents: true,
             gravity: 1., boost_consumption: 1., tick_skip: 8};
        let mut sim = RocketsimWrapper::new(gameconfig);
        let (state, _) = sim.get_rlgym_gamestate(false);
        let array_to_write = make_test_array();
        let path = "./tests/test_files_npy/test3.npy";
        write_npy(path, &array_to_write).unwrap();
        let mut setter = ReplaySetter::new("./tests/test_files_npy/test3.npy", Some(false), Some(false));
        let mut wrapper = setter.build_wrapper(3, true, Some(&state));
        setter.reset(&mut wrapper);
        let (state, _) = sim.set_state(wrapper, false);
        assert_eq!(state.ball.position.x, 2.);
        assert_eq!(state.players[0].car_data.position.x, 3.);
        assert_eq!(state.players[5].boost_amount, 4.);
        assert!(state.boost_pads.iter().all(|x| *x != 0.));
    }

    #[test]
    fn replay_setter_load_ones(){
        rocketsim_rs::init(None);
        let gameconfig = crate::envs::game_match::GameConfig{team_size: 1, spawn_opponents: true,
            gravity: 1., boost_consumption: 1., tick_skip: 8};
        let mut sim = RocketsimWrapper::new(gameconfig);
        let (state, _) = sim.get_rlgym_gamestate(false);
        let pos_ball_0_x = 0;
        let pos_car_0_x = 9;
        let pos_car_1_boost = pos_car_0_x + (13 * 2) - 1;
        let mut array_to_write = Array2::<f32>::zeros((2, ((13 * 2) + 9)));
        for i in 0..2{
            array_to_write[[i,pos_ball_0_x]] = 2.;
            array_to_write[[i,pos_car_0_x]] = 3.;
            array_to_write[[i,pos_car_1_boost]] = 4.;
        }
        let path = "./tests/test_files_npy/test1.npy";
        write_npy(path, &array_to_write).unwrap();
        let mut setter = ReplaySetter::new(path, Some(false), Some(false));
        let mut wrapper = setter.build_wrapper(1, true, Some(&state));
        setter.reset(&mut wrapper);
        let (state, _) = sim.set_state(wrapper, false);
        assert_eq!(state.ball.position.x, 2.);
        assert_eq!(state.players[0].car_data.position.x, 3.);
        assert_eq!(state.players[1].boost_amount, 4.);
    }

    #[test]
    #[should_panic]
    fn replay_setter_panic_load(){
        // this panics because we write an f64 but it expects f32
        let array_to_write = Array2::<f64>::zeros((2, ((13 * 6) + 9)));
        let path = "./tests/test_files_npy/test_load.npy";
        write_npy(path, &array_to_write).unwrap();
        ReplaySetter::new(path, Some(false), Some(false));
    }

    #[test]
    fn replay_setter_random_boost(){
        rocketsim_rs::init(None);
        let gameconfig = crate::envs::game_match::GameConfig{team_size: 3, spawn_opponents: true,
             gravity: 1., boost_consumption: 1., tick_skip: 8};
        let mut sim = RocketsimWrapper::new(gameconfig);
        let (state, _) = sim.get_rlgym_gamestate(false);
        let array_to_write = make_test_array();
        let path = "./tests/test_files_npy/test_boost.npy";
        write_npy(path, &array_to_write).unwrap();
        let mut setter = ReplaySetter::new(path, Some(true), Some(false));
        let mut wrapper = setter.build_wrapper(3, true, Some(&state));
        setter.reset(&mut wrapper);
        let (state, _) = sim.set_state(wrapper, false);
        assert_ne!(state.players[5].boost_amount, 4.);
    }

    #[test]
    fn replay_setter_random_pads(){
        rocketsim_rs::init(None);
        let gameconfig = crate::envs::game_match::GameConfig{team_size: 3, spawn_opponents: true,
             gravity: 1., boost_consumption: 1., tick_skip: 8};
        let mut sim = RocketsimWrapper::new(gameconfig);
        let (state, _) = sim.get_rlgym_gamestate(false);
        let array_to_write = make_test_array();
        let path = "./tests/test_files_npy/test_pads.npy";
        write_npy(path, &array_to_write).unwrap();
        let mut setter = ReplaySetter::new(path, Some(false), Some(true));
        let mut wrapper = setter.build_wrapper(3, true, Some(&state));
        setter.reset(&mut wrapper);
        for pad in wrapper.pads.iter(){
            if !pad.is_active{
                assert!(pad.cooldown == 4. || pad.cooldown == 10.);
            }
        }
        let (state, _) = sim.set_state(wrapper, false);
        //it's technically possible for this to fail if all 34 pads roll true, but that seems unlikely, but just try it again
        assert!(!state.boost_pads.iter().all(|x| *x != 0.));  
    }

    fn make_test_array() -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> {
        let pos_ball_0_x = 0;
        let pos_car_0_x = 9;
        let pos_car_6_boost = pos_car_0_x + (13 * 6) - 1;
        let mut array_to_write = Array2::<f32>::zeros((2, ((13 * 6) + 9)));
        for i in 0..2{
            array_to_write[[i,pos_ball_0_x]] = 2.;
            array_to_write[[i,pos_car_0_x]] = 3.;
            array_to_write[[i,pos_car_6_boost]] = 4.;
        }
        array_to_write
    }
}