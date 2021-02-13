use crate::util::*;
use crate::stockpile::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cube {
    pub name: String,
    pub locked: bool,
    pub entries: HashMap<EntryId, Entry>,
    // Todo: polymorphism
    pub relation: LinearModel<EntryId>,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            name: String::new(),
            locked: false,
            entries: HashMap::new(),
            relation: LinearModel::new()
        }
    }
    pub fn empty(&self) -> bool {
        self.entries.len() == 0
    }
    pub fn get(&self, id: EntryId) -> &Entry {
        self.entries.get(&id).unwrap()
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
    fn tiptoe(&mut self, id: EntryId) {
        self.relation.add(id, None);
    }
    fn chain(&mut self, new_comer: EntryId, host: EntryId) {
        self.relation.add(new_comer, Some(host))
    }
}

impl Erase<EntryId> for Cube {
    fn erase(&mut self, id: EntryId) {
        if let Some(_) = self.entries.remove(&id) {
            self.relation.del(id);
        }
    }
}
