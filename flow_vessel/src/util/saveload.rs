use crate::{Vessel, Bridge};

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    WebError,
    FormatError,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    FileError,
    WebError,
    WriteError,
    FormatError,
}

#[cfg(not(target_arch = "wasm32"))]
impl Vessel {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "LighghtEeloo", "flow.er")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("flow_data.json");

        path
    }

    pub async fn load() -> Result<Vessel, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        let vessel: Vessel = serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)?;

        if let Bridge::Linked{..} = vessel.settings.bridge {
            // Todo: link and save.
        }

        Ok(vessel)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::FileError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        if let Bridge::Linked{..} = self.settings.bridge {
            // Todo: link and save.
        }

        // This is a simple way to save at most once every couple seconds
        // Todo: But it's annoying >_<.
        // async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl Vessel {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;

        window.local_storage().ok()?
    }

    async fn load_local() -> Result<Vessel, LoadError> {
        log::debug!("loading...");
        let storage = Self::storage().ok_or(LoadError::FileError)?;

        let contents = storage
            .get_item("flow.er.vessel")
            .map_err(|_| LoadError::FileError)?
            .ok_or(LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    } 

    async fn load_linked(addr: String, port: u16) -> Result<Vessel, LoadError> {
        log::debug!("loading linked: {}:{}", addr, port);
        Err(LoadError::WebError)
    }

    pub async fn load() -> Result<Vessel, LoadError> {
        let vessel: Vessel = Self::load_local().await.or_else(|load_error| {
            if let LoadError::FileError = load_error {
                Ok(Vessel::default())
            } else { Err(load_error) }
        })?;

        let bridge = vessel.settings.bridge.clone();

        let vessel = if let Bridge::Linked { addr, port } = bridge {
            Self::load_linked(addr, port).await.or_else(|load_error|
                if let LoadError::WebError = load_error {
                    Ok(vessel)
                } else {
                    Err(load_error)
                }
            )?
        } else { vessel };

        Ok(vessel)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        log::debug!("saving...");
        let storage = Self::storage().ok_or(SaveError::FileError)?;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

        storage
            .set_item("flow.er.vessel", &json)
            .map_err(|_| SaveError::WriteError)?;

        // let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn saveload() -> Result<(), &'static str> {
        let f = Vessel::load();
        let vessel = futures::executor::block_on(f).map_err(|_| "load err").unwrap_or(Vessel::new());
        println!("{:#?}", vessel);

        // let vessel = Vessel::new();
        {
            let mut vessel = vessel.clone();
            {
                let obj = vessel.entity_grow().map_err(|_| "grow err")?;
                let flow_view = Cube::new(CubeType::FlowView).with_obj(obj);
                let flow_view = vessel.glass.add_cube(flow_view);
                let meta = CubeMeta { router: Router::Workspace, idx: 0 };
                vessel.glass.place_cube(flow_view, meta).expect("place_cube failed");
            }
            {
                let promised_land = Cube::new(CubeType::PromisedLand);
                let promised_land = vessel.glass.add_cube(promised_land);
                let meta = CubeMeta { router: Router::Workspace, idx: 1 };
                vessel.glass.place_cube(promised_land, meta).expect("place_cube failed");
            }
            {
                let node_view = Cube::new(CubeType::NodeView);
                let node_view = vessel.glass.add_cube(node_view);
                let meta = CubeMeta { router: Router::Workspace, idx: 2 };
                vessel.glass.place_cube(node_view, meta).expect("place_cube failed");
            }
            println!("{:#?}", vessel);
            let f = vessel.save();
            futures::executor::block_on(f).map_err(|_| "save err")?;
        }
        
        // remain unchanged
        let f = vessel.save();
        futures::executor::block_on(f).map_err(|_| "save err")?;
        Ok(())
    }
}
