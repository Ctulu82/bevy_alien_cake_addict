use bevy::prelude::*;

use super::bonus::BonusState;
use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J, RESET_FOCUS};
use super::player::PlayerState;
use super::state::GameState;

// 카메라 생성과 추적 시스템을 묶는 플러그인입니다.
pub(super) struct CameraPlugin;

// 카메라의 목표 초점과 현재 초점을 보관하는 리소스입니다.
// 목표 초점은 플레이어와 케이크 위치에 따라 바뀌고, 현재 초점은 목표를 향해 부드럽게 이동합니다.
#[derive(Resource, Default)]
pub(super) struct CameraFocus {
    pub(super) should_focus: Vec3,
    pub(super) is_focus: Vec3,
}

impl Plugin for CameraPlugin {
    // 카메라 생성과 추적 시스템을 등록합니다.
    // 카메라는 앱 시작 시 한 번 생성하고, 플레이 중에는 매 프레임 부드럽게 초점을 갱신합니다.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras)
            .add_systems(Update, focus_camera.run_if(in_state(GameState::Playing)));
    }
}

// 3D 카메라를 보드 전체가 보이는 위치에 생성합니다.
// `CameraFocus` 리소스 안의 현재 초점과 목표 초점을 기본 위치로 맞춰,
// 첫 프레임부터 카메라가 안정적인 지점을 바라보도록 합니다.
fn setup_cameras(mut commands: Commands, mut camera_focus: ResMut<CameraFocus>) {
    camera_focus.should_focus = Vec3::from(RESET_FOCUS);
    camera_focus.is_focus = camera_focus.should_focus;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),
            2.0 * BOARD_SIZE_J as f32 / 3.0,
            BOARD_SIZE_J as f32 / 2.0 - 0.5,
        )
        .looking_at(camera_focus.is_focus, Vec3::Y),
    ));
}

// 플레이어와 케이크 위치를 기준으로 카메라가 바라볼 목표 지점을 계산합니다.
// 둘 다 있으면 두 위치의 중간을 보고, 플레이어만 있으면 플레이어를 따라가며,
// 아무 대상도 없으면 보드 중앙을 바라보도록 되돌립니다.
// 실제 카메라 초점은 목표 지점으로 즉시 순간이동하지 않고 프레임 시간에 따라 부드럽게 보간됩니다.
fn focus_camera(
    time: Res<Time>,
    player: Res<PlayerState>,
    bonus: Res<BonusState>,
    mut camera_focus: ResMut<CameraFocus>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.0;

    if let (Some(player_entity), Some(bonus_entity)) = (player.entity, bonus.entity) {
        let transform_query = transforms.p1();
        if let (Ok(player_transform), Ok(bonus_transform)) = (
            transform_query.get(player_entity),
            transform_query.get(bonus_entity),
        ) {
            camera_focus.should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.5);
        }
    } else if let Some(player_entity) = player.entity {
        if let Ok(player_transform) = transforms.p1().get(player_entity) {
            camera_focus.should_focus = player_transform.translation;
        }
    } else {
        camera_focus.should_focus = Vec3::from(RESET_FOCUS);
    }

    let mut camera_motion = camera_focus.should_focus - camera_focus.is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_secs();
        camera_focus.is_focus += camera_motion;
    }

    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(camera_focus.is_focus, Vec3::Y);
    }
}
