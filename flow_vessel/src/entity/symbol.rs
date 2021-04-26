use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Symbol {
    Processing(Process),
    Linted(Lint),
    // Innocent
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Linted(Lint::default())
    }
}

impl Symbol {
    pub fn parse(attempt: String) -> Option<Self> {
        match attempt.as_str() {
            "New" => Some(Symbol::Processing(Process::New)),
            "Planning" => Some(Symbol::Processing(Process::Planning)),
            "Pending" => Some(Symbol::Processing(Process::Pending)),
            "Marching" => Some(Symbol::Processing(Process::Marching)),
            "Done" => Some(Symbol::Processing(Process::Done)),
            "Numberic" => Some(Symbol::Linted(Lint::Numberic)),
            "Programmatic" => Some(Symbol::Linted(Lint::Programmatic)),
            "Upper" => Some(Symbol::Linted(Lint::Upper)),
            "Lower" => Some(Symbol::Linted(Lint::Lower)),
            "Greek" => Some(Symbol::Linted(Lint::Greek)),
            "Circle" => Some(Symbol::Linted(Lint::Circle)),
            "Square" => Some(Symbol::Linted(Lint::Square)),
            "Dash" => Some(Symbol::Linted(Lint::Dash)),
            _ => None,
        }
    }

    fn vague_mapping(attempt: &str) -> String {
        let possibilities = [
            "New",
            "Planning",
            "Pending",
            "Marching",
            "Done",
            "Numberic",
            "Programmatic",
            "Upper",
            "Lower",
            "Greek",
            "Circle",
            "Square",
            "Dash",
        ];
        let candidates: Vec<String> = possibilities
            .iter()
            .filter(|x| {
                x.to_lowercase()
                    .starts_with(attempt.to_lowercase().as_str())
            })
            .map(|x| format!("{}", x))
            .collect();
        if candidates.len() == 1 {
            candidates[0].clone()
        } else {
            format!("")
        }
    }

    pub fn parse_vague(attempt: &str) -> Option<Self> {
        Self::parse(Self::vague_mapping(attempt))
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Process {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}

impl Default for Process {
    fn default() -> Self {
        Process::New
    }
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
        }
        .to_string()
    }
    pub fn vec_all() -> Vec<Self> {
        vec![New, Planning, Pending, Marching, Done]
    }
    pub fn type_src(&self) -> String {
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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
    // Diamond ◆

    // UnOrdered(String)
}

impl Default for Lint {
    fn default() -> Self {
        Lint::Square
    }
}

impl Lint {
    pub fn type_str(&self) -> &str {
        match self {
            Lint::Numberic => "",
            Lint::Programmatic => "programmatic",
            Lint::Upper => "upper",
            Lint::Lower => "lower",
            Lint::Greek => "greek",
            Lint::Circle => "",
            Lint::Square => "",
            Lint::Dash => "", // Lint::UnOrdered(_) => {""}
        }
    }
    pub fn vec_all() -> Vec<Self> {
        use Lint::*;
        vec![
            Numberic,
            Programmatic,
            Upper,
            Lower,
            Greek,
            // Roman,
            Circle,
            Square,
            Dash,
        ]
    }
    pub fn display(&self, mut idx: usize) -> String {
        use Lint::*;
        // Some backups.
        "small: ⚬";
        "machine: ⚉";
        "solid: ●";
        match self {
            Numberic => {
                format!("{}", idx + 1)
            }
            Circle => "○".into(),
            Square => "▣".into(),
            Dash => "—".into(),
            // UnOrdered(s) => {s.into()}
            Programmatic | Upper | Lower | Greek => {
                let mut s = String::from("");
                let (size, chars) = BABEL[self.type_str()];
                loop {
                    let x = idx % size;
                    s.insert(0, chars[x]);
                    idx = (idx - x) / size;
                    if idx == 0 {
                        break;
                    }
                }
                s
            }
        }
    }
}

pub type AlphaBet = (usize, [char; 32]);
pub type Babel = phf::Map<&'static str, AlphaBet>;

pub static BABEL: Babel = phf::phf_map! {
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
            println!(
                "{},\t{},\t{},\t{},\t{},\t{},\t{}",
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
    #[test]
    fn symbol_parse() {
        assert_eq!(
            Symbol::parse_vague("new"),
            Some(Symbol::Processing(Process::New))
        );
        assert_eq!(
            Symbol::parse_vague("pl"),
            Some(Symbol::Processing(Process::Planning))
        );
        assert_eq!(
            Symbol::parse_vague("Pen"),
            Some(Symbol::Processing(Process::Pending))
        );
        assert_eq!(
            Symbol::parse_vague("mar"),
            Some(Symbol::Processing(Process::Marching))
        );
        assert_eq!(
            Symbol::parse_vague("Done"),
            Some(Symbol::Processing(Process::Done))
        );
        assert_eq!(
            Symbol::parse_vague("num"),
            Some(Symbol::Linted(Lint::Numberic))
        );
        assert_eq!(
            Symbol::parse_vague("pr"),
            Some(Symbol::Linted(Lint::Programmatic))
        );
        assert_eq!(
            Symbol::parse_vague("up"),
            Some(Symbol::Linted(Lint::Upper))
        );
        assert_eq!(
            Symbol::parse_vague("Lower"),
            Some(Symbol::Linted(Lint::Lower))
        );
        assert_eq!(
            Symbol::parse_vague("gr"),
            Some(Symbol::Linted(Lint::Greek))
        );
        assert_eq!(
            Symbol::parse_vague("ci"),
            Some(Symbol::Linted(Lint::Circle))
        );
        assert_eq!(
            Symbol::parse_vague("s"),
            Some(Symbol::Linted(Lint::Square))
        );
        assert_eq!(
            Symbol::parse_vague("Dash"),
            Some(Symbol::Linted(Lint::Dash))
        );
        assert_eq!(Symbol::parse_vague("Dashy"), None);
        assert_eq!(Symbol::parse_vague("p"), None);
    }
}
