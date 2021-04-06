use yew::{Component, ComponentLink, Html, ShouldRender};
use flow_vessel::*;

mod update;
mod view;
mod cube_vm;

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
    SettingUpdate {
        settings: Settings
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
    },
    EntityUp {
        id: EntityId,
    },
    EntityDown {
        id: EntityId,
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
        /* */
        let vessel = _vessel_poet();
        // let vessel = _vessel_incr(10);
        // */
        let cubes = vessel.get_cube_vec();
        let cube_vm_vec = Self::cube_vm_vec(cubes, &vessel, link.clone());
        if cfg!(debug_assertions) { log::debug!("{}", vessel.concise_debug_string()) }
        Self {
            vessel,
            cube_vm_vec,
            link
        }
    }

    fn update(&mut self, msg_queue: Self::Message) -> ShouldRender {
        if msg_queue.len() == 0 { return false }
        if cfg!(debug_assertions) { log::debug!("Update: {:#?}", msg_queue); }
        let mut msg_visitor = msg_queue.into_iter();
        while {
            let next = msg_visitor.next();
            if let Some(msg) = next {
                // update msg here
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
            if cfg!(debug_assertions) { log::debug!("{}", self.vessel.concise_debug_string()) }
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
            .map(|(idx, cube)| CubeVM::new(
                idx, 
                cube.clone(), 
                vessel, 
                link.clone()
            )).collect()
    }
}

fn _vessel_poet() -> Vessel {
    let mut v = Vessel::default();
    let id: Vec<EntityId> = (0..8).map(|_|{
        v.entity_grow()
    }).collect();
    v.entity_get_mut(&id[0]).map(|x| x.face = "The Ripple of Your Shadow".into());
    v.entity_get_mut(&id[1]).map(|x| x.face = "Hi Player, ".into());
    v.entity_get_mut(&id[2]).map(|x| x.face = "it has been a while. ".into());
    v.entity_get_mut(&id[3]).map(|x| x.face = "I've been watching you, all along the way. ".into());
    v.entity_get_mut(&id[4]).map(|x| x.face = "You seem lost. ".into());
    v.entity_get_mut(&id[5]).map(|x| x.face = "Nothing but a long, long dream. ".into());
    v.entity_get_mut(&id[6]).map(|x| {
        x.face = "Wake up now. ".into();
        x.bubble = 
"Your life is waiting. 
Do what you have to do. 
Be a king. ".into();
    });
    v.entity_get_mut(&id[7]).map(|x| x.symbol = Symbol::ProcessTracker(Process::New) );
    v.entity_get_mut(&id[7]).map(|x| x.face = "Do your job.".into() );
    v.entity_devote_push(id[1], id[0]);
    v.entity_devote_push(id[2], id[0]);
    v.entity_devote_push(id[3], id[0]);
    v.entity_devote_push(id[4], id[0]);
    v.entity_devote_push(id[5], id[0]);
    v.entity_devote_push(id[6], id[0]);
    v.entity_devote_push(id[7], id[6]);
    let router = Router::Board;
    {
        let cube: Cube = 
            cubes::ClauseTree {
                obj: id[0],
                current: None,
            }.into();
        // let cube = Cube::new(router);
        let cube_type = cube.cube_type;
        v.glass.replace_cube(
            cube, 
            CubeMeta { router, idx: 0 , cube_type }
        );
    }
    {
        let cube: Cube = 
            cubes::Inkblot {
                obj: id[5],
            }.into();
        // let cube = Cube::new(router);
        let cube_type = cube.cube_type;
        v.glass.insert_cube(
            cube, 
            CubeMeta { router, idx: 1 , cube_type }
        );
    }
    v

}

fn _vessel_incr(num: usize) -> Vessel {
    let mut v = Vessel::default();
    let id: Vec<EntityId> = (0..num).map(|_|{
        v.entity_grow()
    }).collect();
    for (i, x )in id.iter().enumerate() {
        v.entity_get_mut(&x).map(|x| {
            x.face = ((65 + i) as u8 as char).into();
            x.symbol = Symbol::Linted(Lint::Greek);
        });
    }
    v
}
