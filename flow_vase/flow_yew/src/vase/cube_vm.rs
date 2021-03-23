use yew::{ComponentLink, Html, html};
use std::time::SystemTime;
use flow_vessel::{Cube, EntityId, Router, Vessel};

pub use super::{Vase, Msg, Msg::*};

mod todo;

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

#[derive(Debug, Clone, Copy)]
pub struct CubeMeta {
    // origin: Cube,
    pub router: Router,
    pub idx: usize
}

pub enum CubeView {
    /// A single entity's notebook.
    Inkblot {
        obj: EntityId
    },
    /// A single entity and its points.
    PointView {
        obj: EntityId,
        current: Option<usize>
    },
    /// A single entity and a todo list view
    TodoList {
        current: Option<usize>,
        todo: todo::TodoList
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
            Cube::PointView { obj, current } => {
                PointView { obj, current }
            }
            Cube::TodoList { obj, current } => {
                let node = vessel.node(&obj);
                node.map(|node| {
                    TodoList { 
                        current, 
                        todo: todo::TodoList::new(
                            node, 
                            link
                        )
                    }
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
            CubeView::TodoList { current: _, todo } => {
                html_uni_vec(format!("todo-list"), html! {
                    <span> { todo.view(vessel) } </span>
                })
            }
            _ => {
                html_uni_vec(format!("none"), html! {
                    <>{"Not implemented."}</>
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

