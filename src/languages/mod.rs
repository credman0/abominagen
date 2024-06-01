pub mod rust;
pub mod python;
pub mod c;
pub mod cpp;
pub mod lua;
pub mod java;
pub mod javascript;
pub mod typescript;
pub mod ruby;
pub mod go;
pub mod php;
pub mod csharp;
pub mod perl;


use crate::codeabstractions::AbstractAction;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    Python,
    C,
    Cpp,
    Lua,
    Java,
    JavaScript,
    TypeScript,
    Ruby,
    Go,
    Php,
    CSharp,
    Perl
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Language::Rust),
            "python" => Ok(Language::Python),
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::Cpp),
            "lua" => Ok(Language::Lua),
            "java" => Ok(Language::Java),
            "javascript" => Ok(Language::JavaScript),
            "typescript" => Ok(Language::TypeScript),
            "ruby" => Ok(Language::Ruby),
            "go" => Ok(Language::Go),
            "php" => Ok(Language::Php),
            "csharp" => Ok(Language::CSharp),
            "perl" => Ok(Language::Perl),
            _ => Err(format!("'{}' is not a recognized language. Please use one of the following: rust, python, c, cpp, lua, java, javascript, typescript, ruby, go, php, csharp, perl", s)),
        }
    }
}

impl Language {
    pub fn wrap_actions(&self, actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
        match self {
            Language::Rust => rust::wrap_actions(actions),
            Language::Python => python::wrap_actions(actions),
            Language::C => c::wrap_actions(actions),
            Language::Cpp => cpp::wrap_actions(actions),
            Language::Lua => lua::wrap_actions(actions),
            Language::Java => java::wrap_actions(actions),
            Language::JavaScript => javascript::wrap_actions(actions),
            Language::TypeScript => typescript::wrap_actions(actions),
            Language::Ruby => ruby::wrap_actions(actions),
            Language::Go => go::wrap_actions(actions),
            Language::Php => php::wrap_actions(actions),
            Language::CSharp => csharp::wrap_actions(actions),
            Language::Perl => perl::wrap_actions(actions),
        }
    }

    /// return 'script' for scripts and 'program' for compiled languages
    pub fn get_descriptor(&self) -> String {
        match self {
            Language::Rust => "program".to_string(),
            Language::Python => "script".to_string(),
            Language::C => "program".to_string(),
            Language::Cpp => "program".to_string(),
            Language::Lua => "script".to_string(),
            Language::Java => "program".to_string(),
            Language::JavaScript => "script".to_string(),
            Language::TypeScript => "script".to_string(),
            Language::Ruby => "script".to_string(),
            Language::Go => "program".to_string(),
            Language::Php => "script".to_string(),
            Language::CSharp => "program".to_string(),
            Language::Perl => "script".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Language::Rust => "Rust".to_string(),
            Language::Python => "Python".to_string(),
            Language::C => "C".to_string(),
            Language::Cpp => "C++".to_string(),
            Language::Lua => "Lua".to_string(),
            Language::Java => "Java".to_string(),
            Language::JavaScript => "JavaScript".to_string(),
            Language::TypeScript => "TypeScript".to_string(),
            Language::Ruby => "Ruby".to_string(),
            Language::Go => "Go".to_string(),
            Language::Php => "PHP".to_string(),
            Language::CSharp => "C#".to_string(),
            Language::Perl => "Perl".to_string(),
        }
    }
}

