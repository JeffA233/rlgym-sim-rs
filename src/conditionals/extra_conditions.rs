use crate::gamestates::game_state::GameState;

use super::terminal_condition::TerminalCondition;

pub struct CombinedTerminalConditions {
    conditionals: Vec<Box<dyn TerminalCondition>>,
}

impl CombinedTerminalConditions {
    pub fn new(conditionals: Vec<Box<dyn TerminalCondition>>) -> Self {
        Self {
            conditionals,
        }
    }
}

impl TerminalCondition for CombinedTerminalConditions {
    fn reset(&mut self, initial_state: &GameState) {
        for conditional in self.conditionals.iter_mut() {
            conditional.reset(initial_state);
        }
    }

    fn is_terminal(&mut self, current_state: &GameState) -> bool {
        self.conditionals.iter_mut().map(|f| f.is_terminal(current_state))
        .any(|x| x)
    }
}

/// Returns a terminal signal when the ball is at x: 0, y: 0 after the specified steps
pub struct NoTouchKickoffTimeoutCondition {
    steps: i64,
    max_steps: i64,
}

impl NoTouchKickoffTimeoutCondition {
    pub fn new(max_steps: i64) -> Self {
        NoTouchKickoffTimeoutCondition { steps: 0, max_steps }
    }
}

impl TerminalCondition for NoTouchKickoffTimeoutCondition {
    fn reset(&mut self, _initial_state: &GameState) {
        self.steps = 0
    }

    fn is_terminal(&mut self, current_state: &GameState) -> bool {
        if current_state.ball.position.x == 0. && current_state.ball.position.y == 0. {
            if current_state.players.iter().any(|x| x.ball_touched) {
                self.steps = 0;
                false
            } else {
                self.steps += 1;
                self.steps >= self.max_steps
            }
        } else {
            false
        }
    }
}
