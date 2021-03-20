use yew::{html, Html};
use flow_vessel::{export_json, Router};
use super::{Vase, Msg::*};

impl Vase {
    pub fn main_view(&self) -> Html {
        html! {
            <div class="app-wrapper">
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
            ("static/icons/branch.svg", Flow, "Flow", false),
            ("static/icons/calendar.svg", Calendar, "Calendar", false),
            ("static/icons/history.svg", TimeCapsule, "TimeCapsule", false),
            ("static/icons/settings.svg", Settings, "Settings", true),
        };
        let sidebar_routers: Html = 
            router_meta.into_iter().map(
                |(src, router, describe, bottom)| {
                    html! {
                        <div class={if !bottom {"router"} else {"router router-bottom"}}>
                            <div class="router-content"
                                onclick=self.link.callback(move |_| {
                                    [SwitchRouter(router)]
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
        let router = self.vessel.router;
        let editor_str = router.type_str();
        // let vm_vec = self.vessel.vm_map.get(&router).map(|vec| {
        //     let per_width = 100.0 / vec.len() as f64;
        //     vec.iter().enumerate().map(|(vm_idx, vm)| {
        //         let style = {
        //             format!("width: {}%;", per_width) 
        //             +&format!("left: {}%;", per_width * vm_idx as f64) 
        //             +&{ if vm_idx != 0 { format!("border-left: 2px solid gray;") } else { format!("") } }
        //         };
        //         html! {
        //             <div class="vm" style={ style }>
        //             // vm cube view
        //             </div>
        //         }
        //     }).collect()
        // }).unwrap_or(Vec::new());
        let editor = 
            html! {
                <div class={editor_str}>
                    // { vm_vec }
                </div>
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
                    text-align: center;
                    font-size: 8pt;
                ">
                    {"Lorem ipsum dolor sit amet consectetur, adipisicing elit."}
                </p>
            </div>
        }
    }
}


impl Vase {
    pub fn src_view_button_view(&self) -> Html {
        html! {
            <button class="status-bar-button" id="src-button"
                title="The source code of the cube."
                // view_status=self.src_view_status()
                // onclick=self.link.callback(move |_| {
                //     Message::Global(vec![GlobalMessage::SrcHit])
                // })
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
                data-clipboard-text={ export_json(&self.vessel) }
            >
                <img src="static/icons/StatusBar/code-download.svg" alt="code_pic"/>
                <span>{"  To Clipboard  "}</span>
            </button>
        }
    }
}

