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
    Rebuild,
    Standby
}

impl Vessel {
    pub fn update_tube(&mut self, tube: Tube) -> Echo {
        use Tube::*;
        match tube {
            SwitchRouter{ router} => {
                self.router = router;
                Echo::Rebuild
            },
            SettingUpdate { settings } => {
                self.settings = settings;
                Echo::Rebuild
            }

            OpenVM { cube, meta } => {
                self.glass.push_cube(cube.clone(), meta.router);
                Echo::Rebuild
            }
            CloseVM { meta } => {
                self.glass.remove_cube(meta);
                Echo::Rebuild
            }

            EntityAdd { dude, owner, idx } => {
                self.entity_add(dude, owner, idx);
                Echo::Rebuild
            }
            EntityUpdate { id, field } => {
                self.entity_mut(&id).map(|entity| {
                    entity.update_entity(field)
                });
                Echo::Rebuild
            }
            EntityDive { id, idx } => {
                self.entity_dive(id, idx);
                Echo::Rebuild
            }
            EntityEmerge { id } => {
                self.entity_emerge(id);
                Echo::Rebuild
            }
            EntityDelete { id } => {
                self.entity_remove(&id);
                Echo::Rebuild
            }
            EntityUp { id } => {
                self.entity_up(id);
                Echo::Rebuild
            }
            EntityDown { id } => {
                self.entity_down(id);
                Echo::Rebuild
            }

        }
    }
}
