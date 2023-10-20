# rlgym-sim-rs
Initial release of Rust RLGym for sim. More documentation soon.

# Differences between RLGym and rlgym-sim-rs
Observation functions must be provided as a Vec (`Vec<Box<dyn ObsBuilder>>`) where the lengh must be equal to the number of agents that will be in the match. The current functionality is that observation builders are per-agent and hence independent of each other.

Terminal conditions must return only a singular boolean when called.

Info does not return the state for now. Unfortunately mixed-type HashMaps are not possible by default with PyO3 though this may be adapted in the future.

# Example Python bindings
See `examples/lib.rs` for an example of how to create Python bindings that you can use.
