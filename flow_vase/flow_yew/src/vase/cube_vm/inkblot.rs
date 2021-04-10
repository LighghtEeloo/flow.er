use yew::{ComponentLink, Html, InputData, NodeRef, html};
use std::collections::HashMap;
use flow_vessel::{CubeMeta, EntityField, EntityId, Vessel, cubes::Inkblot};
use super::{CubeView, Vase, Msg::*};


impl CubeView for Inkblot {
    fn view(&self, vessel: &Vessel, _: CubeMeta, link: ComponentLink<Vase>, _ref_map: &HashMap<EntityId, NodeRef>) -> Html {
        let entity = vessel.entity(&self.obj).cloned().expect("Host invalid.");
        let id = entity.id().clone();
        let link = link.clone();
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
