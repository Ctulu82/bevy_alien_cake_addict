use bevy::prelude::*;

mod bonus;
mod camera;
mod constants;
mod player;
mod resources;
mod setup;
mod state;
mod ui;

use bonus::BonusPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use resources::{BonusSpawnTimer, Game};
use setup::SetupPlugin;
use state::GameState;
use ui::UiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .insert_resource(BonusSpawnTimer(Timer::from_seconds(
                5.0,
                TimerMode::Repeating,
            )))
            .init_state::<GameState>()
            .add_plugins((
                SetupPlugin,
                PlayerPlugin,
                CameraPlugin,
                BonusPlugin,
                UiPlugin,
            ));
    }
}
