use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::ui::{KEY, Model, Message};

#[derive(Debug, Clone)]
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

    SrcViewToggle(Option<bool>),
    // Debug..
    _LogCube,
}

impl CubeMessage {
    // pub fn uni(a: CubeMessage) -> Message {
    //     Message::Cube(vec![a])
    // }
    // pub fn bi(a: [CubeMessage; 2]) -> Message {
    //     Message::Cube(Vec::from(a))
    // }
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
                        self.revisit( Cubey![Focus] );
                    } else {
                        // Todo: change the semantics.
                        for root_id in vec {
                            let new_id = self.cube.grow();
                            self.cube.chain(new_id, root_id);
                            self.refs.insert(new_id, NodeRef::default());
                            self.revisit( Cubey![Focus] );
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
                    self.revisit( Cubey![Focus] );
                }
                SetFocusId(id) => {
                    match id {
                        Some(id) => self.cube.relation.focus(id),
                        None => self.cube.relation.pos = None
                    }
                }
                Wander(dir, fixed) => {
                    self.cube.relation.wander(dir, fixed);
                    self.revisit( Cubey![Focus] );
                }
                Focus => {
                    let id = self.cube.relation.current();
                    let ref_obj = match id {
                        Some(id) => self.refs.get(&id).unwrap(),
                        None => &self.ref_cube_name,
                    };
                    if let Some(input) = ref_obj.cast::<InputElement>() {
                        input.focus().unwrap();
                    }
                }
                SrcViewToggle(x) => {
                    let src_view = match x {
                        None => !self.src_view,
                        Some(x) => x
                    };
                    let writing = if !src_view {
                        match from_json_str(&self.buffer_str) {
                            Ok(cube) => { self.cube = cube; true }
                            _ => false
                        }
                    } else { true };
                    if writing { self.src_view = src_view }
                    self.revisit( Cubey![_LogCube] );
                }
                _LogCube => LOG!("{}", to_json(&self.cube)),
            }
        }
        // Note: Only self.stockpile is saved.
        self.storage.store(KEY, Json(&self.cube));
        // Debug..
        LOG!("Dumped {}: {:#?}", KEY, &self.cube);
        // LOG!("{}", to_json(&self.cube));
        true
    }
}