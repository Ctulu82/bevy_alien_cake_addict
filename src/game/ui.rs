use bevy::prelude::*;

use super::resources::Game;
use super::state::GameState;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
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

fn scoreboard_system(game: Res<Game>, mut display: Single<&mut Text>) {
    display.0 = format!("Sugar Rush: {}", game.score);
}

fn game_over_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn display_score(mut commands: Commands, game: Res<Game>) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Node {
            width: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(format!("Cake eaten: {}", game.cake_eaten)),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 1.0)),
        )],
    ));
}
