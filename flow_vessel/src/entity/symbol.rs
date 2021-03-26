use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Symbol {
    ProcessTracker(Process),
    Linted(Lint),
    Innocent
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Process {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}

use Process::*;
impl Process {
    pub fn type_str(&self) -> String {
        match self {
            Done => "Done",
            Marching => "Marching",
            Pending => "Pending",
            Planning => "Planning",
            New => "New",
        }.to_string()
    }
    pub fn vec_all() -> Vec<Self> {
        vec! {
            New,
            Planning,
            Pending,
            Marching,
            Done,
        }
    }
    pub fn type_src(&self) -> String {
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Lint {
    Numberic,
    Programmatic,
    Upper,
    Lower,
    Greek,
    // Roman,

    Circle,
    Square,
    Dash,
    
    UnOrdered(String)
}

impl Default for Lint {
    fn default() -> Self {
        Lint::Square
    }
}

// Todo: impl Lint.
impl Lint {
    pub fn type_str(&self) -> &str {
        match self {
            Lint::Numberic => {""}
            Lint::Programmatic => {"programmatic"}
            Lint::Upper => {"upper"}
            Lint::Lower => {"lower"}
            Lint::Greek => {"greek"}
            Lint::Circle => {""}
            Lint::Square => {""}
            Lint::Dash => {""}
            Lint::UnOrdered(_) => {""}
        }
    }
    pub fn display(&self, mut idx: usize) -> String {
        use Lint::*;
        // Note: Some backups.
        "small: ⚬";
        "machine: ⚉";
        "solid: ●";
        match self {
            Numberic => {format!("{}", idx+1)}
            Circle => {"○".into()}
            Square => {"▣".into()}
            Dash => {"-".into()}
            UnOrdered(s) => {s.into()}
            Programmatic | Upper | Lower | Greek => {
                let mut s = String::new();
                let (size, chars) = BABEL[self.type_str()];
                loop {
                    let x = idx % size;
                    s.insert(0, chars[x]);
                    idx = (idx - x) / size;
                    if idx == 0 { break; }
                }
                s
            }
        }
    }
}


pub type AlphaBet = (usize, [char; 32]);
pub type Babel = phf::Map<&'static str, AlphaBet>;

pub static BABEL: Babel = phf::phf_map!{
    "programmatic" => (10, ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00']),
    "upper" => (26, ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00']),
    "lower" => (26, ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00']),
    "greek" => (25, ['α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'ς', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00']),
    "" => (0, ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'])
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn babel() {
        println!("upper: {:?}", BABEL.get("upper"));
        println!("nothing: {:?}", BABEL.get("nothing"));
        assert_eq!('A', (BABEL["upper"].1)[0])
    }
    #[test]
    fn lint() {
        for idx in 0..200 {
            println!("{},\t{},\t{},\t{},\t{},\t{},\t{}", 
                Lint::Programmatic.display(idx),
                Lint::Numberic.display(idx),
                Lint::Upper.display(idx),
                Lint::Greek.display(idx),
                Lint::Square.display(idx),
                Lint::Circle.display(idx),
                Lint::Dash.display(idx),
            );
        }
    }
}