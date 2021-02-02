#[allow(dead_code)]
#[allow(unused)]

use crate::prelude::*;

use yew::format::Json;
use yew::web_sys::HtmlInputElement as InputElement;
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
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
            _ => {}
        }
        // Note: Only self.cube is saved.
        self.storage.store(KEY, Json(&self.cube));
        LOG(&"Just dumped.".into());
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
        LOG(&format!("{:?}", String::from("abs")).into());
        let view_new = html! {
            <div class="cube-new">
                <input
                    type="text"
                    placeholder="Enter your proj name."
                    oninput=self.link.callback(|e: InputData| {
                        LOG(&format!("{:?}", e).into());
                        Msg::UpdateEdit(e.value)
                    })
                    // onblur=self.link.callback(move |_| Msg::Edit(idx))
                    // onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                    //     if e.key() == "Enter" { Some(Msg::Edit(idx)) } else { None }
                    // })
                />
            </div>
        };
        let view_main = html! {
            <div class="cube">
                { self.cube_view() }
            </div>
        };

        // Debug..
        if !self.cube.empty() { view_new } else { view_main }
    }

    fn cube_view(&self) -> Html {
        html! {
            <div></div>
        }
    }
}
