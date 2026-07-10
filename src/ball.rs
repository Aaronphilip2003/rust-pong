//! The `Ball` owns its own position, velocity and current speed. Paddle
//! collision *response* (bouncing off a paddle) is handled in `game.rs`
//! since it needs to know about both paddles; this module handles the ball's
//! own motion and its bounce off the top/bottom walls.

use crate::constants::*;
use macroquad::prelude::*;

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
    pub speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        let mut ball = Self {
            rect: Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE),
            vel: vec2(0.0, 0.0),
            speed: BALL_START_SPEED,
        };
        ball.reset();
        ball
    }

    /// Re-centers the ball and launches it in a random-ish diagonal
    /// direction (random left/right, random shallow vertical angle).
    /// Used at the start of the game and after every point scored.
    pub fn reset(&mut self) {
        self.rect.x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
        self.speed = BALL_START_SPEED;

        let dir_x: f32 = if macroquad::rand::gen_range(0, 2) == 0 {
            -1.0
        } else {
            1.0
        };
        // Shallow random vertical angle so serves aren't perfectly flat but
        // also never so steep the ball feels unfair.
        let dir_y: f32 = macroquad::rand::gen_range(-0.4_f32, 0.4_f32);

        self.vel = vec2(dir_x, dir_y).normalize() * self.speed;
    }

    /// Advances the ball and bounces it off the top/bottom walls.
    /// Left/right "wall" behavior (scoring) is handled by the caller, since
    /// that requires updating score state.
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt;
        self.rect.y += self.vel.y * dt;

        if self.rect.y <= 0.0 {
            self.rect.y = 0.0;
            self.vel.y = -self.vel.y;
        } else if self.rect.y + self.rect.h >= SCREEN_HEIGHT {
            self.rect.y = SCREEN_HEIGHT - self.rect.h;
            self.vel.y = -self.vel.y;
        }
    }

    /// Speeds the ball up slightly, preserving its current direction,
    /// capped at BALL_MAX_SPEED so it never becomes unplayable.
    pub fn increase_speed(&mut self) {
        self.speed = (self.speed + BALL_SPEED_INCREMENT).min(BALL_MAX_SPEED);
        let dir = self.vel.normalize();
        self.vel = dir * self.speed;
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
