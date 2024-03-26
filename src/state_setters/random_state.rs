use rand::prelude::*;
use std::f32::consts::PI;

use crate::math::rand_vec3;

use super::{state_setter::StateSetter, wrappers::state_wrapper::StateWrapper};

const X_MAX: f32 = 6726.;
const Y_MAX: f32 = 8902.;
const Z_MAX_BALL: f32 = 1850.;
const Z_MAX_CAR: f32 = 1900.;
const PITCH_MAX: f32 = PI / 2.;
const YAW_MAX: f32 = PI;
const ROLL_MAX: f32 = PI;

/// Random state setter that makes random position/velocity/rotation values for each car and for the ball (within reason, eg. below max speeds)
pub struct RandomState {
    ball_rand_speed: bool,
    cars_rand_speed: bool,
    cars_on_ground: bool,
    rng: SmallRng,
}

impl RandomState {
    pub fn new(ball_rand_speed: Option<bool>, cars_rand_speed: Option<bool>, cars_on_ground: Option<bool>, seed: Option<u64>) -> Self {
        let ball_rand_speed = ball_rand_speed.unwrap_or(false);
        let cars_rand_speed = cars_rand_speed.unwrap_or(false);
        let cars_on_ground = cars_on_ground.unwrap_or(false);
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000),
        };
        let rng = SmallRng::seed_from_u64(seed);

        RandomState {
            ball_rand_speed,
            cars_rand_speed,
            cars_on_ground,
            rng,
        }
    }

    // pub fn reset(&self, state_wrapper: StateWrapper) {
    //     self._reset_ball_random(state_wrapper, self.ball_rand_speed);
    //     self._reset_cars_random(state_wrapper, self.cars_on_ground, self.cars_rand_speed);
    // }

    fn _reset_ball_random(&mut self, state_wrapper: &mut StateWrapper, random_speed: bool) {
        // let mut rng  = rand::thread_rng();
        state_wrapper.ball.set_pos(
            Some(self.rng.gen::<f32>() * X_MAX - X_MAX / 2.),
            Some(self.rng.gen::<f32>() * Y_MAX - Y_MAX / 2.),
            Some(self.rng.gen::<f32>() * Z_MAX_BALL + 100.),
        );
        // Z lower bound check
        if state_wrapper.ball.position.z < 94. {
            state_wrapper.ball.position.z = 94.
        }
        if random_speed {
            let lin_vel = rand_vec3(3000., &mut self.rng);
            let ang_vel = rand_vec3(6., &mut self.rng);
            state_wrapper.ball.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
            state_wrapper.ball.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
        } else {
            state_wrapper.ball.set_lin_vel(Some(0.), Some(0.), Some(1.));
            state_wrapper.ball.set_ang_vel(Some(0.), Some(0.), Some(0.));
        }
    }

    fn _reset_cars_random(&mut self, state_wrapper: &mut StateWrapper, on_ground: bool, random_speed: bool) {
        // let mut rng  = rand::thread_rng();
        // let cars = &mut state_wrapper.cars;
        for car in &mut state_wrapper.cars {
            car.set_pos(
                Some(self.rng.gen::<f32>() * X_MAX - X_MAX / 2.),
                Some(self.rng.gen::<f32>() * Y_MAX - Y_MAX / 2.),
                Some(self.rng.gen::<f32>() * Z_MAX_CAR + 150.),
            );
            // Z lower bound check
            if car.position.z < 100. {
                car.position.z = 100.;
            }

            car.set_rot(
                Some(self.rng.gen::<f32>() * PITCH_MAX - PITCH_MAX / 2.),
                Some(self.rng.gen::<f32>() * YAW_MAX - YAW_MAX / 2.),
                Some(self.rng.gen::<f32>() * ROLL_MAX - ROLL_MAX / 2.),
            );

            car.boost = self.rng.gen::<f32>();

            if random_speed {
                let lin_vel = rand_vec3(2300., &mut self.rng);
                let ang_vel = rand_vec3(5.5, &mut self.rng);
                car.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
                car.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
            } else {
                car.set_lin_vel(Some(0.), Some(0.), Some(0.));
                car.set_ang_vel(Some(0.), Some(0.), Some(0.));
            }

            if on_ground || self.rng.gen::<f32>() < 0.5 {
                car.set_pos(None, None, Some(17.));
                car.set_lin_vel(None, None, Some(0.));
                car.set_rot(Some(0.), None, Some(0.));
                car.set_ang_vel(Some(0.), Some(0.), None);
            }
        }
    }
}

impl StateSetter for RandomState {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        self._reset_ball_random(state_wrapper, self.ball_rand_speed);
        self._reset_cars_random(state_wrapper, self.cars_on_ground, self.cars_rand_speed);
    }

    fn set_seed(&mut self, seed: u64) {
        self.rng = SmallRng::seed_from_u64(seed);
    }
}
