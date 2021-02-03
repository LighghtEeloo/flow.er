#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused)]

use crate::prelude::*;
use crate::cube::*;

use yew::format::Json;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
use yew::{events::KeyboardEvent};
use yew_services::storage::{Area, StorageService};

const KEY: &str = "yew.life.tracer.self";

#[derive(Debug)]
pub enum Msg {
    NewCube,
    AddNode,
    UpdateBuffer(String),
    WriteBuffer(EntryId),
    Focus,
}

pub struct Model {
    cube: Cube,
    buffer_str: String,
    focus_ref: NodeRef,
    storage: StorageService,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let mut cube: Cube = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Cube::new()
            }
        };

        // Debug..
        cube = Cube::new();
        if cube.entries.len() == 0 {
            let entry = Entry::new();
            cube.entries.insert(entry.id(), entry.clone());
            cube.relation = RelationModel::Linear(vec!(entry.id()));
        }
        LOG!("Loaded: {:?}", cube);
        let focus_ref = NodeRef::default();
        Self {
            cube,
            buffer_str: String::new(),
            focus_ref,
            storage,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // Debug..
        LOG!("Updating with: {:?}", msg);
        use Msg::*;
        match msg {
            UpdateBuffer(val) => {
                self.buffer_str = val;
            }
            NewCube => {
                // Todo: Add new cube.
                self.cube.name = self.buffer_str.clone();
                self.buffer_str.clear();
            }
            WriteBuffer(id) => {
                let x: &Entry = self.cube.entries.get(&id).unwrap();
                x.dry().face = mem::take(&mut self.buffer_str);
            }
            Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
            _ => {}
        }
        // Note: Only self.cube is saved.
        self.storage.store(KEY, Json(&self.cube));
        LOG!("Just dumped.");
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view = html! {
            <div class="app-wrapper">
                <div class="frame" id="left-sidebar">
                    { Model::sidebar_tabs() }
                </div>
                <div class="frame" id="main-editor">
                    { self.main_editor() }
                </div>
                <div class="frame" id="status-bar">
                    <p>{"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Earum et voluptates atque, neque sint iste possimus at rerum accusantium quidem. Quia laborum vitae ex sed alias quisquam quibusdam at cupiditate."}</p>
                </div>
                <footer class="info" style="display: none">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/LighghtEeloo/" target="_blank">{ "LighghtEeloo" }</a></p>
                    <p>{ "Inspired by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                </footer>
            </div>
        };
        view
    }
}



impl Model {
    fn sidebar_tabs() -> Html {
        let tab_meta: Vec<(&str, &str, bool)> = vec! {
            ("static/icons/hexagons.svg", "Workspace", false),
            ("static/icons/branch.svg", "Projects", false),
            ("static/icons/history.svg", "History", false),
            ("static/icons/settings.svg", "Settings", true),
        };
        let sidebar_tabs: Html = 
            tab_meta.iter().map(
                |(src, describe, bottom)| {
                    html! {
                        <li class={if !bottom {"tab"} else {"tab tab-bottom"}}>
                            <div class="tab-content">
                                <img src={&*src} alt={&*describe}/>
                                <span class="tooltip">{&*describe}</span>
                            </div>
                        </li>
                    }
                }
            ).collect();
        html! {
            {sidebar_tabs}
        }
    }

    fn main_editor(&self) -> Html {
        let view_new = html! {
            <div class="cube-new">
                <input
                    type="text"
                    placeholder="Enter new proj name."
                    oninput=self.link.callback(|e: InputData| {
                        LOG!("Oninput - new: {:?}", e);
                        Msg::UpdateBuffer(e.value)
                    })
                    onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                        LOG!("Onkeypress - new: {:?}", e);
                        if e.key() == "Enter" { Some(Msg::NewCube) } else { None }
                    })
                />
                <div class="dash-line"></div>
            </div>
        };
        let view_main = self.cube_view();

        // Debug: cube - new?
        if self.cube.empty() { view_new } else { view_main }
    }

    fn cube_view(&self) -> Html {
        use RelationModel::*;
        let relation = &self.cube.relation;
        match relation {
            Linear(vec) => {
                html! {
                    <div class="cube">
                        { for vec.iter().map(|id| self.node_view(id))  }
                    </div>
                }
            }
            _ => html! {}
        }
    }
    
    fn node_view(&self, id: &EntryId) -> Html {
        let id = id.clone();
        html! {
            <div class="node">
                <input
                    type="text"
                    value=self.cube.get(id).face()
                    placeholder="..."
                    oninput=self.link.callback(|e: InputData| {
                        LOG!("Oninput: {:?}", e);
                        Msg::UpdateBuffer(e.value)
                    })
                    onblur=self.link.callback(move |_| Msg::WriteBuffer(id))
                    onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                        LOG!("Onkeypress: {:?}", e);
                        if e.key() == "Enter" { Some(Msg::WriteBuffer(id)) } else { None }
                    })
                />
            </div>
        }
    }
}
