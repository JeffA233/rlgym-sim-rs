use rlgym_sim_rs::action_parsers::test_parser::TestAction;
use rlgym_sim_rs::conditionals::common_conditions::{TimeoutCondition, GoalScoredCondition};
use rlgym_sim_rs::conditionals::extra_conditions::CombinedTerminalConditions;
use rlgym_sim_rs::TerminalCondition;
use rlgym_sim_rs::envs::game_match::GameConfig;
use rlgym_sim_rs::make;
use rlgym_sim_rs::obs_builders::advanced_obs::AdvancedObs;
use rlgym_sim_rs::ObsBuilder;
use rlgym_sim_rs::reward_functions::common_rewards::misc_rewards::EventReward;
use rlgym_sim_rs::state_setters::default_state::DefaultState;
use rocketsim_rs::sim::CarConfig;


#[test]
fn main() {
    let term_conds: Vec<Box<dyn TerminalCondition>> = vec![Box::new(TimeoutCondition::new(400 * 120 as i64)), Box::new(GoalScoredCondition::new())];
    let terminal_condition = Box::new(CombinedTerminalConditions::new(term_conds));
    let reward_fn = Box::new(EventReward::new(None, None, None, None, None, None, None, None));
    let obs_build = Box::new(AdvancedObs::new());
    let obs_builder: Vec<Box<dyn ObsBuilder>> = vec![obs_build];
    let action_parser = Box::new(TestAction::new());
    let state_setter = Box::new(DefaultState::new(None));

    let mut actions = vec![vec![2., 1., 0., 1., 0., 1., 0., 1.]];

    // We must do this now as we only want to initialize RocketSim once across the entire program.
    rocketsim_rs::init(None);

    let game_config = GameConfig {
        tick_skip: 1,
        spawn_opponents: false,
        team_size: 1,
        gravity: 1.,
        boost_consumption: 1.,
        car_config: CarConfig::octane(),
    };

    let game_config = make::MakeConfig {
        game_config,
        terminal_condition,
        reward_fn,
        obs_builder,
        use_single_obs: true,
        action_parser,
        state_setter, 
    };

    // If you want to render, use this as a second argument:
    // let render_config = RenderConfig {
    //     render: true,
    //     update_rate: 120.,
    // };
    
    let mut gym = make::make(game_config, None);

    gym.reset(None, None);

    for _i in 0..(120 * 50) {
        let (obs, reward, done, info) = gym.step(actions.clone());
        if done {
            gym.reset(None, None);
        }
    }
}