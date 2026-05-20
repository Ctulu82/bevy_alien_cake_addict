use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::board::{Board, Cell};
use super::bonus::BonusState;
use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::player::PlayerState;
use super::resources::Random;
use super::score::Score;
use super::state::GameState;

// 한 판을 시작하는 데 필요한 월드 초기화 시스템을 묶는 플러그인입니다.
pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    // 플레이 상태로 진입할 때마다 월드 초기화 시스템을 실행하도록 등록합니다.
    // 게임 오버 후 재시작할 때도 같은 시스템이 호출되어 점수, 보드, 플레이어 위치가 새로 준비됩니다.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

// 한 판의 게임을 시작하기 위한 월드 상태를 구성합니다.
// 난수 생성기를 준비하고, 점수와 플레이어 상태를 초기화하며,
// 조명, 보드 타일, 플레이어 모델, 케이크 에셋 핸들을 생성하거나 저장합니다.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
    mut player: ResMut<PlayerState>,
    mut bonus: ResMut<BonusState>,
    mut score: ResMut<Score>,
) {
    let mut rng = if std::env::var("GITHUB_ACTIONS") == Ok("true".to_string()) {
        // We're seeding the PRNG here to make this example deterministic for testing purposes.
        // This isn't strictly required in practical use unless you need your app to be deterministic.
        ChaCha8Rng::seed_from_u64(19878367467713)
    } else {
        ChaCha8Rng::from_rng(&mut rand::rng())
    };

    score.cake_eaten = 0;
    score.current = 0;
    player.i = BOARD_SIZE_I / 2;
    player.j = BOARD_SIZE_J / 2;
    player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

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
    board.cells = (0..BOARD_SIZE_J)
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

    player.entity = Some(
        commands
            .spawn((
                DespawnOnExit(GameState::Playing),
                Transform {
                    translation: Vec3::new(
                        player.i as f32,
                        board.cells[player.j][player.i].height,
                        player.j as f32,
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

    bonus.handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/cakeBirthday.glb"));

    commands.insert_resource(Random(rng));
}
