//! All "magic numbers" for the game live here. Centralizing them makes the
//! game easy to tune and extend later without hunting through gameplay code.

// ---- Window ----
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

// ---- Paddles ----
pub const PADDLE_WIDTH: f32 = 15.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
/// Distance from the screen edge to the paddle.
pub const PADDLE_MARGIN: f32 = 30.0;

/// Speed (pixels/second) the human player's paddle moves at.
pub const PLAYER_SPEED: f32 = 420.0;

// ---- AI difficulty ----
// These are the AI paddle's maximum movement speeds. Higher speed means the
// AI paddle can keep up with the ball more precisely, which is what makes
// the "Hard" setting feel tougher, without needing anything smarter than
// simple tracking logic.
pub const AI_SPEED_EASY: f32 = 160.0;
pub const AI_SPEED_MEDIUM: f32 = 260.0;
pub const AI_SPEED_HARD: f32 = 420.0;

// ---- Ball ----
pub const BALL_SIZE: f32 = 15.0;
pub const BALL_START_SPEED: f32 = 320.0;
/// How much speed (pixels/second) is added to the ball after each paddle hit.
pub const BALL_SPEED_INCREMENT: f32 = 25.0;
/// Upper bound so the ball never becomes physically unplayable.
pub const BALL_MAX_SPEED: f32 = 720.0;

// ---- Scoring ----
pub const WINNING_SCORE: i32 = 10;
/// Pause (in seconds) after a point is scored before the ball launches again.
pub const SERVE_DELAY: f32 = 1.0;

// ---- Visual style ----
pub const FONT_SIZE_TITLE: f32 = 60.0;
pub const FONT_SIZE_SCORE: f32 = 40.0;
pub const FONT_SIZE_TEXT: f32 = 24.0;
pub const FONT_SIZE_SMALL: f32 = 18.0;
