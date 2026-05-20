use bevy::prelude::*;
use rand::RngExt;

use super::board::Board;
use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::player::PlayerState;
use super::resources::Random;
use super::score::Score;
use super::state::GameState;

// 케이크 생성과 애니메이션 시스템을 묶는 플러그인입니다.
pub(super) struct BonusPlugin;

// 케이크가 새로 등장하는 주기를 관리하는 타이머 리소스입니다.
// 타이머가 끝났을 때 기존 케이크를 처리하고 다음 케이크를 생성합니다.
#[derive(Resource)]
pub(super) struct BonusSpawnTimer(pub(super) Timer);

// 현재 케이크의 ECS 엔티티, 보드 좌표, GLB 씬 핸들을 보관하는 리소스입니다.
// 케이크가 아직 생성되지 않은 시점에는 `entity`가 `None`입니다.
#[derive(Resource, Default)]
pub(super) struct BonusState {
    pub(super) entity: Option<Entity>,
    pub(super) i: usize,
    pub(super) j: usize,
    pub(super) handle: Handle<Scene>,
}

impl Plugin for BonusPlugin {
    // 케이크 생성과 회전 시스템을 플레이 상태에만 묶어서 등록합니다.
    // 게임 오버 상태에서는 새 케이크가 나오거나 기존 케이크 애니메이션이 진행되지 않습니다.
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bonus, rotate_bonus).run_if(in_state(GameState::Playing)),
        );
    }
}

// 일정 시간마다 케이크를 새 위치에 생성합니다.
// 이전 케이크를 먹지 못한 상태라면 점수를 차감하고, 점수가 기준 이하로 떨어지면 게임 오버로 전환합니다.
// 새 케이크는 플레이어가 서 있는 칸을 피해서 무작위 보드 좌표에 배치됩니다.
fn spawn_bonus(
    time: Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    board: Res<Board>,
    player: Res<PlayerState>,
    mut bonus: ResMut<BonusState>,
    mut score: ResMut<Score>,
    mut rng: ResMut<Random>,
) {
    if !timer.0.tick(time.delta()).is_finished() {
        return;
    }

    if let Some(entity) = bonus.entity {
        score.current -= 3;
        commands.entity(entity).despawn();
        bonus.entity = None;
        if score.current <= -5 {
            next_state.set(GameState::GameOver);
            return;
        }
    }

    loop {
        bonus.i = rng.random_range(0..BOARD_SIZE_I);
        bonus.j = rng.random_range(0..BOARD_SIZE_J);
        if bonus.i != player.i || bonus.j != player.j {
            break;
        }
    }
    bonus.entity = Some(
        commands
            .spawn((
                DespawnOnExit(GameState::Playing),
                Transform::from_xyz(
                    bonus.i as f32,
                    board.cells[bonus.j][bonus.i].height + 0.2,
                    bonus.j as f32,
                ),
                SceneRoot(bonus.handle.clone()),
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

// 현재 생성된 케이크 모델을 매 프레임 회전시키고 크기를 살짝 변화시킵니다.
// 점수가 높을수록 크기 변화 폭이 커져, 게임 진행 상황이 케이크 애니메이션에 반영됩니다.
fn rotate_bonus(
    bonus: Res<BonusState>,
    score: Res<Score>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    if let Some(entity) = bonus.entity
        && let Ok(mut cake_transform) = transforms.get_mut(entity)
    {
        cake_transform.rotate_y(time.delta_secs());
        cake_transform.scale =
            Vec3::splat(1.0 + (score.current as f32 / 10.0 * ops::sin(time.elapsed_secs())).abs());
    }
}
