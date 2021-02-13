use crate::ui::*;

impl Model {
    pub fn main_view(&self) -> Html {
        html! {
            <div class="app-wrapper">
                <div class="frame" id="left-sidebar">
                    { Model::sidebar_tabs() }
                </div>
                <div class="frame" id="main-editor">
                    { self.main_editor() }
                </div>
                <div class="frame" id="status-bar">
                    <p>{"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Earum et voluptates atque, neque sint iste possimus at rerum accusantium quidem."}</p>
                </div>
            </div>
        }
    }

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
        let view_new = self.cube_new_view();
        let view_main = self.cube_view();
        // Test: cube - new?
        if self.cube.empty() && self.cube.name.is_empty() { view_new } else { view_main }
    }

}
