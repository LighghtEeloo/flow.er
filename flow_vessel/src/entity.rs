use serde::{Serialize, Deserialize};

use super::identity::*;
use super::tag::*;
use super::time::TimeLog;


// Entity Area

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    pub time: Option<TimeLog>,
    pub face: Face,
    pub bubble: Bubble,
    pub process: ProcessStatus,
    pub tags: TagSet,
}

impl Entity {
    pub fn new_time(id_factory: &EntityIdFactory) -> Self {
        Self {
            id: id_factory.time_id(),
            time: None,
            face: Face::new(),
            bubble: Bubble::new(),
            process: ProcessStatus::New,
            tags: TagSet::new()
        }
    }
    pub fn new_incr(id_factory: &mut EntityIdFactory) -> Self {
        Self {
            id: id_factory.time_id(),
            time: None,
            face: Face::new(),
            bubble: Bubble::new(),
            process: ProcessStatus::New,
            tags: TagSet::new()
        }
    }
    pub fn id(&self) -> &EntityId {
        &self.id
    }
    // pub fn update_entity(&mut self, field: EntityField) {
    //     use EntityField::*;
    //     match field {
    //         TimeStamp(t) => { self.time = t }
    //         Face(f) => { self.face = f }
    //         Bubble(b) => { self.bubble = b }
    //         ProcessStatus(p) => { self.process = p }
    //         TagSet(tf) => { 
    //             self.tags.update_tagset(tf)
    //         }
    //     }
    // }
}

impl Default for Entity {
    fn default() -> Self { 
        Entity {
            id: EntityId::default(),
            time: None,
            face: Face::default(),
            bubble: Bubble::default(),
            process: ProcessStatus::New,
            tags: TagSet::default()
        }
    }
}


pub type Face = String;
pub type Bubble = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProcessStatus {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}


use ProcessStatus::*;
impl ProcessStatus {
    pub fn type_str(&self) -> String {
        match self {
            Done => "Done",
            Marching => "Marching",
            Pending => "Pending",
            Planning => "Planning",
            New => "New",
        }.to_string()
    }
    pub fn reflect(name: &str) -> Self {
        match name {
            "Done" => Done,
            "Marching" => Marching,
            "Pending" => Pending,
            "Planning" => Planning,
            "New" => New,
            _ => New,
        }
    }
    pub fn vec_all() -> Vec<Self> {
        vec! {
            New,
            Planning,
            Pending,
            Marching,
            Done,
        }
    }
    pub fn type_src(&self) -> String {
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}

// #[derive(Debug, Clone)]
// pub enum EntityField {
//     TimeStamp(TimeStamp),
//     Face(Face),
//     Bubble(Bubble),
//     ProcessStatus(ProcessStatus),
//     TagSet(TagSetField)
// }
