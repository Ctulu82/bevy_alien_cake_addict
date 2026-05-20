use bevy::prelude::*;

use super::constants::{BOARD_SIZE_I, BOARD_SIZE_J, RESET_FOCUS};
use super::resources::Game;
use super::state::GameState;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras)
            .add_systems(Update, focus_camera.run_if(in_state(GameState::Playing)));
    }
}

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),
            2.0 * BOARD_SIZE_J as f32 / 3.0,
            BOARD_SIZE_J as f32 / 2.0 - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
    ));
}

fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.0;

    if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
        let transform_query = transforms.p1();
        if let (Ok(player_transform), Ok(bonus_transform)) = (
            transform_query.get(player_entity),
            transform_query.get(bonus_entity),
        ) {
            game.camera_should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.5);
        }
    } else if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.p1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }

    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_secs();
        game.camera_is_focus += camera_motion;
    }

    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}
