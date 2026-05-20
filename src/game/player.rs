use std::f32::consts::PI;

use bevy::prelude::*;

use super::board::Board;
use super::bonus::BonusState;
use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J};
use super::score::Score;
use super::state::GameState;

// 플레이어 관련 시스템을 묶는 플러그인입니다.
// 현재는 이동과 케이크 획득 판정을 담당하는 시스템을 등록합니다.
pub(super) struct PlayerPlugin;

// 플레이어의 ECS 엔티티와 보드 좌표, 이동 쿨다운을 보관하는 리소스입니다.
// 모델의 실제 위치는 `Transform`에 있고, 이 리소스는 게임 규칙 기준의 격자 위치를 관리합니다.
#[derive(Resource, Default)]
pub(super) struct PlayerState {
    pub(super) entity: Option<Entity>,
    pub(super) i: usize,
    pub(super) j: usize,
    pub(super) move_cooldown: Timer,
}

impl Plugin for PlayerPlugin {
    // 플레이 중에만 플레이어 이동 시스템이 동작하도록 등록합니다.
    // `GameState::GameOver` 상태에서는 입력을 받아도 플레이어 위치가 바뀌지 않습니다.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

// 방향키 입력을 읽어 플레이어를 보드 위에서 한 칸씩 이동시킵니다.
// 이동 쿨다운을 사용해 너무 빠른 연속 이동을 막고, 이동 방향에 맞춰 모델 회전도 갱신합니다.
// 이동 후 플레이어와 케이크 위치가 같으면 케이크를 먹은 것으로 처리하고 점수와 누적 개수를 올립니다.
fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    board: Res<Board>,
    mut player: ResMut<PlayerState>,
    mut bonus: ResMut<BonusState>,
    mut score: ResMut<Score>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if player.move_cooldown.tick(time.delta()).is_finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if player.i < BOARD_SIZE_I - 1 {
                player.i += 1;
            }
            rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if player.i > 0 {
                player.i -= 1;
            }
            rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if player.j < BOARD_SIZE_J - 1 {
                player.j += 1;
            }
            rotation = PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if player.j > 0 {
                player.j -= 1;
            }
            rotation = 0.0;
            moved = true;
        }

        if moved {
            player.move_cooldown.reset();
            *transforms.get_mut(player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    player.i as f32,
                    board.cells[player.j][player.i].height,
                    player.j as f32,
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            };
        }
    }

    if let Some(entity) = bonus.entity
        && player.i == bonus.i
        && player.j == bonus.j
    {
        score.current += 2;
        score.cake_eaten += 1;
        commands.entity(entity).despawn();
        bonus.entity = None;
    }
}
