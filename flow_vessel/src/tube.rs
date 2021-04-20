use super::{Cube, CubeMeta, EntityField, EntityId, Router, Settings, Vessel};

/// Tube is the message operating vessel, similar to EntityField
#[derive(Debug, Clone)]
pub enum Tube {
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
}

/// Echo implies the side-effect after the Tube update.
pub enum Echo {
    RebuildVM,
    RebuildRef,
    Standby
}

impl Vessel {
    pub fn update_tube(&mut self, tube: Tube) -> Echo {
        use Tube::*;
        match tube {
            SwitchRouter{ router} => {
                self.glass.router = router;
                Echo::RebuildVM
            },
            SettingUpdate { settings } => {
                self.settings = settings;
                Echo::Standby
            }

            OpenVM { cube, meta } => {
                // self.glass.push_cube(cube.clone(), meta.router).ok();
                Echo::RebuildVM
            }
            CloseVM { meta } => {
                // self.glass.remove_cube(meta).ok();
                Echo::RebuildVM
            }

            EntityAdd { dude, owner, idx } => {
                self.entity_add(dude, owner, idx);
                Echo::RebuildRef
            }
            EntityUpdate { id, field } => {
                self.entity_mut(&id).map(|entity| {
                    entity.update_entity(field)
                });
                Echo::RebuildRef
            }
            EntityDive { id, idx } => {
                self.entity_dive(id, idx);
                Echo::RebuildRef
            }
            EntityEmerge { id } => {
                self.entity_emerge(id);
                Echo::RebuildRef
            }
            EntityDelete { id } => {
                self.entity_remove(&id);
                Echo::RebuildRef
            }
            EntityUp { id } => {
                self.entity_up(id);
                Echo::RebuildRef
            }
            EntityDown { id } => {
                self.entity_down(id);
                Echo::RebuildRef
            }

        }
    }
}
