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

            EntityAdd { owner, idx } => {
                self.vessel.entity_grow_devote(owner, idx);
                true
            }
            EntityUpdate { id, field } => {
                self.vessel.entity_get_mut(&id).map(|entity| {
                    entity.update_entity(field)
                });
                true
            }
            EntityDelete { id } => {
                self.vessel.entity_decay(&id);
                true
            }

            Refresh => false,
        }
    }
}
