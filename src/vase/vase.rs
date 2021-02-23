use crate::util::*;
use crate::yew_util::*;
use super::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Router {
    Cube,
    Flow,
    Calendar,
    TimeCapsule,

    Settings,
}

pub struct Vase {
    pub router: Router,
    pub vessel: Vessel,
    pub storage: StorageService,
    pub link: ComponentLink<Self>,
}

const KEY: &str = "flow.er.data";

#[derive(Debug, Clone)]
pub enum VaseMsg {
    SwitchRouter(Router),
    NoRender
}

impl Component for Vase {
    type Message = Vec<VaseMsg>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let mut vessel: Vessel = {
            let vessel = if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vessel::default()
            };
            vessel
        };
        // Note: trim only on startup.
        vessel.trim().err();
        // Test..
        LOG!("Loaded & Trimmed: {:#?}", vessel);

        Self {
            router: Router::Cube,
            vessel,
            storage,
            link,
        }
    }

    fn update(&mut self, messages: Self::Message) -> ShouldRender {
        use VaseMsg::*;
        LOG!("Updating: {:#?}.", messages);
        let mut res = true;
        for message in messages {
            res = match message {
                SwitchRouter(router) => {
                    self.router = router; true
                }
                NoRender => false,
                _ => {
                    LOG!("No update pattern matched."); false
                }
            }
        }
        // Only self.vessel is saved.
        self.storage.store(KEY, Json(&self.vessel));
        res
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.main_view()
    }
}

