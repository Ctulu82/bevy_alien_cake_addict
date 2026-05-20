# Bevy Alien Cake Addict

A small 3D game built with Bevy. Move the alien around the board, eat birthday cakes, and avoid missing too many cake spawns.

## Gameplay

- Move the alien across the board with the arrow keys.
- Eat the cake before the next one spawns.
- Eating a cake increases the score.
- Missing a cake decreases the score.
- The game ends when the score drops too low.
- Press `Space` on the game over screen to restart.

## Requirements

- Rust 2024 edition compatible toolchain
- Cargo

## Run

```sh
cargo run
```

For an optimized build:

```sh
cargo run --release
```

## Check

```sh
cargo fmt --check
cargo check
```

## Project Structure

```text
src/
  main.rs              # Bevy app entry point
  game/
    mod.rs            # GamePlugin composition
    state.rs          # Game states
    resources.rs      # Shared utility resources
    constants.rs      # Board and camera constants
    board.rs          # Board cell data
    score.rs          # Score and cake count resource
    setup.rs          # World, board, player, and asset setup
    player.rs         # Player movement and cake pickup logic
    camera.rs         # Camera setup and focus tracking
    bonus.rs          # Cake spawn and animation logic
    ui.rs             # Scoreboard and game over UI

assets/
  models/AlienCake/   # GLB assets used by the game
```

## Architecture

The game logic is organized around Bevy plugins. `main.rs` registers `DefaultPlugins` and the top-level `GamePlugin`. `GamePlugin` initializes focused resources for the board, score, player, bonus, and camera, then composes sub-plugins for setup, player control, camera behavior, bonus spawning, and UI.
