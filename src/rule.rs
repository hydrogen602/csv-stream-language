use regex::{Regex, escape};

use crate::commands::DataTypes;

#[derive(Debug)]
pub enum MatchPattern {
    String(Regex),
}

impl MatchPattern {
    /// convert * to .*? and compile regex
    pub fn compile_regex(pre_s: &str) -> Result<Self, regex::Error> {
        let s = escape(pre_s);
        // escape turns * into \*
        let reg = format!(r"^{}$", s.replace(r"\*", r".*?"));
        let r = Regex::new(&reg)?;
        Ok(Self::String(r))
    }

    pub fn is_match(&self, data: &DataTypes) -> bool {
        match self {
            Self::String(r) => {
                match data {
                    DataTypes::String(s) => {
                        r.is_match(s)
                    }
                    e => {
                        eprintln!("Warn: Tried comparing string to {:?}", e);
                        false
                    }
                }
            }
        }
    }
}

// impl FromStr for MatchPattern {
//     type Err = ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
        
//     }
// }
