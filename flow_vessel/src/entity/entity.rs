use serde::{Serialize, Deserialize};

use crate::TimeNote;
use super::identity::*;
use super::tag::*;
use super::symbol::*;


// Entity Area

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    pub time: Option<TimeNote>,
    pub face: Face,
    pub bubble: Bubble,
    pub symbol: Symbol,
    pub tags: TagSet,
    pub blocked: bool,
    pub symbol_toggle: bool,
}

impl Entity {
    pub fn new_time(id_factory: &EntityIdFactory) -> Self {
        Self::new_id(&id_factory.time_id())
    }
    pub fn new_incr(id_factory: &mut EntityIdFactory) -> Self {
        Self::new_id(&id_factory.incr_id())
    }
    pub fn new_id(id: &EntityId) -> Self {
        Entity {
            id: id.clone(),
            time: None,
            face: Face::default(),
            bubble: Bubble::default(),
            symbol: Symbol::Linted(Lint::default()),
            tags: TagSet::default(),
            blocked: false,
            symbol_toggle: false,
        }
    }
    pub fn id(&self) -> &EntityId {
        &self.id
    }
    pub fn update_entity(&mut self, field: EntityField) {
        use EntityField::*;
        match field {
            TimeNote(t) => { self.time = Some(t) }
            Face(f) => { self.face = f }
            Bubble(b) => { self.bubble = b }
            Symbol(s) => { self.symbol = s }
            TagSet(tf) => { 
                self.tags.update_tagset(tf)
            }
        }
    }
}

impl Default for Entity {
    fn default() -> Self { 
        Self::new_id(&EntityId::default())
    }
}


pub type Face = String;
pub type Bubble = String;

#[derive(Debug, Clone)]
pub enum EntityField {
    TimeNote(TimeNote),
    Face(Face),
    Bubble(Bubble),
    Symbol(Symbol),
    TagSet(TagSetField)
}
