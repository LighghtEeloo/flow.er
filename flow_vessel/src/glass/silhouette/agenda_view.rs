use super::{Cube, CubeType, CubeMember, SystemTime, now};

pub struct AgendaView {
    pub current: SystemTime,
}

impl Into<Cube> for AgendaView {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::AgendaView,
            time: Some(self.current),
            ..Cube::default()
        }
    }
}

impl From<Cube> for AgendaView {
    fn from(cube: Cube) -> Self {
        Self {
            current: cube.time.unwrap_or(now())
        }
    }
}

impl CubeMember for AgendaView {}
