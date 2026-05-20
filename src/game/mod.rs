use bevy::prelude::*;

mod board;
mod bonus;
mod camera;
mod constants;
mod player;
mod resources;
mod score;
mod setup;
mod state;
mod ui;

use board::Board;
use bonus::{BonusPlugin, BonusSpawnTimer, BonusState};
use camera::{CameraFocus, CameraPlugin};
use player::{PlayerPlugin, PlayerState};
use score::Score;
use setup::SetupPlugin;
use state::GameState;
use ui::UiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    // 게임 전체를 구성하는 최상위 플러그인입니다.
    // 공유 리소스와 상태를 먼저 초기화한 뒤, 실제 동작은 역할별 하위 플러그인에 맡깁니다.
    // 이렇게 두면 `main.rs`는 단순해지고, 각 게임 기능은 독립된 파일에서 관리할 수 있습니다.
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<Score>()
            .init_resource::<PlayerState>()
            .init_resource::<BonusState>()
            .init_resource::<CameraFocus>()
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
