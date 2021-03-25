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
    SwitchRouter {
        router: Router
    },

    // vm level
    OpenVM {
        cube: Cube,
        meta: CubeMeta
    },
    CloseVM {
        meta: CubeMeta
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
        // Debug..
        /* */
        let vessel = {
            let mut v = Vessel::default();
            let ids: Vec<EntityId> = (0..5).into_iter().map(|_|{
                v.entity_grow()
            }).collect();
            v.entity_get_mut(&ids[0]).map(|x| x.face = "A".to_owned());
            v.entity_get_mut(&ids[1]).map(|x| x.face = "B".to_owned());
            v.entity_get_mut(&ids[2]).map(|x| x.face = "C".to_owned());
            v.entity_get_mut(&ids[3]).map(|x| x.face = "D".to_owned());
            v.entity_get_mut(&ids[4]).map(|x| x.face = "E".to_owned());
            let router = Router::Board;
            v.glass.insert_cube(
                Cube::new(router), 
                CubeMeta { router, idx: 1 }
            );
            v
        };
        // */
        let cubes = vessel.get_cube_vec();
        let cube_vm_vec = cubes.iter().enumerate()
            .map(|(idx,cube)| CubeVM::new(
                idx, 
                cube, 
                &vessel, 
                link.clone()
            )).collect();
        log_obj("Vessel", &vessel);
        Self {
            vessel,
            cube_vm_vec,
            link
        }
    }

    fn update(&mut self, msg_queue: Self::Message) -> ShouldRender {
        log_obj("Update", &msg_queue);
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

                    OpenVM { cube, meta } => {
                        self.vessel.glass.push_cube(cube.clone(), meta.router);
                        let idx = meta.idx;
                        self.cube_vm_vec.push(CubeVM::new(idx, &cube, &self.vessel, self.link.clone()));
                        true
                    }
                    CloseVM { meta } => {
                        self.vessel.glass.remove_cube(meta);
                        self.cube_vm_vec.remove(meta.idx);
                        true
                    }

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
            // clean the glass
            self.vessel.glass.refresh();
            let cube_vec = self.vessel.get_cube_vec();
            // Test: non-invasively update cube_vm_vec.
            for (cube_vm, cube) in self.cube_vm_vec.iter_mut().zip(cube_vec.iter()) {
                cube_vm.update(cube, &self.vessel)
            }
            // let _: Vec<()> = self.cube_vm_vec.iter_mut().zip(cube_vec.iter())
            //     .map(|(cube_vm, cube)| 
            //         cube_vm.update(cube, &self.vessel)
            //     ).collect();

            // self.cube_vm_vec = self.vessel.get_cube_vec().iter().enumerate()
            //     .map(|(idx, cube)| 
            //         // Todo: Update here?.
            //         CubeVM::new(
            //             idx,
            //             cube, 
            //             &self.vessel, 
            //             self.link.clone()
            //         )
            //     ).collect();

            // save
            log_obj("Vessel saved", &self.vessel);
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
