use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub(super) struct BonusSpawnTimer(pub(super) Timer);

pub(super) struct Cell {
    pub(super) height: f32,
}

#[derive(Default)]
pub(super) struct Player {
    pub(super) entity: Option<Entity>,
    pub(super) i: usize,
    pub(super) j: usize,
    pub(super) move_cooldown: Timer,
}

#[derive(Default)]
pub(super) struct Bonus {
    pub(super) entity: Option<Entity>,
    pub(super) i: usize,
    pub(super) j: usize,
    pub(super) handle: Handle<Scene>,
}

#[derive(Resource, Default)]
pub(super) struct Game {
    pub(super) board: Vec<Vec<Cell>>,
    pub(super) player: Player,
    pub(super) bonus: Bonus,
    pub(super) score: i32,
    pub(super) cake_eaten: u32,
    pub(super) camera_should_focus: Vec3,
    pub(super) camera_is_focus: Vec3,
}

#[derive(Resource, Deref, DerefMut)]
pub(super) struct Random(pub(super) ChaCha8Rng);
