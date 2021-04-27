use flow_arena::{Direction, FlowError};

use super::{Cube, CubeMeta, EntityField, EntityId, Router, Settings, Vessel};

/// Tube is the message operating vessel, similar to EntityField
#[derive(Debug, Clone)]
pub enum Tube {
    // router level
    SwitchRouter {
        router: Router,
    },
    SettingUpdate {
        settings: Settings,
    },

    // vm level
    OpenVM {
        cube: Cube,
        meta: CubeMeta,
    },
    CloseVM {
        meta: CubeMeta,
    },

    // entity level
    EntityAdd {
        dude: Option<EntityId>,
        owner: EntityId,
        idx: usize,
    },
    EntityUpdate {
        id: EntityId,
        field: EntityField,
    },
    EntityDelete {
        id: EntityId,
    },
    EntityMigrate {
        id: EntityId,
        dir: Direction,
    },
    // detailed
    EntityGrow,
    EntityLink {
        obj: EntityId,
        owner: EntityId,
        nth: usize,
    },
    EntityDevote {
        obj: EntityId,
        owner: EntityId,
        nth: usize,
    },
    EntityDecay {
        obj: EntityId,
    },
    EntityErase {
        obj: EntityId,
    },
}

/// Echo implies the side-effect after the Tube update.
pub enum Echo {
    RebuildVM,
    RebuildRef,
    FlowError(FlowError),
    SendObj(EntityId),
    Standby,
}

impl Vessel {
    pub fn update_tube(&mut self, tube: Tube) -> Echo {
        use Tube::*;
        match tube {
            SwitchRouter { router } => {
                self.glass.router = router;
                Echo::RebuildVM
            }
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

            EntityAdd { dude, owner, idx } => dude
                .map_or(self.entity_grow_devote(owner, idx), |dude| {
                    self.entity_add(dude, owner, idx)
                })
                .err()
                .map_or(Echo::RebuildRef, |e| Echo::FlowError(e)),
            EntityUpdate { id, field } => {
                self.entity_mut(&id)
                    .map(|entity| entity.update_entity(field));
                Echo::RebuildRef
            }
            EntityDelete { id } => self
                .entity_remove(id)
                .map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef),
            EntityMigrate { id, dir } => self
                .entity_migrate(&id, dir)
                .map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef),
            EntityGrow => {
                let obj = self.entity_grow();
                obj.map_or_else(|e| Echo::FlowError(e), |id| Echo::SendObj(id))
            }
            EntityLink { obj, owner, nth } => {
                let obj = self.entity_link(obj, owner, nth);
                obj.map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef)
            }
            EntityDevote { obj, owner, nth } => {
                let obj = self.entity_devote(obj, owner, nth);
                obj.map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef)
            }
            EntityDecay { obj } => {
                let obj = self.entity_decay(obj);
                obj.map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef)
            }
            EntityErase { obj } => {
                let obj = self.entity_erase(obj);
                obj.map_or_else(|e| Echo::FlowError(e), |_| Echo::RebuildRef)
            }
        }
    }
}
