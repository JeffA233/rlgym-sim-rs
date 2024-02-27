use rocketsim_rs::{
    cxx::UniquePtr,
    math::{RotMat, Vec3},
    sim::{Arena, CarConfig, CarControls, Team},
    GameState as GameState_sim, BoostPad,
};
// use std::cell::RefCell;
use std::{collections::HashMap, sync::RwLock};

use crate::{
    common_values::{BLUE_TEAM, GRAVITY_Z, ORANGE_TEAM, ROCKETSIM_BOOST_PER_SEC},
    gamestates::{
        game_state::GameState as GameState_rlgym,
        physics_object::{PhysicsObject, Position, Velocity},
        player_data::PlayerData,
    },
    state_setters::wrappers::{
        state_wrapper::StateWrapper, 
        // car_wrapper::CarWrapper
    }, 
    envs::game_match::GameConfig,
};

/// used as a means to store stats for a particular agent
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub goals: u32,
    pub own_goals: u32,
    pub assists: u32,
    pub demolitions: u32,
    pub demoed: u32,
    pub shots: u32,
    pub saves: u32,
    pub last_bumped_by_id: u32,
    pub last_car_bumped_id: u32,
    pub bumps_count: u32,
    pub bumped_count: u32,
}

pub struct RocketsimWrapper {
    arena: UniquePtr<Arena>,
    car_ids: Vec<u32>,
    tick_skip: usize,
    car_config: &'static CarConfig,
    // blue_score: i32,
    // orange_score: i32,
    jump_timer: f32,
    prev_touched_ticks: HashMap<u32, u64>,
    car_id_map: HashMap<u32, i32>,
}

impl RocketsimWrapper {
    thread_local!(
        static BLUE_SCORE: RwLock<i32> = RwLock::new(0);
        static ORANGE_SCORE: RwLock<i32> = RwLock::new(0);
        static LAST_GOAL_TICK: RwLock<u64> = RwLock::new(0);
        static STATS: RwLock<Vec<(u32, Stats)>> = RwLock::new(Vec::new());
    );

    pub fn new(config: GameConfig) -> Self {
        // TODO: input more game config stuff here
        // rocketsim start
        // required only once for all threads so we should do it before the multithreading parts instead of here
        // rocketsim_rs::init(None);
        let mut rocket_sim_instance = Arena::default_standard();

        let mut sim_mutator_config = rocket_sim_instance.get_mutator_config();
        sim_mutator_config.gravity.z = GRAVITY_Z * config.gravity;
        sim_mutator_config.boost_used_per_second = ROCKETSIM_BOOST_PER_SEC * config.boost_consumption;
        rocket_sim_instance.pin_mut().set_mutator_config(sim_mutator_config);

        rocket_sim_instance.pin_mut().reset_to_random_kickoff(None);
        let mut car_ids = Vec::new();
        let mut car_id_map = HashMap::new();
        if config.spawn_opponents {
            let mut i = 1;
            // spawn blue cars
            for _ in 0..config.team_size {
                let car_id = rocket_sim_instance.pin_mut().add_car(Team::BLUE, config.car_config);
                car_id_map.insert(car_id, i);
                car_ids.push(car_id);
                i += 1;
            }
            // spawn orange cars
            for _ in 0..config.team_size {
                let car_id = rocket_sim_instance.pin_mut().add_car(Team::ORANGE, config.car_config);
                car_id_map.insert(car_id, i);
                car_ids.push(car_id);
                i += 1;
            }
        } else {
            let mut i = 1;
            // spawn blue cars
            for _ in 0..config.team_size as u32 {
                let car_id = rocket_sim_instance.pin_mut().add_car(Team::BLUE, config.car_config);
                car_id_map.insert(car_id, i);
                car_ids.push(car_id);
                i += 1;
            }
        }

        // init stats
        Self::STATS.with(|stats| {
            let mut guard = stats.write().unwrap();
            for id in car_ids.iter() {
                guard.push((*id, Stats::default()));
            }
        });

        rocket_sim_instance.pin_mut().set_goal_scored_callback(
            |mut arena, team, tick_skip| {
                let curr_tick = arena.as_mut().get_tick_count();
                let tick_skip = tick_skip as u64;

                // -- This section is for orange and blue scores --
                let last_goal_tick = Self::LAST_GOAL_TICK.with(|val| *val.read().unwrap());

                // make it so that tick skip doesn't count multiple goals scored
                if curr_tick < last_goal_tick + tick_skip {
                    Self::LAST_GOAL_TICK.with(|val| {
                        let mut ref_val = val.write().unwrap();
                        *ref_val = curr_tick;
                    });
                    return;
                }

                if team == Team::BLUE {
                    Self::BLUE_SCORE.with(|val| *val.write().unwrap() += 1);
                } else {
                    Self::ORANGE_SCORE.with(|val| *val.write().unwrap() += 1);
                }

                // value that holds the last tick the goal was scored from
                Self::LAST_GOAL_TICK.with(|val| *val.write().unwrap() = curr_tick);
                // -- end of section --

                // -- start of stats section --
                // section adapted from stat_tracker in bindings made by VirxEC

                // Collect all valid ball touches
                let mut all_ball_touches = arena
                    .as_mut()
                    .get_car_infos()
                    .into_iter()
                    .filter_map(|car_info| {
                        if car_info.state.ball_hit_info.is_valid {
                            Some((car_info.id, car_info.team, car_info.state.ball_hit_info.tick_count_when_hit))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                // Sort by ball touch time
                all_ball_touches.sort_by_key(|(_, _, tick_count_when_hit)| *tick_count_when_hit);

                // Sort ball touches by team
                let ball_touches = [
                    all_ball_touches.iter().filter(|(_, team, _)| *team == Team::BLUE).map(|(id, _, _)| *id).collect::<Vec<_>>(),
                    all_ball_touches
                        .iter()
                        .filter(|(_, team, _)| *team == Team::ORANGE)
                        .map(|(id, _, _)| *id)
                        .collect::<Vec<_>>(),
                ];

                // update stats
                let t_index = team as u8 as usize;

                Self::STATS.with(|stats| {
                    // it's possible no car touched the ball on the team that got the goal
                    // so ensure that were was at least one ball touch
                    if !ball_touches[t_index].is_empty() {
                        // the latest ball touch on the same team is the scorer
                        let scorer = ball_touches[t_index].last().copied().unwrap();
                        // println!("Car {scorer} SCORED");

                        let mut guard = stats.write().unwrap();
                        // +1 to the car's goals stat
                        guard.iter_mut().find(|(id, _)| *id == scorer).unwrap().1.goals += 1;

                        if ball_touches[t_index].len() > 1 {
                            // if there were two ball touches, they get the assist
                            let assist = ball_touches[t_index][ball_touches[t_index].len() - 2];

                            // Get the tick count of when the scorer and assist touched the ball
                            let scorer_tick = arena.as_mut().get_car(scorer).ball_hit_info.tick_count_when_hit;
                            let assist_tick = arena.as_mut().get_car(assist).ball_hit_info.tick_count_when_hit;

                            // ensure that the assist is < 5s before the touch of the scoring player
                            if (scorer_tick - assist_tick) as f32 / arena.get_tick_rate() < 5. {
                                // println!("CAR {assist} got an ASSIST");

                                // +1 to the car's assists stat
                                guard.iter_mut().find(|(id, _)| id == &assist).unwrap().1.assists += 1;
                            }
                        }

                        if let Some(latest_hit_id) = all_ball_touches.last().map(|(id, _, _)| *id) {
                            // if the last hit was not the scorer, they get the own goal
                            // rocket league tracks this stat in secret and isn't shown on the scoreboard
                            if latest_hit_id != scorer {
                                // println!("CAR {latest_hit_id} OWN GOALED");

                                // +1 to the car's own goals stat
                                guard.iter_mut().find(|(id, _)| *id == latest_hit_id).unwrap().1.own_goals += 1;
                            }
                        }
                    }
                });
                // -- end of stats section --
            },
            config.tick_skip,
        );

        rocket_sim_instance.pin_mut().set_car_bump_callback(
            |_, bumper, victim, is_demo, _| {
                Self::STATS.with(|stats| {
                    let mut guard = stats.write().unwrap();
                    // get bumper stats
                    let stats_for_bumper_id = guard.iter_mut().find(|(id, _)| *id == bumper).unwrap();
                    // bumper stats adjustment
                    if is_demo {
                        stats_for_bumper_id.1.demolitions += 1;
                    }
                    stats_for_bumper_id.1.bumps_count += 1;
                    stats_for_bumper_id.1.last_car_bumped_id = victim;

                    // get victim stats
                    let stats_for_victim_id = guard.iter_mut().find(|(id, _)| *id == victim).unwrap();
                    // victim stats adjustment
                    if is_demo {
                        stats_for_victim_id.1.demoed += 1;
                    }
                    stats_for_victim_id.1.bumped_count += 1;
                    stats_for_victim_id.1.last_bumped_by_id = bumper;
                });
            },
            0,
        );

        RocketsimWrapper {
            arena: rocket_sim_instance,
            car_ids,
            tick_skip: config.tick_skip,
            car_config: config.car_config,
            jump_timer: 1.25,
            prev_touched_ticks: HashMap::new(),
            car_id_map,
        }
    }

    pub fn set_state(&mut self, state_wrapper: StateWrapper, get_sim_state: bool) -> (GameState_rlgym, Option<GameState_sim>) {
        let mut sim_state = self.arena.pin_mut().get_game_state();

        // reset boost pads
        for (i, pad) in sim_state.pads.iter_mut().enumerate() {
            pad.state = state_wrapper.pads[i];
            // pad.state.cooldown = 0.;
        };

        // cars

        // if we need to go from rlgym id to rocketsim id
        // let mut reverse_id_map = HashMap::new();
        // self.car_id_map.iter().for_each(|(k, v)| {reverse_id_map.insert(*v, *k);});
        //
        for car_info in sim_state.cars.iter_mut() {
            // find the rocketsim car with the correct id
            let rlgym_id = *self.car_id_map.get(&(car_info.id)).unwrap();
            let car_wrapper_op = state_wrapper.cars.iter().find(|car| car.get_car_id() == rlgym_id);
            let car_wrapper = match car_wrapper_op {
                Some(val) => val,
                None => panic!("Unable to find car that had the correct id in the sim from the state wrapper. This is likely an error from the state setter.")
            };
            // dbg
            // let mut car_wrapper = &CarWrapper::new(None, None, None);
            // for car in state_wrapper.cars.iter() {
            //     if car.id == rlgym_id {
            //         car_wrapper = car;
            //         break
            //     }
            // };
            //

            car_info.state.pos = Vec3::new(car_wrapper.position.x, car_wrapper.position.y, car_wrapper.position.z);
            car_info.state.vel = Vec3::new(car_wrapper.linear_velocity.x, car_wrapper.linear_velocity.y, car_wrapper.linear_velocity.z);
            car_info.state.ang_vel = Vec3::new(car_wrapper.angular_velocity.x, car_wrapper.angular_velocity.y, car_wrapper.angular_velocity.z);

            let wrapper_rot_mtx = car_wrapper.rotation.euler_to_rotation();
            car_info.state.rot_mat = RotMat {
                forward: Vec3::new(wrapper_rot_mtx.array[0][0], wrapper_rot_mtx.array[1][0], wrapper_rot_mtx.array[2][0]),
                right: Vec3::new(wrapper_rot_mtx.array[0][1], wrapper_rot_mtx.array[1][1], wrapper_rot_mtx.array[2][1]),
                up: Vec3::new(wrapper_rot_mtx.array[0][2], wrapper_rot_mtx.array[1][2], wrapper_rot_mtx.array[2][2]),
            };

            if self.arena.get_mutator_config().boost_used_per_second == 0. {
                car_info.state.boost = 100.;
            } else {
                car_info.state.boost = car_wrapper.boost * 100.;
            }

            self.arena.pin_mut().set_car_controls(car_info.id, CarControls::default()).unwrap();
        }

        // ball
        sim_state.ball.pos = Vec3::new(state_wrapper.ball.position.x, state_wrapper.ball.position.y, state_wrapper.ball.position.z);
        sim_state.ball.vel = Vec3::new(
            state_wrapper.ball.linear_velocity.x,
            state_wrapper.ball.linear_velocity.y,
            state_wrapper.ball.linear_velocity.z,
        );
        sim_state.ball.ang_vel = Vec3::new(
            state_wrapper.ball.angular_velocity.x,
            state_wrapper.ball.angular_velocity.y,
            state_wrapper.ball.angular_velocity.z,
        );

        self.arena.pin_mut().set_game_state(&sim_state).unwrap();

        // println!("Set ball state");
        // let sim_state = self.arena.pin_mut().get_game_state();
        // let gamestate = self.decode_gamestate(sim_state);
        self.get_rlgym_gamestate(get_sim_state)
    }

    // used for state setting from RLViser (which returns a sim state)
    pub fn set_state_sim(&mut self, sim_state: GameState_sim) -> GameState_rlgym {
        self.arena.pin_mut().set_game_state(&sim_state).unwrap();
        self.decode_gamestate(&sim_state)
    }

    fn decode_gamestate(&mut self, sim_gamestate: &GameState_sim) -> GameState_rlgym {
        let curr_tick = self.arena.get_tick_count();

        let mut ball = PhysicsObject::new();
        ball.position = Position {
            x: sim_gamestate.ball.pos.x,
            y: sim_gamestate.ball.pos.y,
            z: sim_gamestate.ball.pos.z,
        };
        ball.linear_velocity = Velocity {
            x: sim_gamestate.ball.vel.x,
            y: sim_gamestate.ball.vel.y,
            z: sim_gamestate.ball.vel.z,
        };
        ball.angular_velocity = Velocity {
            x: sim_gamestate.ball.ang_vel.x,
            y: sim_gamestate.ball.ang_vel.y,
            z: sim_gamestate.ball.ang_vel.z,
        };

        let mut inverted_ball = PhysicsObject::new();
        inverted_ball.position = Position {
            x: -sim_gamestate.ball.pos.x,
            y: -sim_gamestate.ball.pos.y,
            z: sim_gamestate.ball.pos.z,
        };
        inverted_ball.linear_velocity = Velocity {
            x: -sim_gamestate.ball.vel.x,
            y: -sim_gamestate.ball.vel.y,
            z: sim_gamestate.ball.vel.z,
        };
        inverted_ball.angular_velocity = Velocity {
            x: -sim_gamestate.ball.ang_vel.x,
            y: -sim_gamestate.ball.ang_vel.y,
            z: sim_gamestate.ball.ang_vel.z,
        };

        let mut players = Vec::with_capacity(sim_gamestate.cars.len());

        let orange_score = Self::ORANGE_SCORE.with(|val| *val.read().unwrap());
        let blue_score = Self::BLUE_SCORE.with(|val| *val.read().unwrap());

        for car_info in &sim_gamestate.cars {
            let car = car_info.state;

            let mut car_data = PhysicsObject::new();
            car_data.position = Position {
                x: car.pos.x,
                y: car.pos.y,
                z: car.pos.z,
            };
            car_data.linear_velocity = Velocity {
                x: car.vel.x,
                y: car.vel.y,
                z: car.vel.z,
            };
            car_data.angular_velocity = Velocity {
                x: car.ang_vel.x,
                y: car.ang_vel.y,
                z: car.ang_vel.z,
            };
            
            car_data.rotation_mtx.array[0] = [car.rot_mat.forward.x, car.rot_mat.right.x, car.rot_mat.up.x];
            car_data.rotation_mtx.array[1] = [car.rot_mat.forward.y, car.rot_mat.right.y, car.rot_mat.up.y];
            car_data.rotation_mtx.array[2] = [car.rot_mat.forward.z, car.rot_mat.right.z, car.rot_mat.up.z];
            car_data.has_computed_rot_mtx = true;

            car_data.quaternion = car_data.rotation_mtx.rotation_to_quaternion();
            
            car_data.euler_angles = car_data.quaternion.quat_to_euler();
            car_data.has_computed_euler_angles = true;

            let mut inverted_car_data = PhysicsObject::new();
            inverted_car_data.position = Position {
                x: -car.pos.x,
                y: -car.pos.y,
                z: car.pos.z,
            };
            inverted_car_data.linear_velocity = Velocity {
                x: -car.vel.x,
                y: -car.vel.y,
                z: car.vel.z,
            };
            inverted_car_data.angular_velocity = Velocity {
                x: -car.ang_vel.x,
                y: -car.ang_vel.y,
                z: car.ang_vel.z,
            };
            inverted_car_data.quaternion = car_data.quaternion.invert();
            // no particular reason to do this I think other than to match that car_data also has a computed rot_mtx
            // previously the behavior was that each clone of the gamestate would have to recompute if not already computed
            inverted_car_data.rotation_mtx = car_data.rotation_mtx.invert();
            // inverted_car_data.rotation_mtx = inverted_car_data.quaternion.quat_to_rot_mtx();
            inverted_car_data.has_computed_rot_mtx = true;

            inverted_car_data.euler_angles = inverted_car_data.quaternion.quat_to_euler();
            inverted_car_data.has_computed_euler_angles = true;

            // need to make sure there is a value since the hashmap may not be populated yet
            let prev_touched_ticks_op = self.prev_touched_ticks.get(&car_info.id);
            let prev_touched_tick = match prev_touched_ticks_op {
                Some(val) => *val,
                None => {
                    self.prev_touched_ticks.insert(car_info.id, 0);
                    0
                }
            };

            let stats = Self::STATS.with(|stats| {
                let guard = stats.read().unwrap();
                guard.iter().find(|(id, _)| *id == car_info.id).unwrap().1
            });

            // to get the last time the ball was touched by this player, otherwise tick = 0
            let last_touch_tick = if car.ball_hit_info.is_valid {
                if prev_touched_tick != car.ball_hit_info.tick_count_when_hit {
                    self.prev_touched_ticks.insert(car_info.id, car.ball_hit_info.tick_count_when_hit).unwrap()
                } else {
                    prev_touched_tick
                }
            } else {
                0
            };
            
            let car_id_op = self.car_id_map.get(&car_info.id);
            let car_id = match car_id_op {
                Some(val) => *val,
                None => panic!("unable to find id in car id map")
            };

            let car_bumped_by_id_op = self.car_id_map.get(&stats.last_bumped_by_id);
            let car_bumped_by_id = match car_bumped_by_id_op {
                Some(val) => *val,
                None => 0,
            };
            let car_bumpee_id_op = self.car_id_map.get(&stats.last_car_bumped_id);
            let car_bumpee_id = match car_bumpee_id_op {
                Some(val) => *val,
                None => 0,
            };
            
            let player = PlayerData {
                car_id: car_id as i32,
                team_num: if car_info.team == Team::BLUE { BLUE_TEAM } else { ORANGE_TEAM },
                match_goals: (orange_score + blue_score) as i64,
                // TODO: adapt PlayerData struct to structs that represent better RocketSim data
                match_saves: stats.saves as i64,
                match_shots: stats.shots as i64,
                match_demolishes: stats.demolitions as i64,
                boost_pickups: 0,
                is_demoed: car.is_demoed,
                last_bumped_by: car_bumped_by_id as u32,
                last_bumpee: car_bumpee_id as u32,
                bumps: stats.bumps_count,
                been_bumped: stats.bumped_count,
                on_ground: car.is_on_ground,
                // ball_touched: if self.prev_touched_ticks != car.ball_hit_info.tick_count_when_hit && !car.ball_hit_info.is_valid { self.prev_touched_ticks = car.ball_hit_info.tick_count_when_hit; true } else { false },
                ball_touched: if car.ball_hit_info.is_valid {
                    prev_touched_tick != car.ball_hit_info.tick_count_when_hit
                } else {
                    false
                },
                ball_info: car.ball_hit_info,
                has_jump: !car.has_jumped,
                has_flip: car.air_time_since_jump < self.jump_timer && !(car.has_flipped || car.has_double_jumped),
                boost_amount: (car.boost / 100.),
                car_data,
                inverted_car_data,
                last_ball_touch_tick: last_touch_tick,
                last_actions: car_info.state.last_controls,
            };
            players.push(player);
        }
        players.sort_unstable_by_key(|p| p.car_id);

        // TODO: make this just the actual boost pad stats instead of is_active
        let mut pad_vec = [BoostPad::default(); 34];
        for (pad, vec_item) in sim_gamestate.pads.iter().zip(&mut pad_vec) {
            *vec_item = *pad;
        }
        let mut pad_reversed = pad_vec;
        pad_reversed.reverse();
        GameState_rlgym {
            game_type: 0,
            blue_score,
            orange_score,
            last_touch: 0,
            players,
            ball,
            inverted_ball,
            boost_pads: pad_vec,
            inverted_boost_pads: pad_reversed,
            tick_num: curr_tick,
        }
    }

    pub fn set_game_config(&mut self, new_config: GameConfig, get_sim_state: bool) -> (GameState_rlgym, Option<GameState_sim>) {
        let mut sim_mutator_config = self.arena.get_mutator_config();
        sim_mutator_config.gravity.z = GRAVITY_Z * new_config.gravity;
        sim_mutator_config.boost_used_per_second = ROCKETSIM_BOOST_PER_SEC * new_config.boost_consumption;
        self.arena.pin_mut().set_mutator_config(sim_mutator_config);

        let mut car_ids = self.arena.get_cars();
        let mut car_blue = 0;
        let mut car_orange = 0;
        for car_id in car_ids.iter() {
            let team = self.arena.get_car_team(*car_id);
            if team == Team::ORANGE {
                car_orange += 1;
            } else {
                car_blue += 1;
            }
        }

        // check if car count is the same so that we don't update
        // let car_count = if new_config.spawn_opponents {
        //     new_config.team_size * 2
        // } else {
        //     new_config.team_size
        // };
        let car_count_blue = new_config.team_size;
        let car_count_orange = if new_config.spawn_opponents {new_config.team_size} else {0};

        // NOTE: need to check if the old car and new car settings are the same, sadly don't think we can check via if the reference points to the same thing?
        let new_hitbox_size = new_config.car_config.hitbox_size;
        let old_hitbox_size = self.car_config.hitbox_size;

        if car_blue != car_count_blue || car_orange != car_count_orange || new_hitbox_size == old_hitbox_size {
            for car_id in car_ids.iter() {
                let err = self.arena.pin_mut().remove_car(*car_id);
                match err {
                    Ok(val) => val,
                    Err(err) => println!("unable to remove car: {err}")
                };
            }

            car_ids.clear();
            self.car_id_map.clear();
    
            // let mut car_ids = Vec::new();
            if new_config.spawn_opponents {
                let mut i = 1;
                // spawn blue cars
                for _ in 0..new_config.team_size {
                    let car_id = self.arena.pin_mut().add_car(Team::BLUE, new_config.car_config);
                    self.car_id_map.insert(car_id, i);
                    car_ids.push(car_id);
                    i += 1;
                }
                // spawn orange cars
                for _ in 0..new_config.team_size {
                    let car_id = self.arena.pin_mut().add_car(Team::ORANGE, new_config.car_config);
                    self.car_id_map.insert(car_id, i);
                    car_ids.push(car_id);
                    i += 1;
                }
            } else {
                let mut i = 1;
                // spawn blue cars
                for _ in 0..new_config.team_size as u32 {
                    let car_id = self.arena.pin_mut().add_car(Team::BLUE, new_config.car_config);
                    self.car_id_map.insert(car_id, i);
                    car_ids.push(car_id);
                    i += 1;
                }
            }
        }
        // for car_id in car_ids {
        //     let err = self.arena.pin_mut().remove_car(car_id);
        //     match err {
        //         Ok(val) => val,
        //         Err(err) => println!("unable to remove car: {err}")
        //     };
        // }

        // let mut car_ids = Vec::new();
        // if new_config.spawn_opponents {
        //     // spawn blue cars
        //     for _ in 0..new_config.team_size {
        //         car_ids.push(self.arena.pin_mut().add_car(Team::BLUE, CarConfig::octane()));
        //     }
        //     // spawn orange cars
        //     for _ in 0..new_config.team_size {
        //         car_ids.push(self.arena.pin_mut().add_car(Team::ORANGE, CarConfig::octane()));
        //     }
        // } else {
        //     // spawn blue cars
        //     for _ in 0..new_config.team_size {
        //         car_ids.push(self.arena.pin_mut().add_car(Team::BLUE, CarConfig::octane()));
        //     }
        // }

        self.arena.pin_mut().reset_to_random_kickoff(None);

        // init stats
        Self::STATS.with(|stats| {
            let mut guard = stats.write().unwrap();
            guard.clear();
            for id in car_ids.iter() {
                guard.push((*id, Stats::default()));
            }
        });

        self.car_ids = car_ids;
        self.tick_skip = new_config.tick_skip;
        self.car_config = new_config.car_config;

        self.get_rlgym_gamestate(get_sim_state)
    }

    pub fn get_rlgym_gamestate(&mut self, get_sim_state: bool) -> (GameState_rlgym, Option<GameState_sim>) {
        let rlsim_gamestate = self.arena.pin_mut().get_game_state();
        if get_sim_state {
            (self.decode_gamestate(&rlsim_gamestate), Some(rlsim_gamestate))
        } else {
            (self.decode_gamestate(&rlsim_gamestate), None)
        }
        // self.decode_gamestate(&rlsim_gamestate)
    }

    /// clone actions before this to set prev_acts
    pub fn step(&mut self, actions: Vec<Vec<f32>>, get_sim_state: bool) -> (GameState_rlgym, Option<Vec<GameState_sim>>) {
        let mut acts = Vec::<(u32, CarControls)>::new();

        // package spectator ids with the corresponding action to send to arena
        for (spectator_id, action) in self.car_ids.iter().zip(actions) {
            acts.push((
                *spectator_id,
                CarControls {
                    throttle: action[0],
                    steer: action[1],
                    pitch: action[2],
                    yaw: action[3],
                    roll: action[4],
                    jump: action[5] > 0.,
                    boost: action[6] > 0.,
                    handbrake: action[7] > 0.,
                },
            ));
        }

        self.arena.pin_mut().set_all_controls(&acts).unwrap();

        self.arena.pin_mut().step(1);

        let (gamestate_rlgym, gamestate_sim) = self.get_rlgym_gamestate(get_sim_state);

        // originally was here
        // self.arena.pin_mut().step(self.tick_skip as i32);

        // TODO: need to somehow extract ball hit information from every step probably
        if get_sim_state {
            let mut gamestate_sim_vec = Vec::new();
            gamestate_sim_vec.push(gamestate_sim.unwrap());
            
            if self.tick_skip > 1 {
                for _ in 0..self.tick_skip-1 {
                    self.arena.pin_mut().step(1);
                    gamestate_sim_vec.push(self.arena.pin_mut().get_game_state());
                }
            }

            (gamestate_rlgym, Some(gamestate_sim_vec))
        } else {
            if self.tick_skip > 1 {
                self.arena.pin_mut().step(self.tick_skip as i32 - 1);
            }

            (gamestate_rlgym, None)
        }
    }
}
