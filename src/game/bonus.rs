use bevy::prelude::*;
use rand::RngExt;

use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::resources::{BonusSpawnTimer, Game, Random};
use super::state::GameState;

pub(super) struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bonus, rotate_bonus).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_bonus(
    time: Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut rng: ResMut<Random>,
) {
    if !timer.0.tick(time.delta()).is_finished() {
        return;
    }

    if let Some(entity) = game.bonus.entity {
        game.score -= 3;
        commands.entity(entity).despawn();
        game.bonus.entity = None;
        if game.score <= -5 {
            next_state.set(GameState::GameOver);
            return;
        }
    }

    loop {
        game.bonus.i = rng.random_range(0..BOARD_SIZE_I);
        game.bonus.j = rng.random_range(0..BOARD_SIZE_J);
        if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
            break;
        }
    }
    game.bonus.entity = Some(
        commands
            .spawn((
                DespawnOnExit(GameState::Playing),
                Transform::from_xyz(
                    game.bonus.i as f32,
                    game.board[game.bonus.j][game.bonus.i].height + 0.2,
                    game.bonus.j as f32,
                ),
                SceneRoot(game.bonus.handle.clone()),
                children![(
                    PointLight {
                        color: Color::srgb(1.0, 1.0, 0.0),
                        intensity: 500_000.0,
                        range: 10.0,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 2.0, 0.0),
                )],
            ))
            .id(),
    );
}

fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
    if let Some(entity) = game.bonus.entity
        && let Ok(mut cake_transform) = transforms.get_mut(entity)
    {
        cake_transform.rotate_y(time.delta_secs());
        cake_transform.scale =
            Vec3::splat(1.0 + (game.score as f32 / 10.0 * ops::sin(time.elapsed_secs())).abs());
    }
}
