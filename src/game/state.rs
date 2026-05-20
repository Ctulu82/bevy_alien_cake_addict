use bevy::prelude::*;

// 게임의 큰 흐름을 나타내는 상태입니다.
// `Playing`에서는 이동, 카메라, 케이크, 점수판 시스템이 실행되고,
// `GameOver`에서는 결과 화면과 재시작 입력만 처리됩니다.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub(super) enum GameState {
    // 실제 플레이가 진행 중인 기본 상태입니다.
    #[default]
    Playing,
    // 점수가 기준 이하로 떨어졌을 때 진입하는 종료 상태입니다.
    GameOver,
}
