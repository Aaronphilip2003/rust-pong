//! Rules-based computer opponent. No machine learning, neural networks, or
//! external AI libraries are used here on purpose -- the "AI" is simply a
//! paddle that tracks the ball's vertical position at a capped speed. The
//! difficulty settings only change how fast the paddle is allowed to move,
//! which is enough to create a real difficulty curve.

use crate::constants::*;
use crate::paddle::Paddle;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Maximum vertical speed (pixels/second) granted to the AI paddle.
    pub fn speed(&self) -> f32 {
        match self {
            Difficulty::Easy => AI_SPEED_EASY,
            Difficulty::Medium => AI_SPEED_MEDIUM,
            Difficulty::Hard => AI_SPEED_HARD,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "EASY",
            Difficulty::Medium => "MEDIUM",
            Difficulty::Hard => "HARD",
        }
    }

    /// Cycles to the next difficulty. Not currently wired to an input, but
    /// kept as a small, ready-made hook for extending the menu (e.g. a
    /// "TAB to cycle difficulty" control) without touching game.rs logic.
    #[allow(dead_code)]
    pub fn next(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

/// Moves the AI-controlled paddle toward the ball's vertical center.
/// The paddle never teleports -- it is only ever nudged at its configured
/// max speed, clamped to the screen, each frame.
pub fn update_ai(paddle: &mut Paddle, ball_center_y: f32, difficulty: Difficulty, dt: f32) {
    // Small dead zone prevents the paddle from jittering back and forth
    // when it's already lined up with the ball.
    const DEAD_ZONE: f32 = 6.0;

    let diff = ball_center_y - paddle.center_y();
    if diff.abs() <= DEAD_ZONE {
        return;
    }

    let dir = if diff > 0.0 { 1.0 } else { -1.0 };
    // The paddle's own `speed` field is what actually gets used here, and it
    // is set from `difficulty.speed()` whenever the AI paddle is (re)created
    // -- see `Game::new` and `Game::apply_difficulty` in game.rs. Passing
    // `difficulty` into this function keeps the dependency explicit even
    // though the value is read indirectly through `paddle.speed`.
    let _ = difficulty; // difficulty is expressed via paddle.speed; kept as a parameter for clarity/future use
    paddle.move_vertical(dir, dt);
}
