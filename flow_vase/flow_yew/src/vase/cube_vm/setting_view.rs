use yew::{ComponentLink, Html, NodeRef, html};
use std::collections::HashMap;
use flow_vessel::{CubeMeta, EntityId, Settings, Vessel, cubes::SettingView};
use super::{CubeView, Vase, Msg::*};

impl CubeView for SettingView {
    fn view(&self, vessel: &Vessel, _meta: CubeMeta, link: ComponentLink<Vase>, _ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        let settings = vessel.settings.clone();
        let view_mode = settings.view_mode.clone();
        let view_mode_text = format!("ViewMode: {}", view_mode.display());
        html! {
            <>
                <button
                    class="opterate-btn"
                    onclick=link.callback(move |_| {
                        vec! [SettingUpdate { settings: Settings {
                            view_mode: view_mode.clone().switch(),
                            .. settings.clone()
                        } }]
                    })
                > { view_mode_text } </button>
            </>
        }
    }
}