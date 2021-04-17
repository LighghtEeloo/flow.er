use serde::{Serialize, Deserialize};

pub type Tag = String;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct TagSet {
    data: Vec<Tag>,
}

impl TagSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn into_iter(&self) -> std::vec::IntoIter<Tag> {
        self.data.clone().into_iter()
    }
    pub fn position(&self, tag: &Tag) -> Option<usize> {
        self.into_iter().position(|x| x.clone() == tag.clone())
    }
    pub fn contains(&self, tag: &Tag) -> bool {
        self.position(&tag).is_some()
    }
    pub fn insert(&mut self, index: usize, tag: Tag) -> bool {
        let inserting = !self.contains(&tag) && index <= self.data.len();
        if inserting {
            self.data.insert(index, tag);
        }
        inserting
    }
    pub fn push(&mut self, tag: Tag) -> bool {
        let inserting = !self.contains(&tag);
        if inserting {
            self.data.push(tag);
        }
        inserting
    }
    pub fn remove(&mut self, tag: Tag) -> bool {
        let position = self.position(&tag);
        let removing = position.is_some();
        if let Some(i) = position {
            self.data.remove(i);
        }
        removing
    }
    pub fn update_tagset(&mut self, field: TagSetField) {
        use TagSetField::*;
        match field {
            AddTag(t) => { self.push(t); }
            DelTag(t) => { self.remove(t); }
            ClearTag => { self.data.clear(); }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TagSetField {
    AddTag(Tag),
    DelTag(Tag),
    ClearTag,
}
