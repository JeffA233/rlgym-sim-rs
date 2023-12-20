use crate::{
    action_parsers::action_parser::ActionParser,
    conditionals::terminal_condition::TerminalCondition,
    obs_builders::obs_builder::ObsBuilder,
    reward_functions::reward_fn::RewardFn,
    sim_wrapper::wrapper::RocketsimWrapper,
    state_setters::{state_setter::StateSetter, wrappers::state_wrapper::StateWrapper}, make::MakeConfig,
};

use crate::gamestates::game_state::GameState;

/// Struct that wraps the game structs (basically) and provides an interface to the observation builders, state setters, etc.
pub struct GameMatch {
    pub game_config: GameConfig,
    pub _reward_fn: Box<dyn RewardFn>,
    pub _terminal_condition: Box<dyn TerminalCondition>,
    pub _obs_builder: Vec<Box<dyn ObsBuilder>>,
    pub _action_parser: Box<dyn ActionParser>,
    pub _state_setter: Box<dyn StateSetter>,
    pub agents: usize,
    pub observation_space: Vec<usize>,
    pub action_space: Vec<usize>,
    pub _prev_actions: Vec<Vec<f32>>,
    pub _spectator_ids: Vec<i32>,
    // pub last_touch: i32,
    pub _initial_score: i32,
    pub sim_wrapper: RocketsimWrapper,
}

/// Config struct that takes mutators, team size, tick skip, and spawn opponents.
/// Should be used in the `make` function.
/// 
/// # Default
/// ```rust,ignore
/// fn default() -> Self {
///     Self {
///         gravity: 1., 
///         boost_consumption: 1., 
///         team_size: 1, 
///         tick_skip: 8, 
///         spawn_opponents: true, 
///     }
/// }
/// ```
/// 
#[derive(Clone, Copy, Debug)]
pub struct GameConfig {
    pub gravity: f32,
    pub boost_consumption: f32,
    pub team_size: usize,
    pub tick_skip: usize,
    pub spawn_opponents: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            gravity: 1., 
            boost_consumption: 1., 
            team_size: 1, 
            tick_skip: 8, 
            spawn_opponents: true, 
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub goals: u16,
    pub own_goals: u16,
    pub assists: u16,
    // pub saves: u16,
    // pub shots: u16,
    // pub demolitions: u16,
}

impl GameMatch {
    pub fn new(
        config: MakeConfig,
        // reward_function: Box<dyn RewardFn>,
        // terminal_condition: Box<dyn TerminalCondition>,
        // obs_builder: Vec<Box<dyn ObsBuilder>>,
        // action_parser: Box<dyn ActionParser>,
        // state_setter: Box<dyn StateSetter>,
        // team_size: Option<usize>,
        // tick_skip: Option<usize>,
        // gravity: Option<f32>,
        // boost_consumption: Option<f32>,
        // spawn_opponents: Option<bool>,
    ) -> Self {
        // let team_size = config.game_config.team_size.unwrap_or(1);
        // let tick_skip = tick_skip.unwrap_or(8);
        // let gravity = gravity.unwrap_or(1.);
        // let boost_consumption = boost_consumption.unwrap_or(1.);
        // let spawn_opponents = spawn_opponents.unwrap_or(true);
        let num_agents = if config.game_config.spawn_opponents { config.game_config.team_size * 2 } else { config.game_config.team_size };

        // rocketsim start
        let sim_wrapper = RocketsimWrapper::new(config.game_config);

        GameMatch {
            game_config: config.game_config,
            _reward_fn: config.reward_fn,
            _terminal_condition: config.terminal_condition,
            _obs_builder: config.obs_builder,
            _action_parser: config.action_parser,
            _state_setter: config.state_setter,
            agents: num_agents,
            observation_space: Vec::<usize>::new(),
            action_space: Vec::<usize>::new(),
            _prev_actions: vec![vec![0.; 8]; num_agents],
            _spectator_ids: vec![0; 6],
            // last_touch: -1,
            _initial_score: 0,
            sim_wrapper,
        }
    }

    pub fn episode_reset(&mut self, initial_state: &GameState) {
        self._spectator_ids = initial_state.players.iter().map(|x| x.car_id).collect();
        // self._prev_actions.fill(vec![0.; 8]);
        self._prev_actions = vec![vec![0.; 8]; self.agents];
        self._terminal_condition.reset(initial_state);
        self._reward_fn.reset(initial_state);
        self._obs_builder.iter_mut().map(|func| func.reset(initial_state)).for_each(drop);
        // self._obs_builder.reset(&initial_state);
        // self.last_touch = -1;
        self._initial_score = initial_state.blue_score - initial_state.orange_score;
    }

    pub fn build_observations(&mut self, state: &GameState) -> Vec<Vec<f32>> {
        let obs_build_len = self._obs_builder.len();
        let player_len = state.players.len();
        assert!(obs_build_len >= player_len, "not enough observation builders (len: {obs_build_len}) were provided for the amount of players (len: {player_len})");
        // if state.last_touch == -1 {
        //     state.last_touch = self.last_touch.clone();
        // } else {
        //     self.last_touch = state.last_touch.clone();
        // }

        // let config_arr = self.get_config();

        self._obs_builder.iter_mut().map(|func| func.pre_step(state, &self.game_config)).for_each(drop);
        // self._obs_builder.pre_step(&state);

        // for (i, player) in state.players.iter().enumerate() {
        //     observations.push(self._obs_builder.build_obs(player, &state, &self._prev_actions[i]));
        // }
        // self._obs_builder
        //     .iter_mut()
        //     .zip(&state.players)
        //     .zip(&self._prev_actions)
        //     .map(|((func, player), prev_acts)| func.build_obs(player, state, &self.game_config, prev_acts))
        //     .collect()

        state.players
            .iter()
            .zip(&mut self._obs_builder)
            .map(|(player, func)| func.build_obs(player, state, &self.game_config))
            .collect()

        // if observations.len() == 1 {
        //     return observations
        // } else {
        //     return observations
        // }
    }

    pub fn get_rewards(&mut self, state: &GameState, done: bool) -> Vec<f32> {
        let mut rewards = Vec::<f32>::with_capacity(self.agents);

        self._reward_fn.pre_step(state);

        for player in state.players.iter() {
            if done {
                rewards.push(self._reward_fn.get_final_reward(player, state));
            } else {
                rewards.push(self._reward_fn.get_reward(player, state));
            }
        }

        // if rewards.len() == 1 {
        //     return vec![rewards[0]]
        // } else {
        //     return rewards
        // }
        rewards
    }

    pub fn is_done(&mut self, state: &GameState) -> bool {
        self._terminal_condition.is_terminal(state)
    }

    pub fn get_result(&self, state: &GameState) -> i32 {
        let current_score = state.blue_score - state.orange_score;
        current_score - self._initial_score
    }

    pub fn get_state(&mut self) -> GameState {
        self.sim_wrapper.get_rlgym_gamestate(false).0
    }

    pub fn parse_actions(&mut self, actions: Vec<Vec<f32>>, state: &GameState) -> Vec<Vec<f32>> {
        let parsed_actions = self._action_parser.parse_actions(actions, state);
        let acts_len = parsed_actions.len();
        let players_len = state.players.len();
        assert!(acts_len == players_len, "parsed actions was not the same length (len: {acts_len}) as player count (len: {players_len})");
        self._prev_actions = parsed_actions.to_vec();
        parsed_actions
    }

    // pub fn format_actions(&mut self, actions: Vec<Vec<f64>>) -> Vec<f64> {
    //     let mut acts = Vec::<f64>::new();

    //     self._prev_actions = actions.clone();

    //     for (spectator_id, mut action) in self._spectator_ids.iter().zip(actions) {
    //         acts.push(*spectator_id as f64);
    //         acts.append(&mut action);
    //     }

    //     return acts
    // }

    pub fn get_reset_state(&mut self, state: &GameState) -> StateWrapper {
        let mut new_state = self._state_setter.build_wrapper(self.game_config.team_size, self.game_config.spawn_opponents, Some(state));
        self._state_setter.reset(&mut new_state);
        new_state
    }

    pub fn set_seeds(&mut self, seed: u64) {
        self._state_setter.set_seed(seed);
    }

    pub fn get_config(&self) -> GameConfig {
        // let spawn_opponents_bool = if self.game_config.spawn_opponents { 1 } else { 0 };
        // [
        //     self.game_config.team_size as f32,
        //     spawn_opponents_bool as f32,
        //     self.game_config.tick_skip as f32,
        //     self.game_config.gravity,
        //     self.game_config.boost_consumption,
        // ]
        self.game_config
    }

    pub fn update_settings(&mut self, new_config: GameConfig, new_obs_builder: Option<Vec<Box<dyn ObsBuilder>>>) -> GameState {
        // TODO: do extra modes and more mutators
        self.game_config = new_config;
        let car_count = if new_config.spawn_opponents {
            new_config.team_size * 2
        } else {
            new_config.team_size
        };
        self.agents = car_count;
        // self._obs_builder = new_obs_builder.unwrap_or(self._obs_builder);
        if let Some(val) = new_obs_builder { self._obs_builder = val }
        self.sim_wrapper.set_game_config(new_config, false).0
    }

    fn _auto_detech_obs_space(&mut self) {
        self.observation_space = self._obs_builder[0].get_obs_space();
    }
}

// pub fn async_build_observations(mut _obs_builder: &mut (dyn ObsBuilder), state: &GameState, agents: usize, _prev_actions: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
//     let mut observations = Vec::<Vec<f64>>::with_capacity(agents);

//     // if state.last_touch == -1 {
//     //     state.last_touch = self.last_touch.clone();
//     // } else {
//     //     self.last_touch = state.last_touch.clone();
//     // }

//     _obs_builder.pre_step(&state);

//     for (i, player) in state.players.iter().enumerate() {
//         observations.push(_obs_builder.build_obs(player, &state, &_prev_actions[i]));
//     }

//     // if observations.len() == 1 {
//     //     return observations
//     // } else {
//     //     return observations
//     // }
//     return observations
// }

// pub fn async_get_rewards(mut _reward_fn: &mut (dyn RewardFn), state: &GameState, done: bool, agents: usize, _prev_actions: &Vec<Vec<f64>>) -> Vec<f64> {
//     let mut rewards = Vec::<f64>::with_capacity(agents);

//     _reward_fn.pre_step(&state);

//     for (i, player) in state.players.iter().enumerate() {
//         if done {
//             rewards.push(_reward_fn.get_final_reward(player, &state, &_prev_actions[i]));
//         } else {
//             rewards.push(_reward_fn.get_reward(player, &state, &_prev_actions[i]));
//         }
//     }

//     // if rewards.len() == 1 {
//     //     return vec![rewards[0]]
//     // } else {
//     //     return rewards
//     // }
//     return rewards
// }
