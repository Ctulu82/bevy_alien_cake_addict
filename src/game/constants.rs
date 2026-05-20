// 보드의 가로 방향 칸 수입니다.
pub(super) const BOARD_SIZE_I: usize = 14;

// 보드의 세로 방향 칸 수입니다.
pub(super) const BOARD_SIZE_J: usize = 21;

// 카메라가 추적할 대상이 없을 때 바라볼 기본 보드 중앙 좌표입니다.
pub(super) const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];
