use crate::util::*;

pub type Tag = String;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct TagSet {
    pub data: Vec<Tag>,
}

impl TagSet {
    pub fn new() -> Self {
        Self::default()
    }
}

