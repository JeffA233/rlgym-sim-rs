use rlgym_sim_rs::action_parsers::test_parser::TestAction;
use rlgym_sim_rs::conditionals::common_conditions::{TimeoutCondition, GoalScoredCondition};
use rlgym_sim_rs::conditionals::terminal_condition::TerminalCondition;
use rlgym_sim_rs::envs::game_match::GameConfig;
use rlgym_sim_rs::gamestates::game_state::GameState;
use rlgym_sim_rs::make;
use rlgym_sim_rs::obs_builders::advanced_obs::AdvancedObs;
use rlgym_sim_rs::reward_functions::common_rewards::misc_rewards::EventReward;
// use rlgym_sim_rs::state_setters::default_state::{
//     AgentBallHitStateTester, 
//     BlueGoalStateTester, 
//     OrangeGoalStateTester, 
//     DemoStateTester
// };

use rlgym_sim_rs::{
    obs_builders::obs_builder::ObsBuilder,
    state_setters::default_state::DefaultStateTester,
};
use rocketsim_rs::sim::CarConfig;

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
    // let obs = vec![vec![93, 93, 93], vec![92, 93, 93], vec![91, 93, 93]];
    // let mut vec = Vec::<Vec<i32>>::new();
    // vec.extend(obs);
    let term_cond = Box::new(CombinedTerminalConditions::new(1));
    let reward_fn = Box::new(EventReward::new(None, None, None, None, None, None, None, None));
    let act_parse = Box::new(TestAction::new());
    let state_set = Box::new(DefaultStateTester::new());
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]];
    rocketsim_rs::init(None);
    let tick_skip = 1;
    let config = GameConfig {
        tick_skip,
        spawn_opponents: false,
        team_size: 1,
        gravity: 1.,
        boost_consumption: 1.,
        car_config: CarConfig::octane(),
    };
    // let obs_build: Box<dyn ObsBuilder> = Box::new(AdvancedObs::new());
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..1 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }

    let game_config = make::MakeConfig {
        game_config: config,
        terminal_condition: term_cond,
        reward_fn,
        obs_builder: obs_build_vec,
        use_single_obs: true,
        action_parser: act_parse,
        state_setter: state_set, 
    };
    let mut gym = make::make(game_config, None);

    // -- start testing self-play=false --

    gym.reset(None, None);
    
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 1, "obs was not of correct length for 1v0, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 1v0");

    let new_config = GameConfig { gravity: 1., boost_consumption: 1., team_size: 2, tick_skip: 1, spawn_opponents: false, car_config: CarConfig::octane(), };
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..2 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    gym.update_config(new_config, Some(obs_build_vec));

    gym.reset(None, None);
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]; 2];
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 2, "obs was not of correct length for 2v0, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 2v0");

    let new_config = GameConfig { gravity: 1., boost_consumption: 1., team_size: 3, tick_skip: 1, spawn_opponents: false, car_config: CarConfig::octane(), };
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..3 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    gym.update_config(new_config, Some(obs_build_vec));
    gym.reset(None, None);
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]; 3];
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 3, "obs was not of correct length for 3v0, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 3v0");

    // -- start of self-play=true --

    let new_config = GameConfig { gravity: 1., boost_consumption: 1., team_size: 1, tick_skip: 1, spawn_opponents: true, car_config: CarConfig::octane(), };
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..2 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    gym.update_config(new_config, Some(obs_build_vec));

    gym.reset(None, None);
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]; 2];
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 2, "obs was not of correct length for 1v1, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 1v1");

    let new_config = GameConfig { gravity: 1., boost_consumption: 1., team_size: 2, tick_skip: 1, spawn_opponents: true, car_config: CarConfig::octane(), };
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..4 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    gym.update_config(new_config, Some(obs_build_vec));

    gym.reset(None, None);
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]; 4];
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 4, "obs was not of correct length for 2v2, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 2v2");

    let new_config = GameConfig { gravity: 1., boost_consumption: 1., team_size: 3, tick_skip: 1, spawn_opponents: true, car_config: CarConfig::octane(), };
    let mut obs_build_vec: Vec<Box<dyn ObsBuilder>> = Vec::new();
    for _ in 0..6 {
        obs_build_vec.push(Box::new(AdvancedObs::new()));
    }
    gym.update_config(new_config, Some(obs_build_vec));
    gym.reset(None, None);
    let actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]; 6];
    let (obs, _, _, _) = gym.step(actions);
    let length = obs.len();
    assert!(length == 6, "obs was not of correct length for 3v3, was: {length}");
    assert!(gym._prev_state.players.iter().all(|player| (player.car_id <= 6 && player.car_id >= 0)), "car ids in state did not work correctly in 3v3");
}