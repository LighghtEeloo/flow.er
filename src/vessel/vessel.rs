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
    pub entity_map: HashMap<EntityId, Entity>,
    pub flow: Flow<EntityId>,
    pub router: Router,
    pub vm_info: VMInfo,

    #[serde(skip)]
    pub vm_map: VMMap,
    #[serde(skip)]
    pub buffer: String,
}

impl Vessel {
    pub fn onload(&mut self) -> Result<(), Critic> {
        // trim flow
        self.trim()?;
        self.vm_update();
        Ok(())
    }
    fn trim(&mut self) -> Result<(), Critic> {
        match self.flow.trim().err().map(|vec| {
            LOG!("Trimmed: {:?}", vec);
        }) {
            Some(_) => Err(FlowNodeMismatchError),
            _ => Ok(())
        }
    }
    fn vm_update(&mut self) {
        // load vm
        self.vm_map = self.vm_info.iter().map(|(&router, vec_vm_type)| {
            let mut vec_vm: Vec< Box<dyn Artist<EntityId>> > = vec![];
            for vm_type in vec_vm_type.iter() {
                match vm_type {
                    VMType::Inkblot(id) => {
                        vec_vm.push(Inkblot::from_flow_boxed(&self.flow, id));
                    }
                    VMType::Linear(id) => {
                        vec_vm.push(Linear::from_flow_boxed(&self.flow, id));
                    }
                    VMType::Tree(id) => {
                        // Todo..
                    }
                    VMType::Graph(vec_id) => {
    
                    }
                }
            }
            (router, vec_vm)
        }).collect();
    }
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
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Router {
    Cube,
    Flow,
    Calendar,
    TimeCapsule,

    Settings,
}

impl Default for Router {
    fn default() -> Router { Router::Cube }
}

// #[derive(Debug, Default, Deserialize, Serialize)]
pub type VMInfo = HashMap< Router, Vec< VMType > >;
pub type VMMap = HashMap< Router, Vec< Box<dyn Artist<EntityId>> > >;

#[derive(Debug, Deserialize, Serialize)]
pub enum VMType {
    Inkblot(EntityId),
    Linear(EntityId),
    Tree(EntityId),
    Graph(Vec<EntityId>),
}

