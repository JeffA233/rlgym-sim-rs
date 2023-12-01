//! This crate allows the ability to use Rust's safety and performance to run an RLGym-like instance of RocketSim.
//! 
//! It should closely follow what the [RLGym docs] describe to correctly configure the gym however there are some key differences.
//! 
//! The first difference is that rlgym-sim-rs allows one observation builder per agent. 
//! This means that if you have setup a 3v3 match or switch to a 3v3 match at any point, there must be 6 observation builders provided.
//! You can either do this via the [`make::make`] function initially or you can use the `.update_config()` of `Gym`.
//! 
//! The second difference is that terminal conditions are evaluated from one function instead of allowing for multiple to be used.
//! You can use [conditionals::extra_conditions::CombinedTerminalConditions] in order to use multiple terminal conditions at once.
//! 
//! ## Example of usage
//! See [`make::make`] as a place to start.
//! The file `lib.rs` in examples also has an example scenario for Python bindings with PyO3 if necessary.
//! Also, see `basic_example.rs` in examples.
//! The tests are generally also a good place to look at.
//! 
//! ## Status of development
//! - The crate should currently be considered as not stable although (as far as is known) all basic features are available for use from RLGym.
//!   + Breaking changes may occur though there will be an attempt to keep them as low as possible.
//! - In the future, [RLViser] may be used as an optional visualizer for the gym with enough effort.
//! - More documentation should be written soon and more data from RocketSim will likely be available for use in the gym.
//! 
//! [RLGym docs]: https://rlgym.org/
//! [RLViser]: https://github.com/VirxEC/rlviser/

pub mod action_parsers;
pub mod common_values;
pub mod conditionals;
pub mod envs;
pub mod gamestates;
pub mod gym;
pub mod make;
pub mod math;
pub mod obs_builders;
pub mod reward_functions;
pub mod sim_wrapper;
pub mod state_setters;
pub mod state_generator;
pub mod render;

pub use gym::Gym;
pub use make::{
    MakeConfig, 
    make,
};
pub use action_parsers::{
    action_parser::ActionParser, 
    // default_act::default_action, 
    discrete_act::DiscreteAction, 
    continous_act::ContinuousAction,
};
pub use state_setters::{
    default_state::DefaultState, 
    state_setter::StateSetter, 
    random_state::RandomState, 
    weighted_state_setter::WeightedSampleSetter,
};
pub use obs_builders::{
    advanced_obs::AdvancedObs, 
    obs_builder::ObsBuilder,
};
pub use conditionals::{
    common_conditions::{GoalScoredCondition, TimeoutCondition, NoTouchTimeoutCondition}, 
    extra_conditions::{NoTouchKickoffTimeoutCondition, CombinedTerminalConditions}, 
    terminal_condition::TerminalCondition,
};
pub use math::{
    clip, 
    cosine_similarity, 
    get_dist, 
    vec_div_variable, 
    element_mult_vec, 
    element_div_vec, 
    element_add_vec, 
    element_sub_vec,
    norm_func,
    scalar_projection,
    vector_projection,
    rand_uvec3,
    rand_vec3,
    trace,
    unitvec,
    squared_vecmag,
};
pub use reward_functions::{
    combined_reward::CombinedReward, 
    reward_fn::RewardFn, 
    common_rewards::{ball_goal_rewards, player_ball_rewards, misc_rewards, conditional_rewards},
};
pub use state_generator::{
    combined_gen::CombinedStateGenerator, 
    common_state_mods, 
    state_modifier::StateModifier,
};

pub use state_setters::wrappers::car_wrapper::IntoArray;