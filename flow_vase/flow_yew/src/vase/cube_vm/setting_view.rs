use yew::{ComponentLink, Html, NodeRef, html};
use std::collections::HashMap;
use flow_vessel::{CubeMeta, EntityId, Settings, Vessel, cubes::SettingView};
use super::{CubeView, Vase, Msg::*};

impl CubeView for SettingView {
    fn view(&self, vessel: &Vessel, meta: CubeMeta, link: ComponentLink<Vase>, ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        let settings = vessel.settings.clone();
        html! {
            <>
                <button
                    class="opterate-btn"
                    onclick=link.callback(move |_| {
                        vec! [SettingUpdate { settings: Settings {
                            view_mode: flow_vessel::ViewMode::Mobile,
                            .. settings.clone()
                        } }]
                    })
                > { "ViewMode" } </button>
                // { format!("{:#?}", vessel.settings.view_mode) }
            </>
        }
    }
}