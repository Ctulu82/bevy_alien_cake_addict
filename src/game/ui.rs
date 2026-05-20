use bevy::prelude::*;

use super::score::Score;
use super::state::GameState;

// 점수판, 게임 오버 화면, 재시작 입력 등 UI 관련 시스템을 묶는 플러그인입니다.
pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    // 플레이 UI와 게임 오버 UI에 필요한 시스템을 상태별로 등록합니다.
    // 플레이 상태에서는 점수판을 만들고 갱신하며, 게임 오버 상태에서는 최종 결과와 재시작 입력을 처리합니다.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_scoreboard)
            .add_systems(
                Update,
                scoreboard_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::GameOver), display_score)
            .add_systems(
                Update,
                game_over_keyboard.run_if(in_state(GameState::GameOver)),
            );
    }
}

// 플레이 화면 왼쪽 위에 점수판 텍스트 엔티티를 생성합니다.
// `DespawnOnExit(GameState::Playing)`을 붙여 게임 오버로 나갈 때 자동으로 제거되게 합니다.
fn setup_scoreboard(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Text::new("Score:"),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(5),
            ..default()
        },
    ));
}

// 현재 `Score` 리소스의 점수를 읽어 화면의 점수판 텍스트를 갱신합니다.
// 매 프레임 실행되므로 케이크를 먹거나 놓쳐 점수가 바뀌면 즉시 UI에 반영됩니다.
fn scoreboard_system(score: Res<Score>, mut display: Single<&mut Text>) {
    display.0 = format!("Sugar Rush: {}", score.current);
}

// 게임 오버 화면에서 스페이스바 입력을 감지해 새 플레이 상태로 전환합니다.
// 상태가 `Playing`으로 바뀌면 setup 시스템이 다시 실행되어 새 판이 시작됩니다.
fn game_over_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

// 게임 오버 상태로 들어왔을 때 최종으로 먹은 케이크 개수를 화면 중앙에 표시합니다.
// 이 UI 역시 게임 오버 상태를 벗어나면 자동으로 정리되도록 `DespawnOnExit`을 사용합니다.
fn display_score(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Node {
            width: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(format!("Cake eaten: {}", score.cake_eaten)),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 1.0)),
        )],
    ));
}
