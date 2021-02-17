use crate::util::*;
use crate::stockpile::prelude::*;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Branch {
    pub cubes: HashMap<CubeId, Cube>,
    pub flow: FlowModel<CubeId>,
}

impl Branch {
    pub fn new() -> Self {
        Branch {
            cubes: HashMap::new(),
            flow: FlowModel::new()
        }
    }
    pub fn empty(&self) -> bool {
        self.cubes.len() == 0
    }
    pub fn get(&self, id: CubeId) -> &Cube {
        self.cubes.get(&id).unwrap()
    }
    pub fn get_mut(&mut self, id: CubeId) -> &mut Cube {
        self.cubes.get_mut(&id).unwrap()
    }
}

impl Grow<CubeId> for Branch {
    fn grow(&mut self) -> CubeId {
        let cube = Cube::new();
        let id = cube.id();
        self.cubes.insert(id, cube);
        id
    }
}

impl Chain<CubeId> for Branch {
    fn tiptoe(&mut self, id: CubeId) {
        self.flow.add(id, None);
    }
    fn chain(&mut self, new_comer: CubeId, host: CubeId) {
        self.flow.add(new_comer, Some(host))
    }
}

impl Erase<CubeId> for Branch {
    fn erase(&mut self, id: CubeId) {
        if let Some(_) = self.cubes.remove(&id) {
            self.flow.del(id);
        }
    }
}
