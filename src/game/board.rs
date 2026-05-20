use bevy::prelude::*;

// 보드의 한 칸이 가진 지형 정보를 표현합니다.
// 현재는 타일마다 약간 다른 높이를 저장해, 플레이어와 케이크가 타일 표면에 맞춰 배치되게 합니다.
pub(super) struct Cell {
    pub(super) height: f32,
}

// 전체 보드 데이터를 보관하는 리소스입니다.
// `setup`에서 타일을 생성하며 채우고, 플레이어 이동과 케이크 생성 시스템이 좌표별 높이를 참조합니다.
#[derive(Resource, Default)]
pub(super) struct Board {
    pub(super) cells: Vec<Vec<Cell>>,
}
