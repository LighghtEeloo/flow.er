use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;
use super::prelude::{Vessel};


pub struct Vase {
    pub vessel: Vessel,
    pub storage: StorageService,
    pub link: ComponentLink<Self>,
}

const KEY: &str = "flow.er.data";

#[derive(Debug, Clone)]
pub enum VaseMsg {
    SwitchRouter(Router),
    /// Add Entity **with Link**
    AddEntity(FlowLink<EntityId>),
    LinkEntity(EntityId, FlowLink<EntityId>),
    WriteEntity(EntityId, EntityField),
    EraseEntity(EntityId),
    SetFocusId(VMMeta, EntityId),
    Focus(VMMeta),
    // DelayedFocus(VMMeta),
    Wander(VMMeta, Direction, bool),
    NoRender
}

impl Component for Vase {
    type Message = Vec<VaseMsg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let mut vessel: Vessel = {
            let vessel = if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vessel::default()
            };
            // Debug..
            // let mut vessel = Vessel::default();
            let mut vessel = vessel;

            let mut init_test = || -> Result<(), Critic> {
                let mut root_id = vessel.flow.roots.get(0).cloned().unwrap_or_default();
                // let mut entity = Entity::default();
            //     entity.face = format!("{:?}", entity.time);
            //     vessel.insert_entity(entity.clone(), FlowLink::default())?;
    
            //     let mut a = Entity::default();
            //     a.face = format!("A - {}", a.time);
            //     vessel.insert_entity( a, FlowLink::new_descend_tail(entity.id()) )?;
    
            //     let mut b = Entity::default();
            //     b.face = format!("B - {}", b.time);
            //     vessel.insert_entity( b, FlowLink::new_descend_tail(entity.id()) )?;
    
                vessel.vm_info = HashMap::new();
                vessel.vm_info.insert(Router::Cube, vec![VMType::Inkblot(root_id)]);
                vessel.vm_info.insert(Router::Cube, vec![VMType::Linear(root_id), VMType::Inkblot(root_id)]);
                // vessel.vm_info.insert(Router::Cube, vec![VMType::Linear(entity.id()), VMType::Linear(entity.id())]);
                Ok(())
            };

            init_test().err().map(|x| LOG!("{:?}", x));

            vessel
        };
        // Note: refresh on startup.
        vessel.onload();
        // Test..
        LOG!("Loaded {} & Trimmed: {:#?}", KEY, vessel);

        Self {
            vessel,
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        use VaseMsg::*;
        if messages.is_empty() { return false }
        LOG!("Updating: {:#?}.", messages);
        // LOG!("self.vessel.vm_map: {:#?}", self.vessel.vm_map);
        let mut res = true;
        for message in messages {
            res = match message {
                SwitchRouter(router) => {
                    self.vessel.router = router; true
                }
                NoRender => false,
                AddEntity(flow_link) => {
                    self.vessel.insert_entity(Entity::default(), flow_link).err();
                    true
                }
                LinkEntity(id, flow_link) => {
                    self.vessel.flow.update_link(id, flow_link);
                    true
                }
                WriteEntity(id, field) => {
                    self.vessel.entity_map.get_mut(&id).map(|x| x.update_entity(field));
                    true
                }
                EraseEntity(id) => {
                    self.vessel.erase_entity(id).err();
                    true
                }
                SetFocusId((router, vm_idx), id) => {
                    self.vessel.vm_map.get_mut(&router).map(|vec| {
                        vec.get_mut(vm_idx).map(|vm| {
                            vm.focus(id)
                        })
                    });
                    true
                }
                Focus((router, vm_idx)) => {
                    self.vessel.vm_map.get(&router).map(|vec| {
                        vec.get(vm_idx).map(|vm| -> Option<()> {
                            let ref_obj = vm.current_ref()?;
                            if let Some(input) = ref_obj.cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                            None
                        })
                    });
                    true
                }
                // DelayedFocus(vm_meta) => {
                //     self.revisit(Vasey![Focus(vm_meta)]);
                //     true
                // }
                Wander(vm_meta, dir, fixed) => {
                    let (router, vm_idx) = vm_meta;
                    self.vessel.vm_map.get_mut(&router).map(|vec| {
                        vec.get_mut(vm_idx).map(|vm| -> Option<()> {
                            vm.wander(dir, fixed);
                            None
                        })
                    });
                    self.revisit(Vasey![Focus(vm_meta)]);
                    true
                }
                // Test..
                // _ => {
                //     LOG!("No update pattern matched."); false
                // }
            };
            self.vessel.refresh();
        }
        // Only self.vessel is saved.
        self.storage.store(KEY, Json(&self.vessel));
        // Test..
        // LOG!("Updated: {:#?}", self.vessel);
        res
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}


impl Vase {
    pub fn revisit(&mut self, msgs: Vec<VaseMsg>) {
        self.link.callback(move |_| { msgs.clone() }).emit(());
    }
}
