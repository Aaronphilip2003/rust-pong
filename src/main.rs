//! Entry point. Keeps only window configuration and the top-level game
//! loop; all actual gameplay logic lives in `game.rs` (and the modules it
//! coordinates: `paddle.rs`, `ball.rs`, `ai.rs`).
//!
//! This structure -- plus Macroquad, which has no native-vs-wasm branching
//! required in user code -- is what lets `cargo run` work locally and
//! `cargo build --target wasm32-unknown-unknown --release` produce a build
//! deployable to GitHub Pages, from the exact same source.

mod ai;
mod ball;
mod constants;
mod game;
mod paddle;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Pong".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
