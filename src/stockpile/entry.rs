use crate::util::*;
use crate::stockpile::time::*;
use crate::stockpile::identity::*;



// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    id: EntryId,
    pub face: Face,
    pub bubble: Bubble,
    pub filter: Filter,
}


pub type Face = String;
pub type Bubble = String;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    process: ProcessStatus,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProcessStatus {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}


impl Entry {
    pub fn new() -> Self {
        Entry::with_id(EntryId::new_stamped())
    }
    pub fn with_id(id: EntryId) -> Self {
        Self {
            id,
            face: Face::new(),
            bubble: Bubble::new(),
            filter: Filter::new()
        }
    }
    pub fn id(&self) -> EntryId {
        self.id.clone()
    }
    pub fn face(&self) -> &Face {
        &self.face
    }
    pub fn set_face(&mut self, face: Face) {
        self.face = face;
    }
    pub fn bubble(&self) -> &Bubble {
        &self.bubble
    }
    pub fn set_bubble(&mut self, bubble: Bubble) {
        self.bubble = bubble;
    }
    pub fn filter(&self) -> &Filter {
        &self.filter
    }
    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }
    pub fn process(&self) -> &ProcessStatus {
        &self.filter.process
    }
    pub fn set_process(&mut self, process: ProcessStatus) {
        self.filter.process = process;
    }
}

impl Default for Entry {
    fn default() -> Self { 
        Entry::new()
    }
    
}


// Filter impl.


impl Filter {
    pub fn new() -> Self {
        Filter {
            process: ProcessStatus::New,
            tags: Vec::new()
        }
    }
}

impl ProcessStatus {
    pub fn type_str(&self) -> String {
        use ProcessStatus::*;
        match self {
            Done => "Done",
            Marching => "Marching",
            Pending => "Pending",
            Planning => "Planning",
            New => "New",
        }.to_string()
    }
    pub fn reflect(name: &str) -> Self {
        use ProcessStatus::*;
        match name {
            "Done" => Done,
            "Marching" => Marching,
            "Pending" => Pending,
            "Planning" => Planning,
            "New" => New,
            _ => New,
        }
    }
    pub fn vec_all() -> Vec<Self> {
        use ProcessStatus::*;
        vec! {
            New,
            Planning,
            Pending,
            Marching,
            Done,
        }
    }
    pub fn type_src(&self) -> String {
        use ProcessStatus::*;
        // Todo: Replace dummy.
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}
