//! The `Paddle` struct represents both the human-controlled paddle and the
//! computer-controlled paddle. They share the same shape/collision logic;
//! only the code that *decides* how they move differs (see `main.rs` for the
//! player and `ai.rs` for the computer).

use crate::constants::*;
use macroquad::prelude::*;

pub struct Paddle {
    /// Axis-aligned bounding box used for both drawing and collision.
    pub rect: Rect,
    /// Pixels/second this paddle is allowed to move at.
    pub speed: f32,
}

impl Paddle {
    /// Creates a paddle vertically centered on screen at the given x position.
    pub fn new(x: f32, speed: f32) -> Self {
        Self {
            rect: Rect::new(
                x,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            speed,
        }
    }

    /// Moves the paddle vertically. `dir` should be -1.0 (up), 0.0 (still),
    /// or 1.0 (down); it gets scaled by speed and delta-time so movement is
    /// frame-rate independent.
    pub fn move_vertical(&mut self, dir: f32, dt: f32) {
        self.rect.y += dir * self.speed * dt;
        self.clamp_to_screen();
    }

    /// Keeps the paddle fully within the vertical bounds of the screen.
    pub fn clamp_to_screen(&mut self) {
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        }
        if self.rect.y + self.rect.h > SCREEN_HEIGHT {
            self.rect.y = SCREEN_HEIGHT - self.rect.h;
        }
    }

    /// Resets the paddle back to vertical center (used between rounds/games).
    pub fn reset_position(&mut self) {
        self.rect.y = SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;
    }

    pub fn center_y(&self) -> f32 {
        self.rect.y + self.rect.h / 2.0
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
