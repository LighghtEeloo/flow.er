use yew::{ComponentLink, Html, NodeRef, html};
use std::collections::HashMap;
use flow_vessel::{CubeMeta, EntityId, cubes::SettingView};

use super::CubeView;

impl CubeView for SettingView {
    fn view(&self, vessel: &flow_vessel::Vessel, meta: CubeMeta, link: ComponentLink<super::Vase>, ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        html! {
            <></>
        }
    }
}