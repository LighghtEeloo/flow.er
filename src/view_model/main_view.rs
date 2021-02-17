use crate::view_model::*;

impl Model {
    pub fn main_view(&self) -> Html {
        html! {
            <div class="app-wrapper"
                onkeydown=self.link.callback(move |_e: KeyboardEvent| Message::_Debug(format!("Global!")))
            >
                { self.sidebar_routers() }
                { self.main_editor() }
                { self.status_bar() }
                <script src="static/third/assassin.js"/>
            </div>
        }
    }

    fn sidebar_routers(&self) -> Html {
        use Router::*;
        let router_meta: Vec<(&str, Router, &str, bool)> = vec! {
            ("static/icons/hexagons.svg", Cube, "Cube", false),
            ("static/icons/branch.svg", Branch, "Branch", false),
            ("static/icons/history.svg", History, "History", false),
            ("static/icons/settings.svg", Settings, "Settings", true),
        };
        let sidebar_routers: Html = 
            router_meta.into_iter().map(
                |(src, router, describe, bottom)| {
                    html! {
                        <div class={if !bottom {"router"} else {"router router-bottom"}}>
                            <div class="router-content"
                                onclick=self.link.callback(move |_| {
                                    Globaly!(GlobalMessage::SwitchRouter(router))
                                })
                            >
                                <img src={src} alt={describe}/>
                                <span class="tooltip">{describe}</span>
                            </div>
                        </div>
                    }
                }
            ).collect();
        html! {
            <div class="frame" id="left-sidebar">
                { sidebar_routers }
            </div>
        }
    }

    fn main_editor(&self) -> Html {
        // Todo: Router.
        let editor = 
            match self.router {
                Router::Cube => {
                    let cube_model = &self.cube_model;
                    if cube_model.src_view { 
                        cube_model.src_view()
                    } else if cube_model.cube.is_empty() { 
                        cube_model.cube_new_view() 
                    } else { 
                        cube_model.cube_view() 
                    }
                }
                Router::Branch => {
                    let branch_model = &self.branch_model;
                    if branch_model.src_view { 
                        branch_model.src_view()
                    } else { 
                        branch_model.branch_view() 
                    }
                }
                _ => html! {}
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
                view_status=self.src_view_status()
                onclick=self.link.callback(move |_| {
                    Message::Global(vec![GlobalMessage::SrcHit])
                })
            >
                <img src="static/icons/StatusBar/src-code.svg" alt="code_pic"/>
                <span>{"  Source Code  "}</span>
            </button>
        }
    }
    pub fn export_button_view(&self) -> Html {
        html! {
            <button class="status-bar-button" id="export-button"
                title="Copy src to clipboard."
                data-clipboard-text={ export_json(&self.cube_model.cube) }
            >
                <img src="static/icons/StatusBar/code-download.svg" alt="code_pic"/>
                <span>{"  To Clipboard  "}</span>
            </button>
        }
    }
}
