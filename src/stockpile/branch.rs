use crate::util::*;
use crate::stockpile::prelude::*;


#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Branch {
    pub cubes: HashMap<CubeId, Cube>,
    pub flow: FlowModel<CubeId>,
}


impl Branch {
    pub fn new() -> Self {
        Self::default()
    }
    /// Remove all is_empty() cubes.
    pub fn clean(&mut self) {
        let kill_list: Vec<CubeId> = self.cubes.iter()
            .map(|(&key, cube)| {
                if cube.is_empty() { Some(key) }
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

impl IdentityMap<CubeId, Cube> for Branch {
    fn is_empty(&self) -> bool {
        self.cubes.len() == 0
    }
    fn get_cloned(&self, id: CubeId) -> Cube {
        self.cubes.get(&id).cloned().unwrap_or_default()
    }

    fn get_update(&mut self, id: CubeId) -> &Cube {
        if !self.cubes.contains_key(&id) {
            LOG!("Error: cube with id {:?} not found.", id);
            let no_data = self.cubes.insert(id, Cube::with_id(id)).is_none();
            // Note: should be None;
            assert!(no_data)
        }
        self.cubes.get(&id).unwrap()
    }

    fn get_mut(&mut self, id: CubeId) -> &mut Cube {
        if !self.cubes.contains_key(&id) {
            LOG!("Error: cube with id {:?} not found.", id);
            let no_data = self.cubes.insert(id, Cube::with_id(id)).is_none();
            // Note: should be None;
            assert!(no_data)
        }
        self.cubes.get_mut(&id).unwrap()
    }

    fn set(&mut self, id: CubeId, value: Cube) -> Result<(), (CubeId, Cube)> {
        if self.cubes.contains_key(&id) {
            let has_data = self.cubes.insert(id, Cube::with_id(id)).is_some();
            // Note: should be Some;
            assert!(has_data);
            Ok(())
        } else {
            Err((id, value))
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
