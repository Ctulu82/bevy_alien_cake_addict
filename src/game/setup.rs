use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::resources::{Cell, Game, Random};
use super::state::GameState;

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let mut rng = if std::env::var("GITHUB_ACTIONS") == Ok("true".to_string()) {
        // We're seeding the PRNG here to make this example deterministic for testing purposes.
        // This isn't strictly required in practical use unless you need your app to be deterministic.
        ChaCha8Rng::seed_from_u64(19878367467713)
    } else {
        ChaCha8Rng::from_rng(&mut rand::rng())
    };

    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = BOARD_SIZE_I / 2;
    game.player.j = BOARD_SIZE_J / 2;
    game.player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

    commands.spawn((
        DespawnOnExit(GameState::Playing),
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    let cell_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/tile.glb"));
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rng.random_range(-0.1..0.1);
                    commands.spawn((
                        DespawnOnExit(GameState::Playing),
                        Transform::from_xyz(i as f32, height - 0.2, j as f32),
                        SceneRoot(cell_scene.clone()),
                    ));
                    Cell { height }
                })
                .collect()
        })
        .collect();

    game.player.entity = Some(
        commands
            .spawn((
                DespawnOnExit(GameState::Playing),
                Transform {
                    translation: Vec3::new(
                        game.player.i as f32,
                        game.board[game.player.j][game.player.i].height,
                        game.player.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/alien.glb")),
                ),
            ))
            .id(),
    );

    game.bonus.handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/cakeBirthday.glb"));

    commands.insert_resource(Random(rng));
}
