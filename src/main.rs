use bevy::prelude::*;

mod game;

use game::GamePlugin;

// 애플리케이션의 진입점입니다.
// Bevy 기본 플러그인을 등록해 창, 렌더링, 입력, 에셋 로딩 등을 활성화하고,
// 게임 전용 로직은 `GamePlugin`에 위임합니다.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
