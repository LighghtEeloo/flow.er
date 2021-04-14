use yew::{ComponentLink, Html, NodeRef, html};
use std::{collections::HashMap};
use flow_vessel::{Cube, CubeMember, CubeMeta, CubeType, EntityId, Tube::*, Vessel, ViewMode, cubes};
use flow_vessel::ClauseTreeCore;

pub use super::{Vase};
mod btn;

// mod inkblot;
mod clause_tree;
use clause_tree::ClauseTree;
// mod clause_tree_alt;
// mod flow_view;
// mod setting_view;

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
            };
        // Todo: member_traverse.
        let ref_map = cube.member_traverse(vessel)
            .into_iter().map(|obj| {
                (obj, NodeRef::default())
            }).collect();
        // let ref_map = HashMap::new();
        Self {
            meta, 
            cube,
            ref_map,
            link
        }
    }
    pub fn update(&mut self, idx: usize, cube: &Cube, vessel: &Vessel) {
        self.meta.idx = idx;
        self.cube = cube.clone();
        // Todo: member_traverse.
        self.ref_map = cube.member_traverse(vessel)
            .into_iter().map(|obj| {
                (obj, NodeRef::default())
            }).collect();
    }
    pub fn view(&self, vessel: &Vessel, per_width: f64) -> Html {
        let desktop = ViewMode::Desktop == vessel.settings.view_mode;
        let meta = self.meta;
        let idx = meta.idx;
        let style = {
            format!("position: absolute; height: 100%;") 
            +&format!("width: {}%;", per_width) 
            +&format!("left: {}%;", per_width * if desktop {idx} else {0} as f64) 
            +&{ if idx != 0 { format!("border-left: 2px solid gray;") } else { format!("") } }
        };
        let btn_close: Html = html! {
            <button class="btn-close"
                onclick=self.link.callback(move |_| {
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
        self.cube.view(vessel, self.meta, self.link.clone(), &self.ref_map)
    }
}


pub trait CubeView {
    fn view(&self, vessel: &Vessel, meta: CubeMeta, link: ComponentLink<Vase>, ref_map: &HashMap<EntityId, NodeRef>) -> Html;
}

impl CubeView for Cube {
    fn view(&self, vessel: &Vessel, meta: CubeMeta, link: ComponentLink<Vase>, ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        let cube_type = self.cube_type;
        let cube_view = match self.cube_type {
            CubeType::Blank => {
                let blank: cubes::Blank = self.clone().into();
                html! {
                    <div> {blank.alt} </div>
                }
            }
            // CubeType::Inkblot => {
            //     let ink: cubes::Inkblot = self.clone().into();
            //     ink.view(vessel, meta, link, ref_map)
            // }
            CubeType::ClauseTree => {
                // let clause: cubes::ClauseTreeCube = self.clone().into();
                let core = ClauseTreeCore::new(self.clone(), &vessel, meta);
            //     clause.view(vessel, meta, link, ref_map)
                html! {
                    <ClauseTree core=core meta=meta link_tube=link />
                }
            }
            // CubeType::FlowView => {
            //     let flow: cubes::FlowView = self.clone().into();
            //     flow.view(vessel, meta, link.clone(), ref_map)
            // }
            // CubeType::SettingView => {
            //     let setting_view: cubes::SettingView = self.clone().into();
            //     setting_view.view(vessel, meta, link, ref_map)
            // }
            _ => {
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
        };
        html! {
            <div class={cube_type.type_str()}>
                { cube_view }
            </div>
        }
    }
}

