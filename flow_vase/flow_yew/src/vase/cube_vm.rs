use yew::{ComponentLink, Html, NodeRef, html};
use std::{collections::HashMap};
use flow_vessel::{Cube, CubeMember, CubeMeta, EntityId, Vessel};

pub use super::{Vase, Msg, Msg::*};
mod btn;

mod inkblot;
mod clause_tree;
mod flow_view;

mod setting_view;

pub struct CubeVM {
    pub meta: CubeMeta,
    pub cube: Cube,
    pub ref_map: HashMap<EntityId, NodeRef>,
    pub link: ComponentLink<Vase>
}

impl CubeVM {
    pub fn new(idx: usize, cube: Cube, vessel: &Vessel, link: ComponentLink<Vase>) -> Self {
        let meta = 
            CubeMeta {
                // origin: cube.clone(),
                router: vessel.router,
                idx,
                cube_type: cube.cube_type
            };
        // Todo: member_traverse.
        // let ref_map = cube.member_traverse(vessel)
        //     .into_iter().map(|obj| {
        //         (obj, NodeRef::default())
        //     }).collect();
        let ref_map = HashMap::new();
        Self {
            meta, 
            cube,
            ref_map,
            link
        }
    }
    pub fn update(&mut self, idx: usize, cube: &Cube, vessel: &Vessel) {
        self.meta.idx = idx;
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
        self.cube.view(vessel, self.meta)
    }
}


pub trait CubeView {
    fn view(&self, vessel: &Vessel, meta: CubeMeta) -> Html {
        html! {
            <div class="none">
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
                    {&*format!("{:#?}", vessel)}
                </pre>
            </div>
        }
    }
}

impl CubeView for Cube {}

