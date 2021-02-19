use crate::util::*;
use crate::yew_util::*;
use crate::stockpile::prelude::*;
use crate::view_model::{Model, Message};


#[derive(Debug, Clone)]
pub struct HistoryModel {
    // src_view. true if src-code-view, false if cube view.
    pub src_view: bool,
    pub buffer_str: String,
    pub refs: HashMap<TimeTuple, NodeRef>,
    pub stockpile: Stockpile,
    pub link: ComponentLink<Model>
}

#[derive(Debug, Clone)]
pub enum HistoryMessage {
    UpdateBuffer(String),
    
    NewShot,
    VisitShot(TimeTuple),
    EraseShot(TimeTuple),

    FocusTime(TimeTuple),

    /// None if bare toggle; Some if force turn on / off.
    SrcViewToggle(Option<bool>),
    // Todo: _LogHistory -> Refresh.
    _LogHistory,
}

pub type HistoryMessages = Vec<HistoryMessage>;

impl HistoryModel {
    pub fn history_create(stockpile: &Stockpile, link: &ComponentLink<Model>) -> HistoryModel {
        let time_tuple_iter = stockpile.history.iter().map(|x| (x.flatten_tuple(), NodeRef::default()));
        let refs = HashMap::from_iter(time_tuple_iter);
        HistoryModel {
            src_view: false,
            buffer_str: String::new(),
            refs,
            stockpile: stockpile.clone(),
            link: link.clone()
        }
    }
    pub fn revisit(&mut self, msg: Message) {
        self.link.callback(move |_: ()| msg.clone() ).emit(());
    }
    pub fn history_update(&mut self, messages: HistoryMessages) -> ShouldRender {
        use HistoryMessage::*;
        if messages.is_empty() { return true; }
        // Test..
        LOG!("|--- buffer: {:?}", self.buffer_str);
        for message in messages {
            match message {
                UpdateBuffer(val) => {
                    self.buffer_str = val;
                }
                NewShot => {
                    self.stockpile.history.push(TimeStamp::<Branch>::new_snapshot(&self.stockpile.branch));
                    let time_tuple = self.stockpile.history.last().unwrap().time;
                    self.refs.insert(time_tuple, NodeRef::default());
                    self.revisit( Historyly![FocusTime(time_tuple)] );
                }
                VisitShot(time_tuple) => {
                    let index = self.stockpile.history.iter().position(|x| x.flatten_tuple() == time_tuple).unwrap();
                    let branch = match self.stockpile.history[index].data.clone() {
                        TimeMeta::Snapshot(x) => x,
                        _ => unreachable!()
                    };
                    self.stockpile.branch = branch;
                }
                EraseShot(time_tuple) => {
                    let index = self.stockpile.history.iter().position(|x| x.flatten_tuple() == time_tuple).unwrap();
                    self.stockpile.history.remove(index);
                }
                FocusTime(id) => {
                    let ref_obj = {
                        if self.refs.get(&id).is_none() {
                            self.refs.insert(id, NodeRef::default());
                        }
                        self.refs.get(&id).unwrap()
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
                            self.buffer_str = export_json(&self.stockpile);
                            true 
                        } else { 
                            match from_json_str(&self.buffer_str) {
                                Ok(stockpile) => { 
                                    self.stockpile = stockpile;
                                    self.refs.clear();
                                    self.refs.extend(
                                        self.stockpile.history.iter().map(|k| (k.flatten_tuple(), NodeRef::default()) )
                                    );
                                    true 
                                }
                                _ => false
                            }
                        };
                    if writing { self.src_view = src_view }
                }
                _LogHistory => {
                    LOG!("{:#?}", &self.stockpile);
                },
            }
        }


        true
    }
}
