use yew::{html, Html};
use flow_vessel::{Router, Tube::*, ViewMode, export_json};
// use flow_vessel::display::*;
use super::{Vase};

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
        let router_meta: Vec<(&str, Router, bool)> = Router::vec_all()
            .into_iter().map(|r| {
                (r.src_str(), r, r == Settings)
            }).collect();
        let sidebar_routers: Html = 
            router_meta.into_iter().map(
                |(src, router, bottom)| {
                    html! {
                        <div class={if !bottom {"router"} else {"router router-bottom"}}>
                            <div class="router-content"
                                onclick=self.link.callback(move |_| {
                                    [SwitchRouter{router}]
                                })
                            >
                                <img src={src} alt={router.display_str()}/>
                                <span class="tooltip">{router.display_str()}</span>
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
        // let vm_vec: Vec<Html> = self.cube_vm_vec.iter()
        //     .map(|cv| cv.view()).collect();
        let vm_vec: Vec<Html> = self.cube_vm_vec_view();
        let editor = 
            html! {
                <div id={editor_str} class="vm-container">
                    { vm_vec }
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
                { self.src_button() }
                { self.export_button() }
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
                // { self.status_bar_clock() }
            </div>
        }
    }
}

impl Vase {
    fn cube_vm_vec_view(&self) -> Vec<Html> {
        match self.vessel.settings.view_mode.clone() {
            ViewMode::Desktop => {
                let per_width = 100.0 / self.cube_vm_vec.len() as f64;
                self.cube_vm_vec.iter().map(|cv| {
                    cv.view(&self.vessel, per_width)
                }).collect()
            }
            _ => {
                let last = self.cube_vm_vec.last();
                if let Some(view) =last.map(|cv| cv.view(&self.vessel, 100.0)) {
                    vec! [view]
                } else { Vec::new() }
            }
        }
    }
}


impl Vase {
    fn src_button(&self) -> Html {
        html! {
            <button class="status-bar-button" id="src-button"
                title="The source code of the cube."
                // view_status=self.src_view_status()
                // onclick=self.link.callback(move |_| {
                //     Message::Global(vec![GlobalMessage::SrcHit])
                // })
                // Debug: refresh onclick.
                onclick=self.link.callback(move |_| {
                    []
                })
            >
                <img src="static/icons/StatusBar/src-code.svg" alt="code_pic"/>
                <span>{"  Source Code  "}</span>
            </button>
        }
    }
    fn export_button(&self) -> Html {
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
    // Note: Clock not available yet.
    // fn status_bar_clock(&self) -> Html {
    //     let time = format!("{}", TimeClockLocal::from(flow_vessel::now()));
    //     html! {
    //         <div class="status-bar-button" id="clock">
    //             <span>{ time }</span>
    //         </div>
    //     }
    // }
}


