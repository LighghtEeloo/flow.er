use crate::util::*;

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
    fn find(&self, tag: &Tag) -> bool {
        self.position(&tag).is_some()
    }
    pub fn insert(&mut self, index: usize, tag: Tag) -> bool {
        let inserting = !self.find(&tag) && index <= self.data.len();
        if inserting {
            self.data.insert(index, tag);
        }
        inserting
    }
    pub fn push(&mut self, tag: Tag) -> bool {
        let inserting = !self.find(&tag);
        if inserting {
            self.data.push(tag);
        }
        inserting
    }
    pub fn remove(&mut self, tag: Tag) -> bool {
        let removing = self.find(&tag);
        self.data.retain(|x| x.clone() != tag);
        removing
    }
}
