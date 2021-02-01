#[allow(dead_code)]
#[allow(unused)]

use crate::prelude::*;

use yew::format::Json;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::web_sys::console;
use yew::{classes, html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
use yew::{events::KeyboardEvent, Classes};
use yew_services::storage::{Area, StorageService};

const KEY: &str = "yew.life.tracer.self";

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
}

pub struct Model {
    link: ComponentLink<Self>,
    cube: Cube,
    focus_ref: NodeRef,
    storage: StorageService,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let cube = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Cube::new()
            }
        };
        let focus_ref = NodeRef::default();
        Self {
            link,
            cube,
            focus_ref,
            storage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => {}
        }
        // Test..
        // let mut entry = Entry::new();
        // let id = entry.id();
        // entry.face().push_str(&*format!("Yeh. {}", id));
        // self.cube.entries.insert(id, entry);
        self.storage.store(KEY, Json(&self.cube));
        console::log_1(&"Just dumped.".into());
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let list = vec!("a", "v");
        html! {
            <div class="app-wrapper">
                <div class="frame" id="left-sidebar">
                    <li class="tab"> <div class="tab-content">
                        <img src="static/icons/hexagons.svg" alt="Workspace"/>
                        <span class="tooltip">{"Workspace"}</span>
                    </div> </li>
                    <li class="tab"> <div class="tab-content">
                        <img src="static/icons/branch.svg" alt="Projects"/>
                        <span class="tooltip">{"Projects"}</span>
                    </div> </li>
                    <li class="tab"> <div class="tab-content">
                        <img src="static/icons/history.svg" alt="History"/>
                        <span class="tooltip">{"History"}</span>
                    </div> </li>
                    <li class="tab tab-bottom"> <div class="tab-content">
                        <img src="static/icons/settings.svg" alt="Settings"/>
                        <span class="tooltip">{"Settings"}</span>
                    </div> </li>
                </div>
                <div class="frame" id="main-editor">
                    <p>{"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Earum et voluptates atque, neque sint iste possimus at rerum accusantium quidem. Quia laborum vitae ex sed alias quisquam quibusdam at cupiditate."}</p>
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
        }
    }
}