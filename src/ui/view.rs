use crate::ui::*;

use CubeMessage::*;

impl Model {
    pub fn main_view(&self) -> Html {
        html! {
            <div class="app-wrapper">
                { self.sidebar_tabs() }
                { self.main_editor() }
                { self.status_bar() }
                <script src="static/third/clip.js"/>
            </div>
        }
    }

    fn sidebar_tabs(&self) -> Html {
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
            <div class="frame" id="left-sidebar">
                { sidebar_tabs }
            </div>
        }
    }

    fn main_editor(&self) -> Html {
        // Fixme: cube - new?
        let editor = if self.cube.empty() && self.cube.name.is_empty() { 
            self.cube_new_view() 
        } else if self.src_view { 
            self.cube_src_view()
        } else { 
            self.cube_view() 
        };
        html! {
            <div class="frame" id="main-editor">
                { editor }
            </div>
        }
    }

    fn status_bar(&self) -> Html {
        html! {
            <div class="frame" id="status-bar">
                { self.src_view_button_view() }
                { self.export_button_view() }
                <p style="
                    position: absolute;
                    top: 0; bottom: 0; left: 0; right: 0;
                    height: 100%;
                    width: 60%;
                    margin: auto;
                    font-family: cursive;
                    text-align: center;
                    font-size: 8pt;
                ">
                    {"Lorem ipsum dolor sit amet consectetur, adipisicing elit."}
                </p>
            </div>
        }
    }
}


impl Model {
    pub fn src_view_button_view(&self) -> Html {
        html! {
            <button class="status-bar-button" id="src-button"
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
    pub fn export_button_view(&self) -> Html {
        html! {
            <button class="status-bar-button" id="export-button"
                title="Copy src to clipboard."
                data-clipboard-text="Just because you can doesn't mean you should — clipboard.js"
            >
                <img src="static/icons/StatusBar/code-download.svg" alt="Code_pic"/>
                <span>{"  To Clipboard  "}</span>
            </button>
        }
    }
}
