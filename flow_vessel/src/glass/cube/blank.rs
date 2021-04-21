use super::{Cube, CubeType, CubeMember};

/// Blank with an empty String will be removed
pub struct Blank {
    pub alt: String,
}


impl Into<Cube> for Blank {
    fn into(self) -> Cube {
        Cube {
            cube_type: CubeType::Blank,
            alt: Some(self.alt),
            ..Cube::default()
        }
    }
}

impl From<Cube> for Blank {
    fn from(cube: Cube) -> Self {
        Self {
            alt: cube.alt.unwrap_or_default()
        }
    }
}

impl CubeMember for Blank {}
