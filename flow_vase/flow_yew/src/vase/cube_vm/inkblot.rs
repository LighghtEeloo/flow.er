use yew::{ComponentLink, Html, html, InputData};
use flow_vessel::{Cube, EntityField, EntityId, Vessel};
use super::{CubeView, Vase, Msg::*};

#[derive(Clone)]
pub struct Inkblot {
    obj: EntityId,
    link: ComponentLink<Vase>
}

impl Into<CubeView> for Inkblot {
    fn into(self) -> CubeView {
        CubeView::Inkblot {
            inkblot: self
        }
    }
}

impl Inkblot {
    pub fn new_cube(cube: Cube, link: ComponentLink<Vase>) -> CubeView {
        let inkblot = 
            Self {
                obj: cube.obj.unwrap_or_default(),
                link
            };
        inkblot.into()
    }
    pub fn update_new(mut self, cube: &Cube, vessel: &Vessel) -> CubeView {
        let blank = CubeView::default();
        if let Some(obj) = cube.obj {
            if vessel.entity_get(&obj).is_some() {
                self.obj = obj;
                self.into()
            } else {
                blank
            }
        } else {
            blank
        }
    }
    pub fn view(&self, vessel: &Vessel) -> Html {
        let entity = vessel.entity_get(&self.obj).cloned().expect("Host invalid.");
        let id = entity.id().clone();
        let link = self.link.clone();
        html! {
            <div class="inkblot">
                <div class="head">
                    <input
                        type="Text"
                        placeholder="An arbitrary node."
                        aria-label="Arbitrary Node"
                        value=entity.face
                        oninput=link.callback(move |e: InputData| {
                            [EntityUpdate{
                                id, 
                                field: EntityField::Face(e.value)
                            }]
                        })
                    />
                </div>
                <textarea class="bubble"
                    value=entity.bubble
                    type="text" 
                    oninput=link.callback(move |e: InputData| {
                        [EntityUpdate{
                            id, 
                            field: EntityField::Bubble(e.value)
                        }]
                    })
                    spellcheck=false
                />
            </div>
        }
    }
}
