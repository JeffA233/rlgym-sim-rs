use crate::{
    action_parsers::action_parser::ActionParser, 
    conditionals::terminal_condition::TerminalCondition, 
    envs::game_match::{GameMatch, GameConfig}, 
    gym::Gym,
    obs_builders::obs_builder::ObsBuilder, 
    reward_functions::reward_fn::RewardFn, 
    state_setters::state_setter::StateSetter,
};

/// General configuration struct for the gym which is inputted through the `make` function.
/// 
/// # Example
/// 
/// ```
/// use rlgym_sim_rs::{
///     obs_builders::advanced_obs::AdvancedObs,
///     action_parsers::test_parser::TestAction,
///     conditionals::common_conditions::GoalScoredCondition,
///     envs::game_match::GameConfig,
///     reward_functions::common_rewards::misc_rewards::EventReward,
///     state_setters::default_state::DefaultState,
///     make,
/// };
/// 
/// rocketsim_rs::init(None);
/// 
/// let config = GameConfig {
///     tick_skip: 1,
///     spawn_opponents: false,
///     team_size: 1,
///     gravity: 1.,
///     boost_consumption: 1.,
/// };
/// 
/// let game_config = make::MakeConfig {
///     game_config: config,
///     terminal_condition: Box::new(GoalScoredCondition::new()),
///     reward_fn: Box::new(EventReward::new(None, None, None, None, None, None, None, None)),
///     obs_builder: vec![Box::new(AdvancedObs::new())],
///     use_single_obs: true,
///     action_parser: Box::new(TestAction::new()),
///     state_setter: Box::new(DefaultState::new(None)), 
/// };
/// 
/// let mut gym = make::make(game_config, None);
/// ```
pub struct MakeConfig {
    pub game_config: GameConfig,
    pub terminal_condition: Box<dyn TerminalCondition>,
    pub reward_fn: Box<dyn RewardFn>,
    pub obs_builder: Vec<Box<dyn ObsBuilder>>,
    pub use_single_obs: bool,
    pub action_parser: Box<dyn ActionParser>,
    pub state_setter: Box<dyn StateSetter>, 
}

/// Render configuration struct for the `make` function. 
/// 
/// `update_rate:` allows you to limit the maximum speed of the gym to this many updates/sec for ease of watching.
/// In RLViser this only affects the amount of state updates and not anything to do with the actual fps that it
/// renders at.
#[derive(Clone, Copy, Debug)]
pub struct RenderConfig {
    pub render: bool,
    pub update_rate: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            render: false,
            update_rate: 120.,
        }
    }
}

/// General generator function for the gym.
/// 
/// Use this in order to create a gym instance.
/// 
/// # Example
/// 
/// ```
/// use rlgym_sim_rs::{
///     obs_builders::advanced_obs::AdvancedObs,
///     action_parsers::test_parser::TestAction,
///     conditionals::common_conditions::GoalScoredCondition,
///     envs::game_match::GameConfig,
///     reward_functions::common_rewards::misc_rewards::EventReward,
///     state_setters::default_state::DefaultState,
///     make,
/// };
/// 
/// rocketsim_rs::init(None);
/// 
/// let config = GameConfig {
///     tick_skip: 1,
///     spawn_opponents: false,
///     team_size: 1,
///     gravity: 1.,
///     boost_consumption: 1.,
/// };
/// 
/// let game_config = make::MakeConfig {
///     game_config: config,
///     terminal_condition: Box::new(GoalScoredCondition::new()),
///     reward_fn: Box::new(EventReward::new(None, None, None, None, None, None, None, None)),
///     obs_builder: vec![Box::new(AdvancedObs::new())],
///     use_single_obs: true,
///     action_parser: Box::new(TestAction::new()),
///     state_setter: Box::new(DefaultState::new(None)), 
/// };
/// 
/// let mut gym = make::make(game_config, None);
/// ```
pub fn make(mut config: MakeConfig, render_config: Option<RenderConfig>) -> Gym {
    // let game_speed = game_config.game_speed.unwrap_or(100.);
    let tick_skip = config.game_config.tick_skip;
    config.game_config.tick_skip = if tick_skip == 0 {
        println!("tick_skip was set to 0, regular RLGym has the same behavior as 1 here");
        1
    } else if tick_skip < 1 {
        println!("tick_skip was set to {tick_skip} which is less than 1, check your code, defaulted to tick_skip=1");
        1
    } else {
        tick_skip
    };
    // let spawn_opponents = config.spawn_opponents.unwrap_or(true);
    // let team_size = config.team_size.unwrap_or(1);
    // let gravity = config.gravity.unwrap_or(1.);
    // let boost_consumption = config.boost_consumption.unwrap_or(1.);
    let game_match = GameMatch::new(
        config,
        // Some(team_size),
        // Some(tick_skip),
        // Some(gravity),
        // Some(boost_consumption),
        // Some(spawn_opponents),
    );

    Gym::new(game_match, render_config.unwrap_or_default())
}
