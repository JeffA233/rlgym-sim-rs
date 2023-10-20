use crate::gamestates::game_state::GameState;

pub trait StateModifier {
    fn modify_state(&mut self, state: GameState) -> GameState;
}

pub struct CombinedStateGenerator {
    modifiers: Vec<Box<dyn StateModifier>>,
}