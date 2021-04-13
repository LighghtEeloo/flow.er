use yew::{ShouldRender, web_sys::HtmlInputElement as InputElement};
use flow_vessel::{Cube, CubeMeta, EntityField, EntityId, Router, Settings};

use super::{Vase, CubeVM};

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

    // vase level
    Focus {
        meta: CubeMeta,
        id: EntityId
    },
    Refresh,
}

impl Vase {
    pub fn update_msg(&mut self, msg: Msg) -> ShouldRender {
        use Msg::*;
        // returns true if non-block, false if needs revisit or finish
        match msg {
            SwitchRouter{ router} => {
                self.vessel.router = router;
                self.cube_vm_vec = Vec::new();
                true
            },
            SettingUpdate { settings } => {
                self.vessel.settings = settings;
                true
            }

            OpenVM { cube, meta } => {
                self.vessel.glass.push_cube(cube.clone(), meta.router);
                let idx = meta.idx;
                self.cube_vm_vec.insert(idx,CubeVM::new(idx, cube, &self.vessel, self.link.clone()));
                true
            }
            CloseVM { meta } => {
                self.vessel.glass.remove_cube(meta);
                self.cube_vm_vec.remove(meta.idx);
                true
            }

            EntityAdd { dude, owner, idx } => {
                self.vessel.entity_add(dude, owner, idx);
                true
            }
            EntityUpdate { id, field } => {
                self.vessel.entity_mut(&id).map(|entity| {
                    entity.update_entity(field)
                });
                true
            }
            EntityDive { id, idx } => {
                self.vessel.entity_dive(id, idx);
                true
            }
            EntityEmerge { id } => {
                self.vessel.entity_emerge(id);
                true
            }
            EntityDelete { id } => {
                self.vessel.entity_remove(&id);
                true
            }
            EntityUp { id } => {
                self.vessel.entity_up(id);
                true
            }
            EntityDown { id } => {
                self.vessel.entity_down(id);
                true
            }

            Focus { meta, id } => {
                self.cube_vm_vec.get_mut(meta.idx).map(|vm| {
                    vm.cube.current = Some(id);
                    vm.ref_map.get(&id)
                }).flatten().unwrap().cast::<InputElement>().map(|x|
                    x.focus().unwrap()
                );
                true
            }
            Refresh => {
                let cubes = self.vessel.get_cube_vec();
                self.cube_vm_vec = Self::cube_vm_vec(cubes, &self.vessel, self.link.clone());
                false
            }
        }
    }
}
