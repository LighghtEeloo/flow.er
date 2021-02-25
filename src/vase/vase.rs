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
    WriteEntity(EntityId, EntityField),
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
            let mut vessel = Vessel::default();

            let mut init_test = || -> Result<(), Critic> {
                let mut entity = Entity::default();
                entity.face = format!("{:?}", entity.time);
                vessel.insert_entity(entity.clone(), FlowLink::default())?;
    
                let mut a = Entity::default();
                a.face = format!("A - {}", a.time);
                vessel.insert_entity( a, FlowLink::new_descend_tail(entity.id()) )?;
    
                let mut b = Entity::default();
                b.face = format!("B - {}", b.time);
                vessel.insert_entity( b, FlowLink::new_descend_tail(entity.id()) )?;
    
                vessel.vm_info = HashMap::new();
                vessel.vm_info.insert(Router::Cube, vec![VMType::Linear(entity.id()), VMType::Linear(entity.id())]);
                Ok(())
            };

            init_test().err().map(|x| LOG!("{:?}", x));

            vessel
        };
        // Note: refresh on startup.
        vessel.onload();
        // Test..
        LOG!("Loaded & Trimmed: {:#?}", vessel);

        Self {
            vessel,
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        use VaseMsg::*;
        LOG!("Updating: {:#?}.", messages);
        let mut res = true;
        for message in messages {
            res = match message {
                SwitchRouter(router) => {
                    self.vessel.router = router; true
                }
                NoRender => false,
                WriteEntity(id, field) => {
                    self.vessel.entity_map.get_mut(&id).map(|x| x.update_entity(field));
                    true
                }
                // Test..
                // _ => {
                //     LOG!("No update pattern matched."); false
                // }
            }
        }
        self.vessel.refresh();
        // Only self.vessel is saved.
        self.storage.store(KEY, Json(&self.vessel));
        res
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}

