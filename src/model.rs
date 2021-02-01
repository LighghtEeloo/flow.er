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
        // Note: Only self.cube is saved.
        self.storage.store(KEY, Json(&self.cube));
        console::log_1(&"Just dumped.".into());
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
            <div class="new-cube">

            </div>
        };
        let view_main = html! {
            <div class="cube">
            </div>
        };

        if self.cube.empty() { view_new } else { view_main }
    }
}
