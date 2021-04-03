use yew::ShouldRender;

use super::{Msg, Vase, CubeVM};

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
                self.vessel.entity_get_mut(&id).map(|entity| {
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

            Refresh => false,
        }
    }
}
