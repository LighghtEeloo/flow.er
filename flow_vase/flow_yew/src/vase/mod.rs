mod update;
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
    EntityAdd {
        dude: EntityId,
        owner: EntityId,
        idx: usize
    },
    EntityUpdate {
        id: EntityId,
        field: EntityField
    },
    EntityDive {
        id: EntityId,
        idx: usize,
    },
    EntityEmerge {
        id: EntityId,
        idx: usize,
    },
    EntityDelete {
        id: EntityId
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
        /* *
        let vessel = {
            let mut v = Vessel::default();
            let ids: Vec<EntityId> = (0..7).into_iter().map(|_|{
                v.entity_grow()
            }).collect();
            v.entity_get_mut(&EntityId::default()).map(|x| x.face = "The Ripple of Your Shadow".to_owned());
            v.entity_get_mut(&ids[0]).map(|x| x.face = "Hi Player, ".to_owned());
            v.entity_get_mut(&ids[1]).map(|x| x.face = "it has been a while. ".to_owned());
            v.entity_get_mut(&ids[2]).map(|x| x.face = "I've been watching you, all along the way. ".to_owned());
            v.entity_get_mut(&ids[3]).map(|x| x.face = "You seem lost. ".to_owned());
            v.entity_get_mut(&ids[4]).map(|x| x.face = "Nothing but a long, long dream. ".to_owned());
            v.entity_get_mut(&ids[5]).map(|x| {
                x.face = "Wake up now. ".to_owned();
                x.bubble = 
"Your life is waiting. 
Do what you have to do. 
Be a king. ".to_owned();
            });
            v.entity_get_mut(&ids[6]).map(|x| x.symbol = Symbol::ProcessTracker(Process::New) );
            v.entity_get_mut(&ids[6]).map(|x| x.face = "Do your job.".into() );
            let router = Router::Board;
            let cube: Cube = 
                cubes::Inkblot {
                    obj: ids[5],
                }.into();
            // let cube = Cube::new(router);
            let cube_type = cube.cube_type;
            v.glass.insert_cube(
                cube, 
                CubeMeta { router, idx: 1 , cube_type }
            );
            v
        };
        // */
        let cubes = vessel.get_cube_vec();
        let cube_vm_vec = Self::cube_vm_vec(cubes, &vessel, link.clone());
        if cfg!(debug_assertions) { log_obj("Vessel", &vessel); }
        Self {
            vessel,
            cube_vm_vec,
            link
        }
    }

    fn update(&mut self, msg_queue: Self::Message) -> ShouldRender {
        if msg_queue.len() == 0 { return false }
        if cfg!(debug_assertions) { log_obj("Update", &msg_queue); }
        let mut msg_visitor = msg_queue.into_iter();
        while {
            let next = msg_visitor.next();
            if let Some(msg) = next {
                self.update_msg(msg)
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
            // Todo: deprecated. Auto clean done.
            self.vessel.glass_refresh();
            // if not equal in num then rebuild; else update
            let cubes = self.vessel.get_cube_vec();
            if self.cube_vm_vec.len() != cubes.len() {
                self.cube_vm_vec = Self::cube_vm_vec(cubes, &self.vessel, self.link.clone());
            } else {
                // Test: non-invasively update cube_vm_vec.
                for (idx,(cube_vm, cube)) in self.cube_vm_vec.iter_mut().zip(cubes.iter()).enumerate() {
                    cube_vm.update(idx, cube, &self.vessel)
                }
            }

            // save
            // Debug..
            if cfg!(debug_assertions) { log_obj("Vessel saved", &self.vessel) }
            let save_res = futures::executor::block_on(self.vessel.clone().save());
            if save_res.is_err() {
                log::error!("load err");
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

impl Vase {
    fn cube_vm_vec(cubes: Vec<Cube>, vessel: &Vessel, link: ComponentLink<Vase>) -> Vec<CubeVM> {
        cubes.iter().enumerate()
            .map(|(idx,cube)| CubeVM::new(
                idx, 
                cube, 
                vessel, 
                link.clone()
            )).collect()
    }
}
