use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::view_model::{Model, Message};


#[derive(Debug, Clone)]
pub struct CubeModel {
    // src_view. true if src-code-view, false if cube view.
    pub src_view: bool,
    // erase_lock. true if locked, false if to-erase.
    pub erase_lock: bool,
    pub cube: Cube,
    pub buffer_str: String,
    pub refs: HashMap<EntryId, NodeRef>,
    pub ref_cube_name: NodeRef,
    pub link: ComponentLink<Model>
}

#[derive(Debug, Clone)]
pub enum CubeMessage {
    UpdateBuffer(String),

    ClearCube,
    WriteCubeName,

    NewNode(Vec<EntryId>),
    WriteFace(EntryId),
    WriteProcess(EntryId),
    EraseNode(EntryId),

    SetFocusId(Option<EntryId>),
    Wander(Direction, bool),
    Focus,

    /// None if bare toggle; Some if force turn on / off.
    SrcViewToggle(Option<bool>),
    // Todo: _LogCube -> Refresh.
    _LogCube,
}

pub type CubeMessages = Vec<CubeMessage>;

impl CubeModel {
    pub fn cube_create(cube: &Cube, link: &ComponentLink<Model>) -> CubeModel {
        let entry_id_iter = cube.entries.keys().map(|x| (x.clone(),NodeRef::default()));
        let entry_ref = HashMap::from_iter(entry_id_iter);
        CubeModel {
            src_view: false,
            erase_lock: true,
            cube: cube.clone(),
            buffer_str: String::new(),
            refs: entry_ref,
            ref_cube_name: NodeRef::default(),
            link: link.clone()
        }
    }
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
    pub fn cube_update(&mut self, messages: CubeMessages) -> ShouldRender {
        use CubeMessage::*;
        if messages.is_empty() { return true; }
        // Test..
        LOG!("|--- buffer: {:?}", self.buffer_str);
        let old_erase_lock = self.erase_lock;
        for message in messages {
            match message {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
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
                        self.cube.chain(new_id, None);
                        self.refs.insert(new_id, NodeRef::default());
                        self.revisit( Cubey![Focus] );
                    } else {
                        // Note: 0 or 1.
                        let root_id = vec[0];
                        let new_id = self.cube.grow();
                        self.cube.chain(new_id, Some(root_id));
                        self.refs.insert(new_id, NodeRef::default());
                        self.revisit( Cubey![Focus] );
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
                    if self.erase_lock {
                        self.erase_lock = false;
                    } else {
                        self.cube.erase(id);
                        self.revisit( Cubey![Focus] );
                        self.erase_lock = true;
                    }
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
                    let writing = 
                        if src_view {
                            self.buffer_str = export_json(&self.cube);
                            true 
                        } else { 
                            match from_json_str(&self.buffer_str) {
                                Ok(cube) => { 
                                    self.cube = cube;
                                    self.refs.clear();
                                    self.refs.extend(
                                        self.cube.entries.keys().map(|&k| (k, NodeRef::default()) )
                                    );
                                    true 
                                }
                                _ => false
                            }
                        };
                    if writing { self.src_view = src_view }
                }
                _LogCube => {
                    LOG!("{:#?}", &self.cube);
                },
            }
        }

        // Note: Restore lock.
        if old_erase_lock == false {
            self.erase_lock = true;
        }


        true
    }
}
