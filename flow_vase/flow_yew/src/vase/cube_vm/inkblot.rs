use flow_vessel::{Cube, EntityId};
use super::CubeView;

#[derive(Clone)]
pub struct Inkblot {
    obj: EntityId
}

impl From<Cube> for Inkblot {
    fn from(cube: Cube) -> Self {
        Self {
            obj: cube.obj.unwrap_or_default()
        }
    }
}

impl Into<CubeView> for Inkblot {
    fn into(self) -> CubeView {
        CubeView::Inkblot {
            inkblot: self
        }
    }
}

impl Inkblot {
    pub fn new_cube(cube: Cube) -> CubeView {
        let inkblot: Self = cube.into();
        inkblot.into()
    }
}
