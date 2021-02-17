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
    // pub fn list_orphan(&self) -> Vec<CubeId> {
    //     self.cubes
    //         .keys().cloned()
    //         .filter(|x| if let None = self.flow.data.get(x) {true} else {false} )
    //         .collect()
    // }
    pub fn get(&self, id: CubeId) -> &Cube {
        self.cubes.get(&id).unwrap()
    }
    pub fn get_mut(&mut self, id: CubeId) -> &mut Cube {
        self.cubes.get_mut(&id).unwrap()
    }
    pub fn clean(&mut self) {
        let kill_list: Vec<CubeId> = self.cubes.iter()
            .map(|(&key, cube)| {
                if cube.name.is_empty() { Some(key) }
                else { None }
            })
            .filter(|key| key.is_some())
            .map(|key| key.unwrap())
            .collect();
        for x in kill_list {
            self.erase(x);
        }
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
    fn tiptoe(&mut self, obj: CubeId) {
        self.flow.add_orphan(obj);
    }
    fn chain(&mut self, obj: CubeId, des: Option<CubeId>) {
        self.flow.add(obj, des)
    }
}

impl Erase<CubeId> for Branch {
    fn erase(&mut self, id: CubeId) {
        if let Some(_) = self.cubes.remove(&id) {
            self.flow.del(id);
        }
    }
}
