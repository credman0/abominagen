pub mod rust;
pub mod python;
pub mod c;

use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};
use std::str::FromStr;

#[derive(Debug)]
pub enum Language {
    Rust,
    Python,
    C
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Language::Rust),
            "python" => Ok(Language::Python),
            "c" => Ok(Language::C),
            _ => Err("no match"),
        }
    }
}

impl Language {
    pub fn wrap_actions(&self, actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
        match self {
            Language::Rust => rust::wrap_actions(actions),
            Language::Python => python::wrap_actions(actions),
            Language::C => c::wrap_actions(actions)
        }
    }
}