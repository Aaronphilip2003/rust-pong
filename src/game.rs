//! Core game state machine and orchestration. `Game` owns the paddles, ball,
//! scores and current `GameState`, and is responsible for tying together
//! input, physics (via `Ball`/`Paddle`), the AI (via `ai::update_ai`) and
//! rendering. Keeping this logic in one place (separate from `main.rs`)
//! keeps `main.rs` a thin entry point.

use crate::ai::{self, Difficulty};
use crate::ball::Ball;
use crate::constants::*;
use crate::paddle::Paddle;
use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub player: Paddle,
    pub computer: Paddle,
    pub ball: Ball,
    pub player_score: i32,
    pub computer_score: i32,
    pub difficulty: Difficulty,
    pub paused: bool,

    /// Counts down after a point is scored; the ball sits still and
    /// invisible-to-input until this reaches zero, then serves again.
    serve_timer: f32,
    waiting_to_serve: bool,
}

impl Game {
    pub fn new() -> Self {
        let difficulty = Difficulty::Medium;
        Self {
            state: GameState::Menu,
            player: Paddle::new(PADDLE_MARGIN, PLAYER_SPEED),
            computer: Paddle::new(
                SCREEN_WIDTH - PADDLE_MARGIN - PADDLE_WIDTH,
                difficulty.speed(),
            ),
            ball: Ball::new(),
            player_score: 0,
            computer_score: 0,
            difficulty,
            paused: false,
            serve_timer: 0.0,
            waiting_to_serve: false,
        }
    }

    /// Applies a (possibly new) difficulty, updating the AI paddle's speed.
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.computer.speed = difficulty.speed();
    }

    /// Resets scores/positions for a brand-new game (used from Menu and
    /// after Game Over -> restart).
    fn start_new_game(&mut self) {
        self.player_score = 0;
        self.computer_score = 0;
        self.player.reset_position();
        self.computer.reset_position();
        self.ball.reset();
        self.paused = false;
        self.waiting_to_serve = false;
        self.state = GameState::Playing;
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Menu => self.update_menu(),
            GameState::Playing => self.update_playing(dt),
            GameState::GameOver => self.update_game_over(),
        }
    }

    fn update_menu(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.set_difficulty(Difficulty::Easy);
        } else if is_key_pressed(KeyCode::Key2) {
            self.set_difficulty(Difficulty::Medium);
        } else if is_key_pressed(KeyCode::Key3) {
            self.set_difficulty(Difficulty::Hard);
        }

        if is_key_pressed(KeyCode::Space) {
            self.start_new_game();
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::R) {
            self.start_new_game();
        }
    }

    fn update_playing(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::P) {
            self.paused = !self.paused;
        }
        if self.paused {
            return;
        }

        // --- Player input ---
        let mut dir = 0.0;
        if is_key_down(KeyCode::W) {
            dir -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            dir += 1.0;
        }
        self.player.move_vertical(dir, dt);

        // --- Serve delay handling ---
        if self.waiting_to_serve {
            self.serve_timer -= dt;
            if self.serve_timer <= 0.0 {
                self.waiting_to_serve = false;
                self.ball.reset();
            }
            // While waiting, the AI still tracks the ball's resting position
            // but nothing else advances.
            ai::update_ai(&mut self.computer, self.ball.rect.y + self.ball.rect.h / 2.0, self.difficulty, dt);
            return;
        }

        // --- AI + Ball update ---
        ai::update_ai(&mut self.computer, self.ball.rect.y + self.ball.rect.h / 2.0, self.difficulty, dt);
        self.ball.update(dt);
        self.handle_paddle_collisions();
        self.handle_scoring();
    }

    /// Proper AABB collision detection against both paddles, with position
    /// correction to prevent clipping and a velocity-direction check to
    /// prevent a single collision from being triggered multiple times in a
    /// row while the ball rectangle still overlaps the paddle.
    fn handle_paddle_collisions(&mut self) {
        let ball_rect = self.ball.rect;

        // Left paddle (player) -- only bounce if ball is moving left/toward it.
        if self.ball.vel.x < 0.0 && ball_rect.overlaps(&self.player.rect) {
            self.ball.rect.x = self.player.rect.x + self.player.rect.w;
            self.ball.vel.x = -self.ball.vel.x;
            self.deflect_by_hit_position(self.player.center_y());
            self.ball.increase_speed();
        }
        // Right paddle (computer) -- only bounce if ball is moving right/toward it.
        else if self.ball.vel.x > 0.0 && ball_rect.overlaps(&self.computer.rect) {
            self.ball.rect.x = self.computer.rect.x - self.ball.rect.w;
            self.ball.vel.x = -self.ball.vel.x;
            self.deflect_by_hit_position(self.computer.center_y());
            self.ball.increase_speed();
        }
    }

    /// Adds a bit of vertical "spin" to the ball based on where it struck
    /// the paddle relative to the paddle's center, which makes returns feel
    /// more responsive/skill-based rather than perfectly mirrored bounces.
    fn deflect_by_hit_position(&mut self, paddle_center_y: f32) {
        let ball_center_y = self.ball.rect.y + self.ball.rect.h / 2.0;
        let offset = (ball_center_y - paddle_center_y) / (PADDLE_HEIGHT / 2.0);
        let offset = offset.clamp(-1.0, 1.0);
        self.ball.vel.y += offset * self.ball.speed * 0.5;
    }

    fn handle_scoring(&mut self) {
        if self.ball.rect.x + self.ball.rect.w < 0.0 {
            // Ball exited the left side -> computer scores.
            self.computer_score += 1;
            self.begin_serve_delay();
        } else if self.ball.rect.x > SCREEN_WIDTH {
            // Ball exited the right side -> player scores.
            self.player_score += 1;
            self.begin_serve_delay();
        }

        if self.player_score >= WINNING_SCORE || self.computer_score >= WINNING_SCORE {
            self.state = GameState::GameOver;
        }
    }

    fn begin_serve_delay(&mut self) {
        // Park the ball off-screen center-ish and stop it, then wait before
        // launching the next serve. Position/velocity are fully reset once
        // the timer elapses in `update_playing`.
        self.ball.vel = vec2(0.0, 0.0);
        self.ball.rect.x = SCREEN_WIDTH / 2.0 - self.ball.rect.w / 2.0;
        self.ball.rect.y = SCREEN_HEIGHT / 2.0 - self.ball.rect.h / 2.0;
        self.waiting_to_serve = true;
        self.serve_timer = SERVE_DELAY;
    }

    // ------------------------------------------------------------------
    // Drawing
    // ------------------------------------------------------------------

    pub fn draw(&self) {
        clear_background(BLACK);

        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Playing => self.draw_playing(),
            GameState::GameOver => self.draw_game_over(),
        }
    }

    fn draw_center_line(&self) {
        let mut y = 0.0;
        while y < SCREEN_HEIGHT {
            draw_rectangle(SCREEN_WIDTH / 2.0 - 2.0, y, 4.0, 15.0, WHITE);
            y += 30.0;
        }
    }

    fn draw_menu(&self) {
        let title = "RUST PONG";
        let title_size = measure_text(title, None, FONT_SIZE_TITLE as u16, 1.0);
        draw_text(
            title,
            SCREEN_WIDTH / 2.0 - title_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - 100.0,
            FONT_SIZE_TITLE,
            WHITE,
        );

        let prompt = "Press SPACE to Start";
        let prompt_size = measure_text(prompt, None, FONT_SIZE_TEXT as u16, 1.0);
        draw_text(
            prompt,
            SCREEN_WIDTH / 2.0 - prompt_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - 20.0,
            FONT_SIZE_TEXT,
            WHITE,
        );

        let options = "1 = Easy   2 = Medium   3 = Hard";
        let options_size = measure_text(options, None, FONT_SIZE_SMALL as u16, 1.0);
        draw_text(
            options,
            SCREEN_WIDTH / 2.0 - options_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 + 30.0,
            FONT_SIZE_SMALL,
            WHITE,
        );

        let selected = format!("Difficulty: {}", self.difficulty.label());
        let selected_size = measure_text(&selected, None, FONT_SIZE_SMALL as u16, 1.0);
        draw_text(
            &selected,
            SCREEN_WIDTH / 2.0 - selected_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 + 60.0,
            FONT_SIZE_SMALL,
            WHITE,
        );
    }

    fn draw_playing(&self) {
        self.draw_center_line();
        self.player.draw();
        self.computer.draw();
        self.ball.draw();

        // Scores, top center.
        let player_score_text = self.player_score.to_string();
        draw_text(
            &player_score_text,
            SCREEN_WIDTH / 2.0 - 60.0,
            50.0,
            FONT_SIZE_SCORE,
            WHITE,
        );
        let computer_score_text = self.computer_score.to_string();
        draw_text(
            &computer_score_text,
            SCREEN_WIDTH / 2.0 + 40.0,
            50.0,
            FONT_SIZE_SCORE,
            WHITE,
        );

        // Current difficulty, small text in a corner.
        let diff_text = format!("Difficulty: {}", self.difficulty.label());
        draw_text(&diff_text, 10.0, SCREEN_HEIGHT - 15.0, FONT_SIZE_SMALL, WHITE);

        if self.paused {
            let text = "PAUSED";
            let size = measure_text(text, None, FONT_SIZE_TITLE as u16, 1.0);
            draw_text(
                text,
                SCREEN_WIDTH / 2.0 - size.width / 2.0,
                SCREEN_HEIGHT / 2.0,
                FONT_SIZE_TITLE,
                WHITE,
            );
        }

        // FPS counter, top-right corner (stretch goal).
        let fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, SCREEN_WIDTH - 100.0, 20.0, FONT_SIZE_SMALL, WHITE);
    }

    fn draw_game_over(&self) {
        let result_text = if self.player_score > self.computer_score {
            "YOU WIN"
        } else {
            "COMPUTER WINS"
        };
        let result_size = measure_text(result_text, None, FONT_SIZE_TITLE as u16, 1.0);
        draw_text(
            result_text,
            SCREEN_WIDTH / 2.0 - result_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 - 40.0,
            FONT_SIZE_TITLE,
            WHITE,
        );

        let final_score = format!("{} - {}", self.player_score, self.computer_score);
        let final_score_size = measure_text(&final_score, None, FONT_SIZE_TEXT as u16, 1.0);
        draw_text(
            &final_score,
            SCREEN_WIDTH / 2.0 - final_score_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 + 10.0,
            FONT_SIZE_TEXT,
            WHITE,
        );

        let prompt = "Press R to Restart";
        let prompt_size = measure_text(prompt, None, FONT_SIZE_TEXT as u16, 1.0);
        draw_text(
            prompt,
            SCREEN_WIDTH / 2.0 - prompt_size.width / 2.0,
            SCREEN_HEIGHT / 2.0 + 60.0,
            FONT_SIZE_TEXT,
            WHITE,
        );
    }
}
