use crate::util::*;
use crate::stockpile::prelude::*;


pub struct Branch {
    pub cubes: HashMap<CubeId, Cube>,
    pub flow: FlowModel<CubeId>,
}
