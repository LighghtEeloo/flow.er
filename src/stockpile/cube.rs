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


impl IdentityProduct<CubeId> for Cube {
    fn with_id(id: CubeId) -> Self {
        Self {
            name: String::new(),
            id,
            locked: false,
            entries: HashMap::new(),
            relation: LinearModel::new()
        }
    }
    fn id(&self) -> CubeId {
        self.id.clone()
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityMap<EntryId, Entry> for Cube {
    fn is_empty(&self) -> bool {
        self.entries.len() == 0 && self.name.is_empty()
    }
    fn get_cloned(&self, id: EntryId) -> Entry {
        self.entries.get(&id).cloned().unwrap_or_default()
    }

    fn get_update(&mut self, id: EntryId) -> &Entry {
        if !self.entries.contains_key(&id) {
            LOG!("Error: entry with id {:?} not found.", id);
            let no_data = self.entries.insert(id, Entry::with_id(id)).is_none();
            // Note: should be None;
            assert!(no_data)
        }
        self.entries.get(&id).unwrap()
    }

    fn get_mut(&mut self, id: EntryId) -> &mut Entry {
        if !self.entries.contains_key(&id) {
            LOG!("Error: entry with id {:?} not found.", id);
            let no_data = self.entries.insert(id, Entry::with_id(id)).is_none();
            // Note: should be None;
            assert!(no_data)
        }
        self.entries.get_mut(&id).unwrap()
    }

    fn set(&mut self, id: EntryId, value: Entry) -> Result<(), (EntryId, Entry)> {
        if self.entries.contains_key(&id) {
            let has_data = self.entries.insert(id, Entry::with_id(id)).is_some();
            // Note: should be Some;
            assert!(has_data);
            Ok(())
        } else {
            Err((id, value))
        }
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
