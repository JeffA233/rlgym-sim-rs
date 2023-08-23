// use ndarray::*;
use std::{
    // thread::JoinHandle, 
    time::*};

// use std::collections::HashMap;
// use action_parsers::necto_parsers_3::NectoAction;
// use rlgym_sim_rs::action_parsers::necto_parser_2::NectoAction;
use rlgym_sim_rs::action_parsers::test_parser::TestAction;
// use communication::communication_handler::f32vec_as_u8_slice;
use rlgym_sim_rs::conditionals::custom_conditions::CombinedTerminalConditions;
// use rlgym_sim_rs::envs::game_match::GameMatch;
use rlgym_sim_rs::gamestates::game_state::GameState;
// use rlgym_sim_rs::gamestates::physics_object::Position;
use rlgym_sim_rs::make;
use rlgym_sim_rs::obs_builders::aspo4_array_2::AdvancedObsPadderStacker2;
use rlgym_sim_rs::reward_functions::custom_rewards::get_custom_reward_func_tester;
use rlgym_sim_rs::state_setters::default_state::{
    AgentBallHitStateTester, 
    BlueGoalStateTester, 
    OrangeGoalStateTester, 
    // ExactStateTester, 
    DemoStateTester
};
// use state_setters::random_state::RandomState;

use rlgym_sim_rs::{
    // action_parsers::discrete_act::DiscreteAction,
    // conditionals::common_conditions::TimeoutCondition,
    // envs::game_match::GameConfig,
    // gamestates::physics_object::RotationMatrix,
    obs_builders::obs_builder::ObsBuilder,
    // reward_functions::default_reward::RewardFn,
    state_setters::{
        // custom_state_setters::custom_state_setters,
        default_state::{
            // DefaultState, 
            DefaultStateTester},
    },
};
// use crate::state_setters::state_setter::StateSetter;

// pub mod action_parsers;
// pub mod common_values;
// pub mod communication;
// pub mod conditionals;
// pub mod envs;
// pub mod gamelaunch;
// pub mod gamestates;
// pub mod math;
// pub mod obs_builders;
// pub mod reward_functions;
// pub mod state_setters;
// pub mod gym;
// pub mod make;
// pub mod sim_wrapper;
// use std::fs::File;
// use std::fs::*;
// use std::io::{BufWriter, Write, stdin};
// use std::env::*;
// use std::path::Path;
// use std::{thread, time};
// use crossbeam_channel::{bounded, unbounded, Sender, Receiver};

// use gamelaunch::launch;

// math.norm_func();

#[test]
fn main() {
    // let v = vec![93.0];
    // let bytes = f32vec_as_u8_slice(&v);
    // let mut rot_mtx = RotationMatrix::zeros();
    // rot_mtx.array[0][0] = 0.;
    // rot_mtx.array[1][0] = 1.;
    // rot_mtx.array[2][0] = 2.;
    // rot_mtx.array[0][1] = 3.;
    // rot_mtx.array[1][1] = 4.;
    // rot_mtx.array[2][1] = 5.;
    // rot_mtx.array[0][2] = 6.;
    // rot_mtx.array[1][2] = 7.;
    // rot_mtx.array[2][2] = 8.;

    // let rot_mtx_flat = rot_mtx.into_flat_array();
    // let rot_arr = rot_mtx.column(0);

    let obs = vec![vec![93, 93, 93], vec![92, 93, 93], vec![91, 93, 93]];
    let mut vec = Vec::<Vec<i32>>::new();
    vec.extend(obs);
    let term_cond = Box::new(CombinedTerminalConditions::new(1));
    // let term_cond = Box::new(TimeoutCondition::new(225));
    let reward_fn = get_custom_reward_func_tester();
    let obs_build: Box<dyn ObsBuilder + Send> = Box::new(AdvancedObsPadderStacker2::new(None, Some(0)));
    let obs_build_vec = vec![obs_build];
    let act_parse = Box::new(TestAction::new());
    // let act_parse = Box::new(DiscreteAction::new());
    // let act_parse_2 = Box::new(OldNectoAction::new());
    // let size = act_parse.get_action_space();
    // let size_old = act_parse_2.get_action_space();
    // let state_set = Box::new(custom_state_setters(1));
    let state_set = Box::new(DefaultStateTester::new());
    let mut actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]];
    // let actions2 = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.], vec![2., 1., 0., 1., 0., 1., 0., 1.]];
    rocketsim_rs::init(None);
    let tick_skip = 1;
    let game_config = make::MakeConfig {
        tick_skip: Some(tick_skip),
        spawn_opponents: Some(false),
        team_size: Some(1),
        gravity: None,
        boost_consumption: None,
        terminal_condition: term_cond,
        reward_fn,
        obs_builder: obs_build_vec,
        action_parser: act_parse,
        state_setter: state_set, 
    };
    let mut gym = make::make(game_config);

    // let obs = gym.reset(None, None);
    // let last_state = gym._prev_state.clone();
    // last_state;
    // obs;
    // let (_obs, reward, done, _info) = gym.step(actions2);
    // let store = _obs;
    // store;

    // --TESTING OF MATCH/REWARDS/ETC.--
    // let match_ = gym._game_match;
    // let mut match_ = GameMatch::new(reward_fn,
    //     term_cond,
    //     obs_build,
    //     act_parse,
    //     state_set,
    //     Some(2),
    //     Some(8),
    //     Some(100.),
    //     Some(1.),
    //     Some(1.),
    //     Some(false));

    // --TESTING OF OBS---------------------------------------------------------------------------------------------------
    // (not adequate enough to be honest)
    // let fake_state = GameState::new_test();
    // let acts = act_parse.parse_actions(actions, &fake_state);
    // acts;
    // let fake_action = vec![0., 1., 0., 0., 0., 0., 0., 0.];
    // let game_config = GameConfig {
    //     game_speed: 1.,
    //     gravity: 1.,
    //     boost_consumption: 1.,
    //     team_size: 3,
    //     tick_skip: 6,
    //     spawn_opponents: true
    // };

    // for i in 0..100 {
    //     let built_obs = obs_build.build_obs(&fake_state.players[0], &fake_state, &game_config, &fake_action);
    //     built_obs;
    // }

    // let mut obs_thread_vec = Vec::<Sender<Manager>>::new();
    // let (send, recv_local) = unbounded();
    // for i in 0..6 {
    //     let fake_state_clone = fake_state.clone();
    //     let send_nonlocal = send.clone();
    //     let (send, recv) = unbounded();
    //     let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    //     let reward_fn = get_custom_reward_func();
    //     obs_thread_vec.push(send);

    //     thread::spawn(move || {
    //         worker(send_nonlocal, recv, fake_state_clone, obs_build, reward_fn);
    //     });
    // }

    // let start_time = Instant::now();
    // for i in 0..100000 {
    //     for thrd in &obs_thread_vec {
    //         thrd.send(Manager::Step);
    //     }

    //     for x in 0..obs_thread_vec.len() {
    //         recv_local.recv();
    //     }
    // }
    // let duration = start_time.elapsed();
    // let seconds_elapsed = duration.as_secs_f64();
    // println!("seconds elapsed with threads: {seconds_elapsed}");
    // let fps = (120.*360.)/seconds_elapsed;
    // println!("fps: {fps}");

    // let mut obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    // let mut reward_fn = get_custom_reward_func();

    // let start_time = Instant::now();
    // let mut obs;
    // // let mut rew;
    // for i in 0..100000 {
    //     for x in 0..obs_thread_vec.len() {
    //         obs = obs_build.build_obs(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
    //         // rew = reward_fn.get_reward(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
    //     }
    // }
    // let duration = start_time.elapsed();
    // let seconds_elapsed = duration.as_secs_f64();
    // println!("seconds elapsed with no threads: {seconds_elapsed}");
    // let fps = (120.*360.)/seconds_elapsed;
    // println!("fps: {fps}");

    // end of threaded obs testing ---------------------------------------------------------------------------------------------------

    // seconds elapsed with threads: 12.8681321
    // seconds elapsed with no threads: 13.5231119

    // obs only
    // seconds elapsed with threads: 3.6690481999999998
    // seconds elapsed with no threads: 4.141068

    // obs update
    // not reliable? maybe because optimizations and cutting corners, thinking debug shows the behavior better than run
    // seconds elapsed with threads: 0.9143468
    // fps: 47246.84332028066
    // seconds elapsed with no threads: 1.8254426000000001
    // fps: 23665.49350825931

    // pub enum Manager {
    //     Step
    // }

    // /// packet that comes from the worker
    // pub enum Worker {
    //     StepRet {obs: Vec<f64>, rew: f64}
    // }

    // pub fn worker(send_chan: Sender<Worker>, rec_chan: Receiver<Manager>, fake_state: GameState, game_config: GameConfig, mut obs_builder: Box<AdvancedObsPadderStacker2>, mut reward_fn: Box<dyn RewardFn + Send>) {
    //     loop {
    //         // simple loop that tries to recv for as long as the Manager channel is not hung up waiting for commands from the Manager
    //         let obs: Vec<f64>;
    //         // let rew: f64;
    //         let recv_data = rec_chan.recv();
    //         let cmd = match recv_data {
    //             Ok(out) => out,
    //             Err(err) => {
    //                 println!("recv err in worker: {err}");
    //                 break;
    //             }
    //         };
    //         match cmd {
    //             Manager::Step => {
    //                 obs = obs_builder.build_obs(&fake_state.players[0], &fake_state, &game_config, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
    //                 // rew = reward_fn.get_reward(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
    //                 send_chan.send(Worker::StepRet { obs, rew: 0. }).unwrap();
    //             }
    //         };
    //     }
    // }

    // let obs = match_.build_observations(&mut fake_state);
    // let mut out;
    // out = act_parse.parse_actions(vec![vec![43., 50.]], &fake_state);
    // for i in 0..89 {
    //     let act_vec: Vec<Vec<f32>> = vec![vec![i as f32; 2]];
    //     out = act_parse.parse_actions(act_vec, &fake_state);
    // }
    // match_.episode_reset(&fake_state);
    // let obs = match_.build_observations(&mut fake_state);
    // let rew_f32: f32 = rew.iter().sum();
    // println!("{rew_f32}");
    // --END--
    // gym.reset(None, None);
    // gym.step(actions.clone());

    // let mut state_vec: Vec<GameState> = Vec::new();
    // state_vec.push(gym._prev_state.clone());
    // let mut rew_val: f64 = 0.;
    // let start_time = Instant::now();
    // for _i in 0..(120 * 360) {
    //     let (_obs, reward, done, _info) = gym.step(actions.clone());
    //     let state = gym._prev_state.clone();
    //     if done {
    //         gym.reset(None, None);
    //     }
    //     let info_val = *_info.get("result").unwrap();
    //     if info_val > 0. {
    //         println!("result: {info_val}")
    //     }
    //         let val = reward[0];
    //         let val_limit = 0.01;
    //         // if val > val_limit {
    //         //     println!("val was greater than {val_limit}: {val}, at step {_i}")
    //         // }
    //         if _i % 200 == 0 {
    //             let i_val = _i;
    //             state_vec.push(gym._prev_state.clone());
    //         }
    //     rew_val += val;
    // }
    // // let (_obs, reward, done, _info) = gym.step(actions.clone());
    // let duration = start_time.elapsed();
    // let seconds_elapsed = duration.as_secs_f64();
    // println!("seconds elapsed: {seconds_elapsed}");
    // let fps = (120.*360.)/seconds_elapsed;
    // println!("fps: {fps}");
    // println!("rewards: {rew_val}");

    // now let's make sure blue goals are working ---------------------------------------------------------------------------------------------------
    gym._game_match._state_setter = Box::new(BlueGoalStateTester::new());
    gym.reset(None, None);
    // gym.step(actions.clone());

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut last_blue_score = 0;
    let mut last_blue_score_tick = 0;
    let mut last_done_tick = 0;
    for _i in 0..(120 * 50) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        let mut state = gym._prev_state.clone();
        if done {
            assert!(last_done_tick + ((tick_skip*2) as u64) < state.tick_num, "scored within {tick_skip}*2 ticks which is too close");
            last_done_tick = state.tick_num;
            gym.reset(None, None);
            state = gym._prev_state.clone();
        }
        if state.blue_score != last_blue_score {
            // if the done flag was not shown then we messed up
            assert!(done, "done flag was not shown");

            assert_ne!(last_blue_score_tick, state.tick_num, "scored multiple times on the same state");
            assert!(last_blue_score_tick + ((tick_skip) as u64) < state.tick_num, "scored within {tick_skip} ticks which is too close");
            last_blue_score_tick = state.tick_num;

            assert_eq!(
                last_blue_score + 1,
                state.blue_score,
                "goal was double counted, previous score: {last_blue_score}, current score: {}",
                state.blue_score
            );
            last_blue_score = state.blue_score;
        }
        // let info_val = *_info.get("result").unwrap();
        // if info_val > 0. {
        //     println!("result: {info_val}")
        // }
        let val = reward[0];
        // let val_limit = 0.01;
        // if val > val_limit {
        //     println!("val was greater than {val_limit}: {val}, at step {_i}")
        // }
        if _i % 200 == 0 {
            let i_val = _i;
            state_vec.push(gym._prev_state.clone());
        }
        rew_val += val;
    }
    assert_ne!(last_blue_score, 0, "did not detect a goal scored state");
    // let (_obs, reward, done, _info) = gym.step(actions.clone());
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    // now let's make sure orange goals are working ---------------------------------------------------------------------------------------------------
    gym._game_match._state_setter = Box::new(OrangeGoalStateTester::new());
    gym.reset(None, None);
    // gym.step(actions.clone());

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut last_orange_score = 0;
    for _i in 0..(120 * 50) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        let mut state = gym._prev_state.clone();
        if state.orange_score != last_orange_score {
            // if the done flag was not shown then we messed up
            assert!(done);
            // let test = 0;
            assert_eq!(last_orange_score + 1, state.orange_score);
            last_orange_score = state.orange_score;
        }
        if done {
            gym.reset(None, None);
            state = gym._prev_state.clone();
        }
        // let info_val = *_info.get("result").unwrap();
        // if info_val > 0. {
        //     println!("result: {info_val}")
        // }
        let val = reward[0];
        // let val_limit = 0.01;
        // if val > val_limit {
        //     println!("val was greater than {val_limit}: {val}, at step {_i}")
        // }
        if _i % 200 == 0 {
            let i_val = _i;
            state_vec.push(gym._prev_state.clone());
        }
        rew_val += val;
    }
    // let (_obs, reward, done, _info) = gym.step(actions.clone());
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    // now let's make sure ball touches are working
    gym._game_match._state_setter = Box::new(AgentBallHitStateTester::new());
    gym.reset(None, None);
    // gym.step(actions.clone());
    actions = vec![vec![2., 0., 0., 0., 0., 0., 0., 0.]];

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut last_orange_score = 0;
    let mut touch_counter = 0;
    let mut prev_distance;
    for _i in 0..(120 * 50) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        let mut state = gym._prev_state.clone();
        if state.orange_score != last_orange_score {
            // if the done flag was not shown then we messed up
            // assert_eq!(done, true);
            // let test = 0;
            // assert_eq!(last_orange_score + 1, state.orange_score);
            last_orange_score = state.orange_score;
        }
        if state.players[0].ball_touched {
            touch_counter += 1;
            prev_distance = (state.players[0].car_data.position - state.ball.position)
                .into_array()
                .iter()
                .map(|val| val.powi(2))
                .sum::<f32>()
                .sqrt();
        }
        if done {
            gym.reset(None, None);
            state = gym._prev_state.clone();
        }
        // let info_val = *_info.get("result").unwrap();
        // if info_val > 0. {
        //     println!("result: {info_val}")
        // }
        let val = reward[0];
        // let val_limit = 0.01;
        // if val > val_limit {
        //     println!("val was greater than {val_limit}: {val}, at step {_i}")
        // }
        if _i % 200 == 0 {
            let i_val = _i;
            state_vec.push(gym._prev_state.clone());
        }
        rew_val += val;
    }
    // let (_obs, reward, done, _info) = gym.step(actions.clone());
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("touches: {touch_counter}");

    // now let's make sure demos are working ---------------------------------------------------------------------------------------------------
    let term_cond = Box::new(CombinedTerminalConditions::new(1));
    // let term_cond = Box::new(TimeoutCondition::new(225));
    let reward_fn = get_custom_reward_func_tester();
    let obs_build: Box<dyn ObsBuilder + Send> = Box::new(AdvancedObsPadderStacker2::new(None, Some(0)));
    let obs_build_vec = vec![obs_build];
    let act_parse = Box::new(TestAction::new());
    let state_set = Box::new(DefaultStateTester::new());
    // let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]];
    let actions2 = vec![vec![1., 0., 0., 0., 0., 0., 1., 0.], vec![1., 0., 0., 0., 0., 0., 1., 0.]];
    // rocketsim_rs::init(None);
    let tick_skip = 1;
    let game_config = make::MakeConfig {
        tick_skip: Some(tick_skip),
        spawn_opponents: Some(false),
        team_size: Some(1),
        gravity: None,
        boost_consumption: None,
        terminal_condition: term_cond,
        reward_fn,
        obs_builder: obs_build_vec,
        action_parser: act_parse,
        state_setter: state_set, 
    };
    let mut gym = make::make(game_config);

    gym._game_match._state_setter = Box::new(DemoStateTester::new());
    gym.reset(None, None);
    // gym.step(actions.clone());

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut demos = 0;
    // let mut last_blue_score_tick = 0;
    // let mut last_done_tick = 0;
    for _i in 0..(120 * 50) {
        let (_obs, reward, done, _info) = gym.step(actions2.clone());
        let mut state = gym._prev_state.clone();
        if done {
            gym.reset(None, None);
            state = gym._prev_state.clone();
        }
        if state.players[0].is_demoed || state.players[1].is_demoed {
            demos += 1;
        } else {
            // just for being able to debug here
            let x = 0;
        }
        // let info_val = *_info.get("result").unwrap();
        // if info_val > 0. {
        //     println!("result: {info_val}")
        // }
        let val = reward[0];
        // let val_limit = 0.01;
        // if val > val_limit {
        //     println!("val was greater than {val_limit}: {val}, at step {_i}")
        // }
        if _i % 200 == 0 {
            let i_val = _i;
            state_vec.push(gym._prev_state.clone());
        }
        rew_val += val;
    }

    // let (_obs, reward, done, _info) = gym.step(actions.clone());
    assert!(demos > 0, "No demos were deteched!");
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    // gym.reset(None);

    // let mut rew_val: f32 = 0.;
    // for _i in 0..(15 * 360) {
    //     let (_obs, reward, done, _info) = gym.step(actions.clone());
    //     if done {
    //         gym.reset(None);
    //     }
    //     let rew_str: String = reward.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    //     // println!("{rew_str}");
    //     rew_val += reward[0];
    // }
    // let end_val = rew_val / (15.*360.);
    // println!("rough reward per tick: {end_val}");
    // println!("closing Rocket League");
    // gym.close();
    // println!("waiting");
    // stdin().read_line(&mut String::new()).unwrap();
}
