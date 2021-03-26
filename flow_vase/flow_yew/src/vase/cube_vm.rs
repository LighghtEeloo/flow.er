use yew::{ComponentLink, Html, html};
use std::time::SystemTime;
use flow_vessel::{Cube, CubeMeta, CubeType, EntityId, Vessel};

pub use super::{Vase, Msg, Msg::*};

mod inkblot;
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
                idx,
                cube_type: cube.cube_type
            };
        let view = CubeView::new(cube, vessel, link.clone());
        Self {
            meta, 
            view,
            link
        }
    }
    pub fn update(&mut self, idx: usize, cube: &Cube, vessel: &Vessel) {
        self.meta.idx = idx;
        self.view = self.view.clone().update_new(cube, vessel);
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
            > { "✕" } </button>
        };
        html! {
            <div class="vm" style={ style }>
                // cube_vm view
                { self.view_inner(vessel) }
                { if idx != 0 { btn_close } else { html!{} } }
            </div>
        }
    }
    pub fn view_inner(&self, vessel: &Vessel) -> Html {
        self.view.view(vessel)
    }
}


#[derive(Clone)]
pub enum CubeView {
    /// A single entity's notebook.
    Inkblot {
        inkblot: inkblot::Inkblot
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
    },
    _Plain
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
        let cube = cube.clone();
        match cube.cube_type {
            CubeType::Inkblot => {
                inkblot::Inkblot::new_cube(cube, link)
            }
            CubeType::ClauseTree => {
                let node = vessel.node(&cube.obj.unwrap_or_default());
                node.map(|node| {
                    clause_tree::ClauseTree::new_cube(
                        node, 
                        cube.current_idx, 
                        link
                    )
                }).unwrap_or_default()
            }
            CubeType::PromisedLand => {
                CubeView::_Plain
            }
            CubeType::FlowView => {
                CubeView::_Plain
            }
            CubeType::CalendarView => {
                CubeView::_Plain
            }
            CubeType::TimeView => {
                CubeView::_Plain
            }
            CubeType::SettingView => {
                CubeView::_Plain
            }
            CubeType::Blank => {
                CubeView::_Plain
            }
        }
    }
    pub fn update_new(self, cube: &Cube, vessel: &Vessel) -> Self {
        use CubeView::*;
        // let cube_type = cube.cube_type;
        match self.clone() {
            CubeView::Blank { alt: _ } => {
                Blank { alt: cube.clone().alt.unwrap_or_default() }
            }
            CubeView::Inkblot { inkblot } => {
                inkblot.update_new(cube, vessel)
            }
            CubeView::ClauseTree { clause } => {
                clause.update_new(cube, vessel)
            }
            _ => self
        }
    }
    pub fn view(&self, vessel: &Vessel) -> Html {
        match self {
            CubeView::Blank { alt } => {
                html_uni_vec(format!("blank"), html! {
                    <span> {alt} </span>
                })
            }
            CubeView::Inkblot { inkblot } => {
                html_uni_vec(format!("inkblot"), html! {
                    <span> { inkblot.view(vessel) } </span>
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

