use crate::ui::*;

use CubeMessage::*;

impl Model {
    pub fn main_view(&self) -> Html {
        html! {
            <div class="app-wrapper">
                <div class="frame" id="left-sidebar">
                    { self.sidebar_tabs() }
                </div>
                <div class="frame" id="main-editor">
                    { self.main_editor() }
                </div>
                <div class="frame" id="status-bar">
                    { self.src_view_button_view() }
                    <p style="
                        width: 80%;
                        margin: auto;
                        font-family: cursive;
                        text-align: center;
                        font-size: 8pt;
                    ">
                        {"Lorem ipsum dolor sit amet consectetur, adipisicing elit."}
                    </p>
                </div>
            </div>
        }
    }

    pub fn sidebar_tabs(&self) -> Html {
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
        // Fixme: cube - new?
        if self.cube.empty() && self.cube.name.is_empty() { 
            self.cube_new_view() 
        } else if self.src_view { 
            self.cube_src_view()
        } else { 
            self.cube_view() 
        }
    }

}


impl Model {
    pub fn src_view_button_view(&self) -> Html {
        html! {
            <button class="src-button"
                title="The source code of the cube."
                onclick=self.link.callback(move |_| {
                    Cubey![SrcViewToggle(None)]
                })
            >
                <img src="static/icons/StatusBar/src-code.svg" alt="Code_pic"/>
                <span>{"  Source Code  "}</span>
            </button>
        }
    }
}
