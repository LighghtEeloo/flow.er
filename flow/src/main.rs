use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

fn main() {
    // launch the web app
    dioxus_web::launch(App::run);
}

struct App;

impl App {
    fn run(cx: Scope) -> Element {
        let link = use_state(cx, || "".to_string());
        let dbg_text = use_state(cx, || format!(""));

        let links = use_ref(cx, || im::Vector::new());

        let inputbox = rsx!(input {
            value: "{link}",
            oninput: move |evt| {
                dbg_text.set(evt.inner().value.clone());
                link.set(evt.value.clone())
            },
            onkeydown: move |evt| {
                if evt.inner().key() == Key::Enter {
                    let mut links = links.write();
                    links.push_back(link.get().clone());
                    link.set("".to_string());
                }
            }
        });

        cx.render(rsx!(
            div { "{dbg_text}" }
            inputbox
        ))
    }
}
