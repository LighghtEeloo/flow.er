use yew::{ComponentLink, Html, html};
use std::time::SystemTime;
use flow_vessel::{Cube, EntityId, Router, Vessel};

pub use super::{Vase, Msg};

mod todo;

pub struct CubeVM {
    meta: CubeMeta,
    view: CubeView
}

impl CubeVM {
    pub fn new(idx: usize, cube: &Cube, vessel: &Vessel, link: ComponentLink<Vase>) -> Self {
        let meta = 
            CubeMeta {
                origin: cube.clone(),
                router: vessel.router,
                idx
            };
        Self {
            meta, 
            view: CubeView::new(cube, vessel, link)
        }
    }
    pub fn view(&self, vessel: &Vessel) -> Html {
        self.view.view(vessel, self.meta.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CubeMeta {
    origin: Cube,
    router: Router,
    idx: usize
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
    pub fn view(&self, vessel: &Vessel, meta: CubeMeta) -> Html {
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

