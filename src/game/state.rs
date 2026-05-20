use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub(super) enum GameState {
    #[default]
    Playing,
    GameOver,
}
