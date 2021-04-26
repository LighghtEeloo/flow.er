use serde::{Deserialize, Serialize};

use crate::{Filter, Identity};

pub mod identity;
pub mod symbol;
pub mod tag;
pub mod timenote;

use identity::*;
use symbol::*;
use tag::*;
use timenote::*;

// Entity Area

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    pub time_note: Option<TimeNote>,
    pub face: Face,
    pub bubble: Bubble,
    pub symbol: Symbol,
    pub tags: TagSet,
    #[serde(default)]
    pub blocked: bool,
    #[serde(skip)]
    pub symbol_toggle: bool,
}

impl Entity {
    pub fn new_time(id_factory: &EntityIdFactory) -> Self {
        Self::new_id(&id_factory.time_id())
    }
    pub fn new_incr(id_factory: &mut EntityIdFactory) -> Self {
        Self::new_id(&id_factory.incr_id())
    }
    pub fn new_rotate(id_factory: &mut EntityIdFactory) -> Self {
        Self::new_id(&id_factory.rotate_id())
    }
    pub fn new_id(id: &EntityId) -> Self {
        Entity {
            id: id.clone(),
            time_note: None,
            face: Face::default(),
            bubble: Bubble::default(),
            symbol: Symbol::default(),
            tags: TagSet::default(),
            blocked: false,
            symbol_toggle: false,
        }
    }
    pub fn id(&self) -> &EntityId {
        &self.id
    }
    pub fn duplicate_from(&mut self, dude: &Self) {
        self.symbol = dude.symbol.clone();
    }

    pub fn matched(&self, filter: &Filter) -> bool {
        match filter {
            Filter::Identity(id) => self.id().parse_match(&id),
            Filter::Face(face) => self.face.contains(face),
            Filter::Symbol(s) => &self.symbol == s,
            Filter::Tag(t) => self.tags.contains(&t),
            Filter::All => true,
        }
    }

    /// true if all filter matches
    pub fn matched_all(&self, filters: &Vec<Filter>) -> bool {
        filters.into_iter().fold(true, |is, filter| {
            let matching = self.matched(filter);
            is && matching
        })
    }

    /// false if any filter matches
    pub fn matched_none(&self, filters: &Vec<Filter>) -> bool {
        !filters.into_iter().fold(false, |is, filter| {
            let matching = self.matched(filter);
            is || matching
        })
    }

    pub fn update_entity(&mut self, field: EntityField) {
        use EntityField::*;
        match field {
            TimeNote(t) => self.time_note = Some(t),
            Face(f) => self.face = f,
            Bubble(b) => self.bubble = b,
            Symbol(s) => {
                self.symbol_toggle = false;
                self.symbol = s
            }
            TagSet(tf) => {
                self.tags.update_tagset(tf).ok();
            }
            Blocked => {
                self.blocked = !self.blocked;
            }
            SymbolToggle => {
                self.symbol_toggle = !self.symbol_toggle;
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
    TagSet(TagSetField),
    Blocked,
    SymbolToggle,
}
