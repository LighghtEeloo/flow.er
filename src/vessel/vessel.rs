use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;

/// Vessel is the app data processing unit. 
/// Some part of it will be (de)serialized and stored / exchanged, 
/// while the other parts will be used at runtime.
/// 
/// self.entity_map can store Entity entries according to a given EntityId.
/// self.flow can store the relation of the model.

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Vessel {
    entity_map: HashMap<EntityId, Entity>,
    flow: Flow<EntityId>,
    
    #[serde(skip)]
    vm: Vec<Box<dyn Dancer<EntityId>>>,
    #[serde(skip)]
    refs: HashMap<EntityId, NodeRef>,
    #[serde(skip)]
    buffer: String,
}

impl Vessel {
    /// add / updates an entity. 
    /// 
    /// updates entity_map if entity exists; 
    /// updates flow if not in flow.
    /// 
    /// ignores flow_link if existing in flow.
    pub fn insert_entity(&mut self, entity: Entity, flow_link: FlowLink<EntityId>) -> Result<(), Critic> {
        let id = entity.id();
        self.entity_map.insert(id, entity);
        self.flow.add(id)?;
        self.flow.link(id, flow_link)?;
        Ok(())
    }
    pub fn trim(&mut self) -> Result<(), Critic> {
        match self.flow.trim().err().map(|vec| {
            LOG!("Trimmed: {:?}", vec);
        }) {
            Some(_) => Err(FlowNodeMismatchError),
            _ => Ok(())
        }
    }
}

