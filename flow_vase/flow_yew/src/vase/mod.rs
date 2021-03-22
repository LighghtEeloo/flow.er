mod view;
mod cube_vm;

use yew::{Component, ComponentLink, Html, ShouldRender};
use flow_vessel::*;

use super::log_obj;
use cube_vm::CubeVM;


pub struct Vase {
    vessel: Vessel,
    cube_vm_vec: Vec<CubeVM>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    // router level
    SwitchRouter{
        router: Router
    },

    // entity level
    EntityUpdate {
        id: EntityId,
        field: EntityField
    },

    // refresh
    Refresh,
}

impl Component for Vase {
    // a conditional queue.
    type Message = Vec<Msg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let vessel_future = Vessel::load();
        let vessel = futures::executor::block_on(vessel_future).unwrap_or(Vessel::new());
        let cubes = vessel.get_cube_vec();
        let cube_vm_vec = cubes.iter()
            .map(|c| CubeVM::new(c, &vessel, link.clone())).collect();
        log_obj("Vessel", &vessel);
        Self {
            vessel,
            cube_vm_vec,
            link
        }
    }

    fn update(&mut self, msg_queue: Self::Message) -> ShouldRender {
        let mut msg_visitor = msg_queue.into_iter();
        while {
            let next = msg_visitor.next();
            if let Some(msg) = next {
                use Msg::*;
                // returns true if non-block, false if needs revisit or finish
                match msg {
                    SwitchRouter{ router} => {
                        self.vessel.router = router;
                        true
                    },

                    EntityUpdate { id, field } => {
                        self.vessel.entity_get_mut(&id).map(|entity| {
                            entity.update_entity(field)
                        });
                        true
                    }

                    Refresh => false,
                }
            } else {
                // quit loop
                false
            }
        } {}
        let left: Vec<Msg> = msg_visitor.collect();
        if !left.is_empty() {
            self.link.callback(move |()| left.clone()).emit(());
            false
        } else {
            // save
            let save_res = futures::executor::block_on(self.vessel.clone().save());
            if save_res.is_err() {
                log_obj("load err", -1);
            }
            true
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}
