use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

// 게임에서 사용하는 난수 생성기 리소스입니다.
// `setup`에서 생성해 저장하고, 케이크 위치를 무작위로 고를 때 사용합니다.
#[derive(Resource, Deref, DerefMut)]
pub(super) struct Random(pub(super) ChaCha8Rng);
