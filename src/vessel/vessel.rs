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
        self.vm_load();
        Ok(())
    }
    pub fn refresh(&mut self) -> Result<(), Critic> {
        self.vm_refresh();
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
    fn vm_load(&mut self) {
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
    fn vm_refresh(&mut self) {
        let flow = &self.flow;
        self.vm_map.values_mut().for_each(|vec_vm| {
            for vm in vec_vm.iter_mut() {
                vm.flow_update(flow);
                vm.compute();
            }
        });
    }
    /// add / updates an entity together with its flow. 
    /// 
    /// if entity exists, updates entity_map but also complains, and no flow will be linked; 
    pub fn insert_entity(&mut self, entity: Entity, flow_link: FlowLink<EntityId>) -> Result<(), Critic> {
        let id = entity.id();
        self.entity_map.insert(id, entity);
        self.flow.add(id)?;
        self.flow.link(id, flow_link)?;
        Ok(())
    }
    pub fn erase_entity(&mut self, obj: EntityId) -> Result<(), Critic> {
        self.flow.del(obj)?;
        self.entity_map.remove(&obj);
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

impl Router {
    pub fn type_str(&self) -> &'static str {
        use Router::*;
        match self {
            Cube => "cube",
            Flow => "flow",
            Calendar => "calendar",
            TimeCapsule => "time-capsule",
            Settings => "settings"
        }
    }
}

// #[derive(Debug, Default, Deserialize, Serialize)]
pub type VMInfo = HashMap< Router, Vec< VMType > >;
pub type VMMeta = (Router, usize);
pub type VMMap = HashMap< Router, Vec< Box<dyn Artist<EntityId>> > >;

#[derive(Debug, Deserialize, Serialize)]
pub enum VMType {
    Inkblot(EntityId),
    Linear(EntityId),
    Tree(EntityId),
    Graph(Vec<EntityId>),
}

