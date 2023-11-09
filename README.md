# rlgym-sim-rs
Initial release of Rust RLGym for sim. More documentation soon.

Originally a port of [rocket-league-gym-sim](https://github.com/AechPro/rocket-league-gym-sim/tree/main) (which is a version of [RLGym](https://www.rlgym.org)). It no longer has exactly the same functionality but the goal is moving from one to the other should not be difficult. 

## Differences between RLGym-sim and rlgym-sim-rs
Observation functions must be provided as a Vec (`Vec<Box<dyn ObsBuilder>>`) where the lengh must be equal to the number of agents that will be in the match. The current functionality is that observation builders are per-agent and hence independent of each other.

Terminal conditions must return only a singular boolean when called.

Info does not return the state for now. Unfortunately mixed-type HashMaps are not possible by default with PyO3 though this may be adapted in the future.

## Installation
Installed via cargo: `cargo add rlgym-sim-rs`

While RocketSim will automatically compile thanks to Virx's [bindings](https://github.com/VirxEC/rocketsim-rs), you must still use your own [collision meshes](https://github.com/ZealanL/RocketSim#installation).

## Usage
See `tests/tester.rs` and `tests/team_change_test.rs` as rough examples for now. You can also look at `examples/lib.rs` for an example of how to create Python bindings that you can use.
