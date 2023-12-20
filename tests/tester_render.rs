// use ndarray::*;
use std::{
    // thread::JoinHandle, 
    time::{
        // Duration,
        Instant,
    }};

// use std::collections::HashMap;
use rlgym_sim_rs::action_parsers::test_parser::TestAction;
use rlgym_sim_rs::conditionals::common_conditions::{TimeoutCondition, GoalScoredCondition};
use rlgym_sim_rs::conditionals::terminal_condition::TerminalCondition;
use rlgym_sim_rs::envs::game_match::GameConfig;
// use communication::communication_handler::f32vec_as_u8_slice;
// use rlgym_sim_rs::envs::game_match::GameMatch;
use rlgym_sim_rs::gamestates::game_state::GameState;
// use rlgym_sim_rs::gamestates::physics_object::Position;
use rlgym_sim_rs::make;
use rlgym_sim_rs::obs_builders::advanced_obs::AdvancedObs;
use rlgym_sim_rs::reward_functions::common_rewards::misc_rewards::EventReward;
use rlgym_sim_rs::state_setters::default_state::{
    AgentBallHitStateTester, 
    BlueGoalStateTester, 
    OrangeGoalStateTester, 
    // ExactStateTester, 
    DemoStateTester,
    DefaultStateTesterPitched,
    DefaultStateTester,
};

use rlgym_sim_rs::obs_builders::obs_builder::ObsBuilder;

pub struct CombinedTerminalConditions {
    timeout_condition: TimeoutCondition,
    goal_scored_condition: GoalScoredCondition,
}

impl CombinedTerminalConditions {
    pub fn new(tick_skip: usize) -> Self {
        CombinedTerminalConditions {
            timeout_condition: TimeoutCondition::new(400 * 120 / tick_skip as i64),
            goal_scored_condition: GoalScoredCondition::new(),
        }
    }
}

impl TerminalCondition for CombinedTerminalConditions {
    fn reset(&mut self, _initial_state: &GameState) {
        self.timeout_condition.reset(_initial_state);
        self.goal_scored_condition.reset(_initial_state);
    }

    fn is_terminal(&mut self, current_state: &GameState) -> bool {
        [
            self.timeout_condition.is_terminal(current_state),
            self.goal_scored_condition.is_terminal(current_state),
        ]
        .iter()
        .any(|x| x == &true)
    }
}


#[test]
fn main() {
    let obs = vec![vec![93, 93, 93], vec![92, 93, 93], vec![91, 93, 93]];
    let mut vec = Vec::<Vec<i32>>::new();
    vec.extend(obs);
    // NOTE: this tester breaks without tick_skip = 1, seems to be just simply due to how tick skip works
    let tick_skip = 1;
    let term_cond = Box::new(CombinedTerminalConditions::new(tick_skip));
    let reward_fn = Box::new(EventReward::new(None, None, None, None, None, None, None, None));
    let obs_build: Box<dyn ObsBuilder> = Box::new(AdvancedObs::new());
    let obs_build_vec = vec![obs_build];
    let act_parse = Box::new(TestAction::new());
    let state_set = Box::new(DefaultStateTesterPitched::new());

    let mut actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]];

    rocketsim_rs::init(None);
    
    let config = GameConfig {
        tick_skip,
        spawn_opponents: false,
        team_size: 1,
        gravity: 1.,
        boost_consumption: 1.,
    };
    let game_config = make::MakeConfig {
        game_config: config,
        terminal_condition: term_cond,
        reward_fn,
        obs_builder: obs_build_vec,
        action_parser: act_parse,
        state_setter: state_set, 
    };
    let render_config = make::RenderConfig {
        render: true,
        // 3x as fast as realtime (120 tps)
        update_rate: 360.
    };
    let mut gym = make::make(game_config, Some(render_config));

    // now let's make sure blue goals are working ---------------------------------------------------------------------------------------------------
    gym._game_match._state_setter = Box::new(BlueGoalStateTester::new());
    gym.reset(None, None);

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut last_blue_score = 0;
    let mut last_blue_score_tick = 0;
    let mut last_done_tick = 0;
    for _i in 0..((120/tick_skip) * 5) {
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

        if _i % 200 == 0 {
            state_vec.push(gym._prev_state.clone());
        }
        
        rew_val += reward[0];
    }
    assert_eq!(last_blue_score, 6, "did not get the correct number of goals, got: {}", last_blue_score);
    assert_ne!(last_blue_score, 0, "did not detect a goal scored state");
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    // now let's make sure orange goals are working ---------------------------------------------------------------------------------------------------
    gym._game_match._state_setter = Box::new(OrangeGoalStateTester::new());
    gym.reset(None, None);

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut last_orange_score = 0;
    for _i in 0..((120/tick_skip) * 5) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        let state = gym._prev_state.clone();

        if state.orange_score != last_orange_score {
            // if the done flag was not shown then we messed up
            assert!(done);
            assert_eq!(last_orange_score + 1, state.orange_score);
            last_orange_score = state.orange_score;
        }

        if done {
            gym.reset(None, None);
        }

        if _i % 200 == 0 {
            state_vec.push(gym._prev_state.clone());
        }

        rew_val += reward[0];
    }
    assert_eq!(last_orange_score, 15, "did not get the correct number of goals, got: {}", last_orange_score);
    assert_ne!(last_orange_score, 0, "did not detect a goal scored state");
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    // now let's make sure ball touches are working
    gym._game_match._state_setter = Box::new(AgentBallHitStateTester::new());
    gym.reset(None, None);
    actions = vec![vec![2., 0., 0., 0., 0., 0., 0., 0.]];

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let start_time = Instant::now();
    let mut last_orange_score = 0;
    let mut touch_counter = 0;
    for _i in 0..((120/tick_skip) * 50) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        let state = gym._prev_state.clone();
        if state.orange_score != last_orange_score {
            last_orange_score = state.orange_score;
        }
        if state.players[0].ball_touched {
            touch_counter += 1;
        }
        if done {
            gym.reset(None, None);
        }

        if _i % 200 == 0 {
            state_vec.push(gym._prev_state.clone());
        }
        rew_val += reward[0];
    }

    assert!(touch_counter > 0);
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("touches: {touch_counter}");
    gym.close_renderer();

    // now let's make sure demos are working ---------------------------------------------------------------------------------------------------
    let term_cond = Box::new(CombinedTerminalConditions::new(1));
    let reward_fn = Box::new(EventReward::new(None, None, None, None, None, None, None, None));
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..2 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    let act_parse = Box::new(TestAction::new());
    let state_set = Box::new(DefaultStateTester::new());
    let actions2 = vec![vec![1., 0., 0., 0., 0., 0., 1., 0.], vec![1., 0., 0., 0., 0., 0., 1., 0.]];
    let tick_skip = 1;
    let config = GameConfig {
        tick_skip,
        spawn_opponents: true,
        team_size: 1,
        gravity: 1.,
        boost_consumption: 1.,
    };
    let game_config = make::MakeConfig {
        game_config: config,
        terminal_condition: term_cond,
        reward_fn,
        obs_builder: obs_build_vec,
        action_parser: act_parse,
        state_setter: state_set, 
    };
    let mut gym = make::make(game_config, Some(render_config));

    gym._game_match._state_setter = Box::new(DemoStateTester::new());
    gym.reset(None, None);

    let mut state_vec: Vec<GameState> = Vec::new();
    state_vec.push(gym._prev_state.clone());
    let mut rew_val: f32 = 0.;
    let start_time = Instant::now();
    let mut match_demos = 0;
    let mut demoed = false;
    let mut bumps_count = 0;
    let mut bumped_count = 0;
    let mut last_bumped_id = 0;
    let mut last_bumpee_id = 0;
    for _i in 0..((120/tick_skip) * 50) {
        let (_obs, reward, done, _info) = gym.step(actions2.clone());
        let mut state = gym._prev_state.clone();
        if done {
            gym.reset(None, None);
            state = gym._prev_state.clone();
        }
        if state.players[0].is_demoed {
            match_demos = state.players[1].match_demolishes;
            demoed = true;
            bumps_count = state.players[1].bumps;
            bumped_count = state.players[0].been_bumped;
            last_bumpee_id = state.players[1].last_bumpee;
            last_bumped_id = state.players[0].last_bumped_by;
        }

        if _i % 200 == 0 {
            state_vec.push(gym._prev_state.clone());
        }

        rew_val += reward[0];
    }

    println!("stats for demo test were {{match_demos: {match_demos}, demoed: {demoed} bumped_count: {bumped_count}, bumps_count: {bumps_count}, last_bumpee_id: {last_bumpee_id}, last_bumped_id: {last_bumped_id}}}");
    assert!(match_demos > 0, "No demos were deteched!");
    assert!(bumped_count > 0, "No bumps were detected for player 1!");
    assert!(bumps_count > 0, "No bumps were detected for player 0!");
    assert!(last_bumpee_id != 0, "Bumpee was not detected!");
    assert!(last_bumped_id != 0, "Bumper was not detected!");
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed: {seconds_elapsed}");
    let fps = (120. * 360.) / seconds_elapsed;
    println!("fps: {fps}");
    println!("rewards: {rew_val}");

    gym.close_renderer();
}
