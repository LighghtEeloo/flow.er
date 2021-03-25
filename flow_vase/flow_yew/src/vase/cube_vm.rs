use yew::{ComponentLink, Html, html};
use std::time::SystemTime;
use flow_vessel::{Cube, CubeMeta, EntityId, Vessel};

pub use super::{Vase, Msg, Msg::*};

mod clause_tree;

pub struct CubeVM {
    pub meta: CubeMeta,
    pub view: CubeView,
    pub link: ComponentLink<Vase>
}

impl CubeVM {
    pub fn new(idx: usize, cube: &Cube, vessel: &Vessel, link: ComponentLink<Vase>) -> Self {
        let meta = 
            CubeMeta {
                // origin: cube.clone(),
                router: vessel.router,
                idx
            };
        let view = CubeView::new(cube, vessel, link.clone());
        Self {
            meta, 
            view,
            link
        }
    }
    pub fn update(&mut self, cube: &Cube) {
        
    }
    pub fn view(&self, vessel: &Vessel, per_width: f64) -> Html {
        let meta = self.meta;
        let idx = meta.idx;
        let style = {
            format!("position: absolute; height: 100%;") 
            +&format!("width: {}%;", per_width) 
            +&format!("left: {}%;", per_width * idx as f64) 
            +&{ if idx != 0 { format!("border-left: 2px solid gray;") } else { format!("") } }
        };
        let btn_close: Html = html! {
            <button class="btn-close"
                onclick=self.link.callback(move |_| {
                    // Todo: close vm.
                    [ CloseVM { meta } ]
                })
            > { "x" } </button>
        };
        html! {
            <div class="vm" style={ style }>
                { btn_close }
                // cube_vm view
                { self.view_inner(vessel) }
            </div>
        }
    }
    pub fn view_inner(&self, vessel: &Vessel) -> Html {
        self.view.view(vessel)
    }
}


pub enum CubeView {
    /// A single entity's notebook.
    Inkblot {
        obj: EntityId
    },
    /// A single entity and a todo list view
    ClauseTree {
        clause: clause_tree::ClauseTree
    },
    PromisedLand,
    FlowView {
        obj: EntityId,
        current: EntityId
    },
    CalendarView {
        current: SystemTime
    },
    TimeView,
    SettingView,
    Blank {
        alt: String
    }
}

impl Default for CubeView {
    fn default() -> Self {
        Self::Blank {
            alt: format!("Clean and innocent.")
        }
    }
}

impl CubeView {
    pub fn new(cube: &Cube, vessel: &Vessel, link: ComponentLink<Vase>) -> Self {
        use CubeView::*;
        match cube.clone() {
            Cube::Inkblot { obj } => 
                Inkblot { obj },
            Cube::ClauseTree { obj, current } => {
                let node = vessel.node(&obj);
                node.map(|node| {
                    clause_tree::ClauseTree::new_cube(
                        node, 
                        current, 
                        link
                    )
                }).unwrap_or_default()
            }
            Cube::PromisedLand => PromisedLand,
            Cube::FlowView { obj, current } => {
                FlowView { obj, current }
            }
            Cube::CalendarView { current } => {
                CalendarView { current }
            }
            Cube::TimeView => TimeView,
            Cube::SettingView => SettingView,
            Cube::Blank { alt } => Blank { alt }
        }
    }
    pub fn view(&self, vessel: &Vessel) -> Html {
        match self {
            CubeView::Blank { alt } => {
                html_uni_vec(format!("blank"), html! {
                    <span> {alt} </span>
                })
            }
            CubeView::ClauseTree { clause } => {
                html_uni_vec(format!("clause-tree"), html! {
                    <span> { clause.view(vessel) } </span>
                })
            }
            _ => {
                let debug = format!("{:#?}", vessel);
                html_uni_vec(format!("none"), html! {
                    <>
                        <>{"Not implemented."}</>
                        <pre style="
                            position: absolute;
                            height: 90%; 
                            top: 0;
                            bottom: 0;
                            width: 60%;
                            right: 5%;
                            overflow: scroll;
                            margin: auto 0;
                            padding: 12px;
                            border: solid 6px green;
                            border-radius: 10px;
                            // font-family: 
                        ">
                            {&*debug}
                        </pre>
                    </>
                })
            }
        }
    }
}

fn html_uni_vec(class: String, content: Html) -> Html {
    html! {
        <div class=class> {content} </div>
    }
}

