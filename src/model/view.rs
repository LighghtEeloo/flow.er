use crate::model::*;

impl Model {
    pub fn sidebar_tabs() -> Html {
        let tab_meta: Vec<(&str, &str, bool)> = vec! {
            ("static/icons/hexagons.svg", "Workspace", false),
            ("static/icons/branch.svg", "Projects", false),
            ("static/icons/history.svg", "History", false),
            ("static/icons/settings.svg", "Settings", true),
        };
        let sidebar_tabs: Html = 
            tab_meta.into_iter().map(
                |(src, describe, bottom)| {
                    html! {
                        <li class={if !bottom {"tab"} else {"tab tab-bottom"}}>
                            <div class="tab-content">
                                <img src={src} alt={describe}/>
                                <span class="tooltip">{describe}</span>
                            </div>
                        </li>
                    }
                }
            ).collect();
        html! {
            { sidebar_tabs }
        }
    }

    pub fn main_editor(&self) -> Html {
        let view_new = html! {
            <div class="cube-new">
            { self.cube_new_input_view() }
            </div>
        };
        let view_main = self.cube_view();

        // Test: cube - new?
        if self.cube.empty() && self.cube.name.is_empty() { view_new } else { view_main }
    }

}
