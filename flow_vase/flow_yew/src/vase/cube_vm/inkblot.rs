use yew::{ComponentLink, Html, html, InputData};
use flow_vessel::{Cube, CubeMeta, EntityField, EntityId, Vessel};
use super::{CubeView, Vase, Msg::*};

#[derive(Clone)]
pub struct Inkblot {
    obj: EntityId,
    link: ComponentLink<Vase>
}

impl CubeView for Inkblot {
    fn view(&self, vessel: &Vessel, _: CubeMeta) -> Html {
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
