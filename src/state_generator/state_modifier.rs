use crate::gamestates::game_state::GameState;

/// default trait type for the modifiers that are used in things such as the CombinedStateGenerator
pub trait StateModifier {
    fn modify_state(&mut self, state: &mut GameState);
}