#[allow(dead_code)]
#[allow(unused)]

use crate::prelude::*;

use yew::format::Json;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::web_sys::console;
use yew::{classes, html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
use yew::{events::KeyboardEvent, Classes};

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
    path: TracerPath,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Note: default TracerPath.
        let path = TracerPath::from("tracer.data/tr.json");
        let cube = match Cube::load(&path) {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                Cube::new()
            }
        }; 
        let focus_ref = NodeRef::default();
        Self {
            link,
            cube,
            focus_ref,
            path
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => {}
        }
        // Todo: dump
        let mut entry = Entry::new();
        let id = entry.id();
        entry.face().push_str(&*format!("Yeh. {}", id));

        self.cube.entries.insert(id, entry);
        self.cube.dump(&self.path);
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
                <section class="sequence">
                    <header class="header">
                        <h1>{ "Sequence" }</h1>
                    </header>
                </section>
                <section class="main">
                    <input
                        type="checkbox"
                        class="toggle-all"
                        id="toggle-all"
                        checked=true
                        onclick=self.link.callback(|_| Msg::ToggleAll)
                    />
                    <label for="toggle-all" />
                    <ul class="todo-list">
                        { for list.iter().map(|e| html!(<div>{e}</div>)) }
                    </ul>
                </section>
                <footer class="info">
                    <p>{ format!("Path: {:?}.", self.path) }</p>
                </footer>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/LighghtEeloo/" target="_blank">{ "LighghtEeloo" }</a></p>
                    <p>{ "Inspired by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                </footer>
            </div>
        }
    }
}