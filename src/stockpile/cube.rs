use crate::util::*;
use crate::stockpile::time::*;
use crate::stockpile::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub id: CubeId,
    pub locked: bool,
    pub entries: HashMap<EntryId, Entry>,
    // Todo: polymorphism
    pub relation: LinearModel<EntryId>,
}

impl Cube {
    pub fn new() -> Self {
        Cube::with_id(CubeId::new_stamped())
    }
    pub fn with_id(id: CubeId) -> Self {
        Cube {
            name: String::new(),
            id,
            locked: false,
            entries: HashMap::new(),
            relation: LinearModel::new()
        }
    }
    pub fn id(&self) -> CubeId {
        self.id.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0 && self.name.is_empty()
    }
    pub fn get(&self, id: EntryId) -> &Entry {
        self.entries.get(&id).unwrap()
    }
    pub fn get_mut(&mut self, id: EntryId) -> &mut Entry {
        self.entries.get_mut(&id).unwrap()
    }
}

impl Grow<EntryId> for Cube {
    fn grow(&mut self) -> EntryId {
        let entry = Entry::new();
        let id = entry.id();
        self.entries.insert(id, entry);
        id
    }
}

impl Chain<EntryId> for Cube {
    fn tiptoe(&mut self, obj: EntryId) {
        // Note: shouldn't be used, just fallback.
        self.relation.add(obj, None);
        panic!()
    }
    fn chain(&mut self, obj: EntryId, des: Option<EntryId>) {
        self.relation.add(obj, des)
    }
}

impl Erase<EntryId> for Cube {
    fn erase(&mut self, id: EntryId) {
        if let Some(_) = self.entries.remove(&id) {
            self.relation.del(id);
        }
    }
}
