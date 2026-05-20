use bevy::prelude::*;

// 현재 게임의 점수와 누적 케이크 획득 수를 보관하는 리소스입니다.
// 플레이어와 케이크 시스템이 값을 변경하고, UI 시스템이 이 값을 읽어 화면에 표시합니다.
#[derive(Resource, Default)]
pub(super) struct Score {
    pub(super) current: i32,
    pub(super) cake_eaten: u32,
}
