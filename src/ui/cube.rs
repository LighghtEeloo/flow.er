use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::ui::{KEY, Model, Message};

#[derive(Debug)]
pub enum CubeMessage {
    UpdateBuffer(String),
    NewCube,
    ClearCube,
    WriteCubeName,
    NewNode(Vec<EntryId>),
    WriteFace(EntryId),
    WriteProcess(EntryId),
    EraseNode(EntryId),
    SetFocusId(Option<EntryId>),
    Wander(Direction, bool),
    Focus,
}

impl CubeMessage {
    pub fn uni(a: CubeMessage) -> Message {
        Message::Cube(vec![a])
    }
    pub fn bi(a: [CubeMessage; 2]) -> Message {
        Message::Cube(Vec::from(a))
    }
    pub fn multi(a: CubeMessages) -> Message {
        Message::Cube(a)
    }
}

pub type CubeMessages = Vec<CubeMessage>;



impl Model {
    pub fn cube_update(&mut self, messages: CubeMessages) -> ShouldRender {
        use CubeMessage::*;
        // Test..
        if messages.is_empty() { return true; }
        LOG!("Updating with: {:?}\nand buffer: {:?}", messages, self.buffer_str);
        for msg in messages {
            match msg {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
                }
                NewCube => {
                    // Todo: Add new stockpile.
                    self.cube.name = mem::take(&mut self.buffer_str);
                }
                ClearCube => {
                    self.cube.relation.clear();
                    self.cube.entries.clear();
                }
                WriteCubeName => {
                    self.cube.name = mem::take(&mut self.buffer_str);
                }
                NewNode(vec) => {
                    if vec.len() == 0 {
                        let new_id = self.cube.grow();
                        self.cube.tiptoe(new_id);
                        self.refs.insert(new_id, NodeRef::default());
                        self.link.callback(move |_: ()| CubeMessage::uni(Focus) ).emit(());
                    } else {
                        // Todo: change the semantics.
                        for root_id in vec {
                            let new_id = self.cube.grow();
                            self.cube.chain(new_id, root_id);
                            self.refs.insert(new_id, NodeRef::default());
                            self.link.callback(move |_: ()| CubeMessage::uni(Focus) ).emit(());
                        }
                    }
                }
                WriteFace(id) => {
                    let x: &mut Entry = self.cube.entries.get_mut(&id).unwrap();
                    x.set_face(mem::take(&mut self.buffer_str));
                }
                WriteProcess(id) => {
                    let x: &mut Entry = self.cube.entries.get_mut(&id).unwrap();
                    x.set_process(ProcessStatus::reflect(self.buffer_str.as_str()));
                }
                EraseNode(id) => {
                    self.cube.erase(id);
                    self.link.callback(move |_: ()| CubeMessage::uni(Focus) ).emit(());
                }
                SetFocusId(id) => {
                    match id {
                        Some(id) => self.cube.relation.focus(id),
                        None => self.cube.relation.pos = None
                    }
                }
                Wander(dir, fixed) => {
                    // Todo: register shift key.
                    self.cube.relation.wander(dir, fixed);
                    self.link.callback(move |_: ()| CubeMessage::uni(Focus) ).emit(());
                }
                Focus => {
                    let id = self.cube.relation.current();
                    // Debug..
                    LOG!("Focusing: {:?}", id);
                    match id {
                        Some(id) => {
                            if let Some(input) = self.refs.get(&id).unwrap().cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                        }
                        None => {
                            if let Some(input) = self.ref_name.cast::<InputElement>() {
                                input.focus().unwrap();
                            }
                        }
                    }
                }
            }
        }
        // Note: Only self.stockpile is saved.
        self.storage.store(KEY, Json(&self.cube));
        // Debug..
        LOG!("Dumped {}: {:#?}", KEY, &self.cube);
        // LOG!("{}", to_json(&self.stockpile));
        true
    }
}