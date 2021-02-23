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
    router: Router,
    vessel: Vessel,
    storage: StorageService,
    link: ComponentLink<Self>,
}

const KEY: &str = "flow.er.data";

#[derive(Debug, Clone)]
pub enum Message {
    
    NoRender
}

impl Component for Vase {
    type Message = Vec<Message>;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let mut vessel: Vessel = {
            let vessel = if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vessel::default()
            };
            // Debug..
            // let mut stockpile = Stockpile::new();
            // stockpile.editor_info = EditorInfo::new_some(CubeId::new());
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
        use Message::*;
        LOG!("Updating: {:#?}.", messages);
        let mut res = true;
        for message in messages {
            res = match message {
                NoRender => false,
                _ => {
                    LOG!("No update pattern matched.");
                    true
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

