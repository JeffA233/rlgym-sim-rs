use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
use std::f32::consts::PI;

use crate::gamestates::physics_object::{Position, Velocity, EulerAngle};

use super::{state_setter::StateSetter, wrappers::state_wrapper::StateWrapper};

/// State setter that creates a default Rocket League state
pub struct DefaultState {
    spawn_blue_pos: Vec<Vec<f32>>,
    spawn_blue_yaw: Vec<f32>,
    spawn_orange_pos: Vec<Vec<f32>>,
    spawn_orange_yaw: Vec<f32>,
    rng: SmallRng,
}

impl DefaultState {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000),
        };
        let rng = SmallRng::seed_from_u64(seed);
        DefaultState {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.],
            ],
            spawn_blue_yaw: vec![0.25 * PI, 0.75 * PI, 0.5 * PI, 0.5 * PI, 0.5 * PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.],
            ],
            spawn_orange_yaw: vec![-0.75 * PI, -0.25 * PI, -0.5 * PI, -0.5 * PI, -0.5 * PI],
            rng,
        }
    }
}

impl Default for DefaultState {
    fn default() -> Self {
        Self::new(None)
    }
}

impl StateSetter for DefaultState {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let mut spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();

        // this is to try to rearrange the order in a randomized way
        spawn_inds.sort_by_cached_key(|_| self.rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f32;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]];
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]];
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_lin_vel(Some(0.), Some(0.), Some(0.));
            car.set_ang_vel(Some(0.), Some(0.), Some(0.));
            car.set_rot(Some(0.), Some(yaw), Some(0.));
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: 0., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: 0., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }

    fn set_seed(&mut self, seed: u64) {
        self.rng = SmallRng::seed_from_u64(seed);
    }
}

// specific state tester for testing states from RLBot
pub struct ExactStateTester {}

impl ExactStateTester {
    pub fn new() -> Self {
        ExactStateTester {}
    }
}

impl Default for ExactStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for ExactStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        // let spawn_inds = vec![0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let car0 = &mut state_wrapper.cars[0];
        car0.position = Position { x: 1326.39, y: 243.70999, z: 177.16999};
        car0.angular_velocity = Velocity { x: -2.6771, y: 3.2167, z: 1.4274 };
        car0.linear_velocity = Velocity { x: 1231.8709, y: 383.341, z: -259.441 };
        car0.boost = 0.0;
        car0.rotation = EulerAngle { pitch: -0.1974, yaw: -0.5224, roll: 0.2607};

        let car1 = &mut state_wrapper.cars[1];
        car1.position = Position { x: -576.4099, y: -691.8399, z: 16.1299 };
        car1.angular_velocity = Velocity { x: -0.02661, y: -0.08521, z: -0.37851 };
        car1.linear_velocity = Velocity { x: -15.611, y: -466.8809, z: 6.6209 };
        car1.boost = 0.06;
        car1.rotation = EulerAngle { pitch: -0.0134, yaw: -1.5207, roll: 0.002 };

        state_wrapper.ball.position = Position { x: 3987.0397, y: -707.8599, z: 187.4899 };
        state_wrapper.ball.linear_velocity = Velocity { x: 791.3609, y: -270.9309, z: 1775.5809 };
        state_wrapper.ball.angular_velocity = Velocity { x: 0.6609, y: 5.9448, z: 0.4714 };
    }
}

// this has no randomization in the spawn indices for testing purposes
pub struct DefaultStateTester {
    spawn_blue_pos: Vec<Vec<f32>>,
    spawn_blue_yaw: Vec<f32>,
    spawn_orange_pos: Vec<Vec<f32>>,
    spawn_orange_yaw: Vec<f32>,
}

impl DefaultStateTester {
    pub fn new() -> Self {
        DefaultStateTester {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.],
            ],
            spawn_blue_yaw: vec![0.25 * PI, 0.75 * PI, 0.5 * PI, 0.5 * PI, 0.5 * PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.],
            ],
            spawn_orange_yaw: vec![-0.75 * PI, -0.25 * PI, -0.5 * PI, -0.5 * PI, -0.5 * PI],
        }
    }
}

impl Default for DefaultStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for DefaultStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f32;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]];
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]];
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(None, Some(yaw), None);
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: 0., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: 0., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

// this has no randomization in the spawn indices for testing purposes
pub struct DefaultStateTesterPitched {
    spawn_blue_pos: Vec<Vec<f32>>,
    spawn_blue_yaw: Vec<f32>,
    spawn_orange_pos: Vec<Vec<f32>>,
    spawn_orange_yaw: Vec<f32>,
}

impl DefaultStateTesterPitched {
    pub fn new() -> Self {
        Self {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 21.],
                vec![2048., -2560., 21.],
                vec![-256., -3840., 21.],
                vec![256., -3840., 21.],
                vec![0., -4608., 21.],
            ],
            spawn_blue_yaw: vec![0.25 * PI, 0.75 * PI, 0.5 * PI, 0.5 * PI, 0.5 * PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 21.],
                vec![-2048., 2560., 21.],
                vec![256., 3840., 21.],
                vec![-256., 3840., 21.],
                vec![0., 4608., 21.],
            ],
            spawn_orange_yaw: vec![-0.75 * PI, -0.25 * PI, -0.5 * PI, -0.5 * PI, -0.5 * PI],
        }
    }
}

impl Default for DefaultStateTesterPitched {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for DefaultStateTesterPitched {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f32;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]];
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]];
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(Some(0.25 * PI), Some(yaw), Some(0.15 * PI));
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: 0., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: 0., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

/// for testing blue goal rewards and such
pub struct BlueGoalStateTester {
    spawn_blue_pos: Vec<Vec<f32>>,
    spawn_blue_yaw: Vec<f32>,
    spawn_orange_pos: Vec<Vec<f32>>,
    spawn_orange_yaw: Vec<f32>,
}

impl BlueGoalStateTester {
    pub fn new() -> Self {
        BlueGoalStateTester {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.],
            ],
            spawn_blue_yaw: vec![0.25 * PI, 0.75 * PI, 0.5 * PI, 0.5 * PI, 0.5 * PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.],
            ],
            spawn_orange_yaw: vec![-0.75 * PI, -0.25 * PI, -0.5 * PI, -0.5 * PI, -0.5 * PI],
        }
    }
}

impl Default for BlueGoalStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for BlueGoalStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f32;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]];
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]];
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(None, Some(yaw), None);
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: 4500., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: 1000., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

/// for testing orange goal rewards and such
pub struct OrangeGoalStateTester {
    spawn_blue_pos: Vec<Vec<f32>>,
    spawn_blue_yaw: Vec<f32>,
    spawn_orange_pos: Vec<Vec<f32>>,
    spawn_orange_yaw: Vec<f32>,
}

impl OrangeGoalStateTester {
    pub fn new() -> Self {
        OrangeGoalStateTester {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.],
            ],
            spawn_blue_yaw: vec![0.25 * PI, 0.75 * PI, 0.5 * PI, 0.5 * PI, 0.5 * PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.],
            ],
            spawn_orange_yaw: vec![-0.75 * PI, -0.25 * PI, -0.5 * PI, -0.5 * PI, -0.5 * PI],
        }
    }
}

impl Default for OrangeGoalStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for OrangeGoalStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f32;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]];
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]];
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(None, Some(yaw), None);
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: -4900., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: -1000., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

/// for testing orange goal rewards and such
pub struct AgentBallHitStateTester {
    spawn_blue_pos: Vec<Vec<f32>>,
    // spawn_blue_yaw: Vec<f64>,
    spawn_orange_pos: Vec<Vec<f32>>,
    // spawn_orange_yaw: Vec<f64>
}

impl AgentBallHitStateTester {
    pub fn new() -> Self {
        AgentBallHitStateTester {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.],
            ],
            // spawn_blue_yaw: vec![0.25*PI, 0.75*PI, 0.5*PI, 0.5*PI, 0.5*PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.],
            ],
            // spawn_orange_yaw: vec![-0.75*PI, -0.25*PI, -0.5*PI, -0.5*PI, -0.5*PI]
        }
    }
}

impl Default for AgentBallHitStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for AgentBallHitStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = [0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            // let yaw: f64;

            if car.get_team_num() == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                // yaw = self.spawn_blue_yaw[spawn_inds[blue_count]].clone();
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                // yaw = self.spawn_orange_yaw[spawn_inds[orange_count]].clone();
                orange_count += 1;
            }

            car.set_pos(Some(0.), Some(-300.), Some(pos[2]));
            car.set_rot(None, Some(0.5 * PI), None);
            car.boost = 0.33;
        }

        state_wrapper.ball.position = Position { x: 0., y: 0., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: -1000., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

/// for testing orange goal rewards and such
pub struct DemoStateTester {
    // spawn_blue_pos: Vec<Vec<f32>>,
    // spawn_blue_yaw: Vec<f64>,
    // spawn_orange_pos: Vec<Vec<f32>>,
    // spawn_orange_yaw: Vec<f64>
}

impl DemoStateTester {
    pub fn new() -> Self {
        DemoStateTester {
            // spawn_blue_pos: vec![
            //     vec![-2048., -2560., 17.],
            //     vec![2048., -2560., 17.],
            //     vec![-256., -3840., 17.],
            //     vec![256., -3840., 17.],
            //     vec![0., -4608., 17.],
            // ],
            // spawn_blue_yaw: vec![0.25*PI, 0.75*PI, 0.5*PI, 0.5*PI, 0.5*PI],
            // spawn_orange_pos: vec![
            //     vec![2048., 2560., 17.],
            //     vec![-2048., 2560., 17.],
            //     vec![256., 3840., 17.],
            //     vec![-256., 3840., 17.],
            //     vec![0., 4608., 17.],
            // ],
            // spawn_orange_yaw: vec![-0.75*PI, -0.25*PI, -0.5*PI, -0.5*PI, -0.5*PI]
        }
    }
}

impl Default for DemoStateTester {
    fn default() -> Self {
        Self::new()
    }
}

impl StateSetter for DemoStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        // let spawn_inds = vec![0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        // let mut blue_count = 0;
        // let mut orange_count = 0;
        // for car in &mut state_wrapper.cars {
        let car1 = &mut state_wrapper.cars[0];
        // let pos;
        // let yaw: f64;

        // if car1.team_num == 0 {
        //     pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
        //     // yaw = self.spawn_blue_yaw[spawn_inds[blue_count]].clone();
        //     blue_count += 1;
        // } else {
        //     pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
        //     // yaw = self.spawn_orange_yaw[spawn_inds[orange_count]].clone();
        //     orange_count += 1;
        // }

        car1.set_pos(Some(0.), Some(-300.), Some(17.));
        car1.set_rot(None, Some(0.5 * PI), None);
        car1.set_lin_vel(Some(0.), Some(2300.), Some(0.));
        car1.boost = 1.0;

        let car2 = &mut state_wrapper.cars[1];
        // let pos;
        // let yaw: f64;

        // if car2.team_num == 0 {
        //     pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
        //     // yaw = self.spawn_blue_yaw[spawn_inds[blue_count]].clone();
        //     blue_count += 1;
        // } else {
        //     pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
        //     // yaw = self.spawn_orange_yaw[spawn_inds[orange_count]].clone();
        //     orange_count += 1;
        // }

        car2.set_pos(Some(0.), Some(300.), Some(17.));
        car2.set_rot(None, Some(-0.5 * PI), None);
        car2.set_lin_vel(Some(0.), Some(-2300.), Some(0.));
        car2.boost = 1.0;

        state_wrapper.ball.position = Position { x: 4000., y: 0., z: 91.25 };
        state_wrapper.ball.linear_velocity = Velocity { x: 0., y: 0., z: 0. };
        state_wrapper.ball.angular_velocity = Velocity { x: 0., y: 0., z: 0. };
    }
}

