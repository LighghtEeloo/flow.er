use crate::util::*;
use crate::stockpile::time::*;
use crate::stockpile::identity::*;



// Entry Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    dry: EntryDry,
    // timestamps: Vec<TimeStamp>,
    // Todo: Add positional info.
    // position: (f64, f64)
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryDry {
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
        let stamp = TimeStamp::created();
        let id = EntryId::from_time(&stamp.data);
        Entry {
            dry: EntryDry::new(id),
            // timestamps: vec!(stamp)
        }
    }
    pub fn dry(&self) -> EntryDry {
        self.dry.clone()
    }
    pub fn strip(self) -> EntryDry {
        self.dry
    }
    pub fn id(&self) -> EntryId {
        self.dry.id.clone()
    }
    pub fn face(&self) -> &Face {
        &self.dry.face
    }
    pub fn set_face(&mut self, face: Face) {
        self.dry.face = face;
    }
    pub fn bubble(&self) -> &Bubble {
        &self.dry.bubble
    }
    pub fn set_bubble(&mut self, bubble: Bubble) {
        self.dry.bubble = bubble;
    }
    pub fn filter(&self) -> &Filter {
        &self.dry.filter
    }
    pub fn set_filter(&mut self, filter: Filter) {
        self.dry.filter = filter;
    }
    pub fn process(&self) -> &ProcessStatus {
        &self.dry.filter.process
    }
    pub fn set_process(&mut self, process: ProcessStatus) {
        self.dry.filter.process = process;
    }
}

impl From<EntryDry> for Entry {
    fn from(v: EntryDry) -> Self {
        Entry {
            dry: v,
            // timestamps: vec!(TimeStamp::created())
        }
    }
}

impl Default for Entry {
    fn default() -> Self { 
        Entry::new()
    }
    
}

impl EntryDry {
    pub fn new(id: EntryId) -> Self {
        EntryDry {
            id,
            face: Face::new(),
            bubble: Bubble::new(),
            filter: Filter::new()
        }
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
