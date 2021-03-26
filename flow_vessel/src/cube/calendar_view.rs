use super::{Cube, CubeType, now, SystemTime};

pub struct CalendarView {
    pub current: SystemTime,
}

impl Into<Cube> for CalendarView {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::CalendarView,
            time: Some(self.current),
            ..Cube::default()
        }
    }
}

impl From<Cube> for CalendarView {
    fn from(cube: Cube) -> Self {
        Self {
            current: cube.time.unwrap_or(now())
        }
    }
}