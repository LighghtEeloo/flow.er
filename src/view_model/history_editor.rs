use crate::yew_util::*;
// use crate::stockpile::prelude::*;
use crate::view_model::*;

use HistoryMessage::*;


// History_view

impl HistoryModel {
    pub fn history_view(&self) -> Html {
        let vec = &self.stockpile.history;
        html! {
            <div class="history">
                { self.snapshot_capture_button() }
                <div class="node-group">
                    { for vec.iter().enumerate().rev().map(|(index, _)| self.node_view(index)) }
                </div>
                <link rel="stylesheet" href="static/history.css" />
            </div>
        }
    }

    
    fn snapshot_capture_button(&self) -> Html {
        html! {
            <button class="snapshot-button"
                title="Create Snapshot."
                onclick=self.link.callback(move |_| {
                    Historyly![NewShot, _LogHistory]
                })
            >
                <img src="static/icons/History/create.svg" alt="code_pic"/>
                <span>{"   Create Snapshot   "}</span>
            </button>
        }
    }


    fn node_view(&self, index: usize) -> Html {
        html! {
            <div class="node">
                { self.node_span_view(index) }
                { self.node_visit_button_view(index) }
                { self.node_erase_button_view(index) }
            </div>
        }
    }


    fn node_span_view(&self, index: usize) -> Html {
        let time = self.stockpile.history[index].time;
        let sys_time = &self.stockpile.history[index];
        html! {
            <span
                type="text"
                ref=self.refs.get(&time).unwrap().clone()
                aria-label="TimeShot"
            > 
                { format!("{}", sys_time) }
            </span>
        }
    }


    fn node_visit_button_view(&self, index: usize) -> Html {
        let time = self.stockpile.history[index].time;
        // let sys_time = self.stockpile.history[index].universal();
        html! {
            <button class="visit-button"
                title="Visit Snapshot."
                onclick=self.link.callback(move |_| {
                    Historyly![VisitShot(time)]
                })
            >
                <img src="static/icons/History/visit.svg" alt="code_pic"/>
                // <span>{"Visit Snapshot"}</span>
            </button>
        }
    }


    fn node_erase_button_view(&self, index: usize) -> Html {
        let time = self.stockpile.history[index].time;
        // let sys_time = self.stockpile.history[index].universal();
        html! {
            <button class="delete-button"
                title="Delete Snapshot."
                ondblclick=self.link.callback(move |_| {
                    Historyly![EraseShot(time)]
                })
            >
                <img src="static/icons/History/delete.svg" alt="code_pic"/>
                // <span>{"Delete Snapshot"}</span>
            </button>
        }
    }

}

// History_src_view

impl HistoryModel {
    pub fn src_view(&self) -> Html {
        html! {
            <div class="src">
                <textarea class="src-input"
                    value=if self.src_view { self.buffer_str.clone() } else { String::new() }
                    type="text" 
                    oninput=self.link.callback(move |e: InputData| {
                        LOG!("OnInput: {:?}", e);
                        Historyly![UpdateBuffer(e.value)]
                    })
                    spellcheck=false
                />
            </div>
        }
    }
}
