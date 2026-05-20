use std::f32::consts::PI;

use bevy::prelude::*;

use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::resources::Game;
use super::state::GameState;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).is_finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if game.player.i < BOARD_SIZE_I - 1 {
                game.player.i += 1;
            }
            rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if game.player.i > 0 {
                game.player.i -= 1;
            }
            rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if game.player.j < BOARD_SIZE_J - 1 {
                game.player.j += 1;
            }
            rotation = PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if game.player.j > 0 {
                game.player.j -= 1;
            }
            rotation = 0.0;
            moved = true;
        }

        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.i as f32,
                    game.board[game.player.j][game.player.i].height,
                    game.player.j as f32,
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            };
        }
    }

    if let Some(entity) = game.bonus.entity
        && game.player.i == game.bonus.i
        && game.player.j == game.bonus.j
    {
        game.score += 2;
        game.cake_eaten += 1;
        commands.entity(entity).despawn();
        game.bonus.entity = None;
    }
}
