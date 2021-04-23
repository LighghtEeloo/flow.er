use flow_arena::{Direction, FlowError};

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
        dude: Option<EntityId>,
        owner: EntityId,
        idx: usize
    },
    EntityUpdate {
        id: EntityId,
        field: EntityField
    },
    EntityDelete {
        id: EntityId
    },
    EntityMigrate {
        id: EntityId,
        dir: Direction
    },
}

/// Echo implies the side-effect after the Tube update.
pub enum Echo {
    RebuildVM,
    RebuildRef,
    FlowError (FlowError),
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
                let cube = self.glass.add_cube(cube);
                self.glass.place_cube(cube.clone(), meta).ok();
                Echo::RebuildVM
            }
            CloseVM { meta } => {
                self.glass.remove_cube(meta).ok();
                Echo::RebuildVM
            }

            EntityAdd { dude, owner, idx } => {
                dude.map_or(
                    self.entity_grow_devote(owner, idx),
                    |dude| self.entity_add(dude, owner, idx)
                )
                .err()
                .map_or(Echo::RebuildRef, |e| Echo::FlowError (e))
            }
            EntityUpdate { id, field } => {
                self.entity_mut(&id).map(|entity| {
                    entity.update_entity(field)
                });
                Echo::RebuildRef
            }
            EntityDelete { id } => {
                self.entity_remove(&id).err()
                    .map_or(Echo::RebuildRef, |e| Echo::FlowError (e))
            }
            EntityMigrate { id, dir } => {
                self.entity_migrate(&id, dir).err()
                    .map_or(Echo::RebuildRef, |e| Echo::FlowError (e))
            }
        }
    }
}
