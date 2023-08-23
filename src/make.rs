use crate::{
    action_parsers::action_parser::ActionParser, 
    conditionals::terminal_condition::TerminalCondition, 
    envs::game_match::GameMatch, 
    gym::Gym,
    obs_builders::obs_builder::ObsBuilder, 
    reward_functions::default_reward::RewardFn, 
    state_setters::state_setter::StateSetter,
};

pub struct MakeConfig {
    pub tick_skip: Option<usize>,
    pub spawn_opponents: Option<bool>,
    pub team_size: Option<usize>,
    pub gravity: Option<f32>,
    pub boost_consumption: Option<f32>,
    pub terminal_condition: Box<dyn TerminalCondition + Send>,
    pub reward_fn: Box<dyn RewardFn + Send>,
    pub obs_builder: Vec<Box<dyn ObsBuilder + Send>>,
    pub action_parser: Box<dyn ActionParser + Send>,
    pub state_setter: Box<dyn StateSetter + Send>, 
}

/// General generator function for the gym
pub fn make(game_config: MakeConfig) -> Gym {
    // let game_speed = game_config.game_speed.unwrap_or(100.);
    let tick_skip = match game_config.tick_skip {
        Some(tick_skip) => {
            if tick_skip == 0 {
                println!("tick_skip was set to 0, regular RLGym has the same behavior as 1 here");
                1
            } else if tick_skip < 1 {
                println!("tick_skip was set to {tick_skip} which is less than 1, defaulted to tick_skip=1");
                1
            } else {
                tick_skip
            }
        }
        None => 8,
    };
    let spawn_opponents = game_config.spawn_opponents.unwrap_or(true);
    let team_size = game_config.team_size.unwrap_or(1);
    let gravity = game_config.gravity.unwrap_or(1.);
    let boost_consumption = game_config.boost_consumption.unwrap_or(1.);
    let game_match = GameMatch::new(
        game_config.reward_fn,
        game_config.terminal_condition,
        game_config.obs_builder,
        game_config.action_parser,
        game_config.state_setter,
        Some(team_size),
        Some(tick_skip),
        Some(gravity),
        Some(boost_consumption),
        Some(spawn_opponents),
    );

    Gym::new(game_match)
}
