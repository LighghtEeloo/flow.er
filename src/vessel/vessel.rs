use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;

/// Vessel is the app data processing unit. 
/// Some part of it will be (de)serialized and stored / exchanged, 
/// while the other parts will be used at runtime.
/// 
/// The entity_map can

#[derive(Debug, Deserialize, Serialize)]
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
    /// ignores flow_add if existing in flow.
    fn insert_entity(&mut self, entity: Entity, flow_add: FlowAdd<EntityId>) {
        let id = entity.id();
        self.entity_map.insert(id, entity);
        self.flow.add(id, flow_add);
    }
}

