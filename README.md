# Rust Pong

A polished, single-player Pong clone written entirely in Rust using
[Macroquad](https://macroquad.rs/). Play against a rules-based computer
opponent with three difficulty levels, in a clean retro black-and-white
arcade style. Runs natively on desktop and compiles straight to WebAssembly
for deployment to GitHub Pages.

## Project Overview

You control the left paddle and rally against a computer-controlled paddle
on the right. The ball speeds up slightly with every paddle hit, keeping
rallies tense. First player to 10 points wins. The game has a full menu ->
playing -> game over flow, difficulty selection, and a scoreboard.

The whole project is organized into small, single-responsibility modules
(`paddle`, `ball`, `ai`, `game`, `constants`) so it's easy to read, tweak, and
extend.

## Features

* Single-player Pong with smooth, frame-rate-independent movement
* Rules-based computer opponent (no ML/neural nets -- simple, tunable
  tracking logic) with three difficulty levels: Easy, Medium, Hard
* Proper AABB collision detection with clip prevention and no
  double-triggering on a single hit
* Score tracking, win condition (first to 10), and a full
  Menu / Playing / Game Over state machine
* Retro black-and-white visual style
* Pause (`P`) and a live FPS counter
* Native desktop build *and* WebAssembly build from the same source, ready
  for GitHub Pages

## Controls

| Key       | Action                                  |
|-----------|------------------------------------------|
| `W`       | Move your paddle up                      |
| `S`       | Move your paddle down                    |
| `SPACE`   | Start the game (from the menu)           |
| `1` / `2` / `3` | Select Easy / Medium / Hard (from the menu) |
| `P`       | Pause / unpause                          |
| `R`       | Restart (from the Game Over screen)      |

## Project Structure

```text
rust-pong/
├── src/
│   ├── main.rs        # Window setup + top-level game loop
│   ├── paddle.rs       # Paddle struct: movement, clamping, drawing
│   ├── ball.rs         # Ball struct: motion, wall bounce, speed-up, reset
│   ├── ai.rs           # Difficulty enum + rules-based paddle tracking
│   ├── game.rs         # GameState machine, collisions, scoring, rendering
│   └── constants.rs    # All tunable values (sizes, speeds, scores, etc.)
├── assets/             # Reserved for future art/audio (currently empty)
├── Cargo.toml
├── README.md
└── .gitignore
```

## Run Locally

Requires a recent stable Rust toolchain (1.80+ recommended, since Macroquad
relies on `size_of` being available via the standard prelude).

```bash
cargo run
```

## Build Release (native)

```bash
cargo build --release
```

The optimized binary will be at `target/release/rust-pong` (or
`rust-pong.exe` on Windows).

## Build for WebAssembly (GitHub Pages)

1. Add the WASM target once:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Build the WASM binary:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

   This produces `target/wasm32-unknown-unknown/release/rust-pong.wasm`.

3. Create a minimal `index.html` (for example in a new `dist/` folder
   alongside the `.wasm` file) that loads Macroquad's JS shim and your wasm
   binary:

   ```html
   <!DOCTYPE html>
   <html>
     <head>
       <meta charset="utf-8">
       <title>Rust Pong</title>
       <style>
         html, body { margin: 0; background: black; height: 100%; }
         canvas { display: block; margin: 0 auto; }
       </style>
     </head>
     <body>
       <canvas id="glcanvas" tabindex="1"></canvas>
       <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
       <script>
         load("rust-pong.wasm");
       </script>
     </body>
   </html>
   ```

4. Copy `rust-pong.wasm` next to `index.html`, then push both to a `gh-pages`
   branch (or your `docs/` folder, depending on how your repo's GitHub Pages
   is configured). GitHub Pages will serve `index.html`, which loads and runs
   the WASM binary directly in the browser.

## Extending the Game

Because all values live in `src/constants.rs`, tuning feel (paddle speed,
ball speed, AI reaction speed, winning score, etc.) is a one-line change.
The `game.rs` module is the natural place to add new states or mechanics
(e.g. two-player local mode, power-ups) since it already owns the state
machine, scoring, and collision logic.

Ideas for further stretch goals not yet implemented: paddle/wall/score sound
effects and a small particle burst on paddle hits -- both were intentionally
left out to keep this initial version dependency-light and asset-free, but
`ai.rs`'s `Difficulty::next()` helper and the modular structure make them
straightforward to bolt on later.
