use std::path::PathBuf;

use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("rust".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let src_dir = base_dir.clone().push("src".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(src_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(src_dir, "main.rs".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    let cargo_toml = generate_cargo_toml(base_dir);
    wrapped_actions.push(AbstractAction::CreateFile(cargo_toml));
    wrapped_actions
}

fn generate_cargo_toml(base_dir:AbstractDirectory) -> AbstractFile {
    let mut contents = String::new();
    contents.push_str("[package]\n");
    contents.push_str("name = \"rustgen\"\n");
    contents.push_str("version = \"0.1.0\"\n");
    contents.push_str("edition = \"2018\"\n");
    AbstractFile::new(base_dir, "Cargo.toml".to_string(), contents)
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("use std::fs::File;\n");
    full_src.push_str("use std::io::Write;\n");
    full_src.push_str("use std::path::PathBuf;\n");
    full_src.push_str("fn main() {\n");
    full_src.push_str(&code);
    full_src.push_str("}\n");
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let directory = file.path();
                if !directory.path().is_empty() {
                    code.push_str("let mut dir_path = PathBuf::new();");
                    for dir in directory.path() {
                        code.push_str(&format!("dir_path.push(\"{}\");\n", dir));
                    }
                    code.push_str(&format!("dir_path.push(\"{}\");\n", file.name()));
                } else {
                    code.push_str(&format!("let dir_path = PathBuf::from(\"{}\");\n", &file.name()));
                }
                code.push_str(&format!("let mut file = File::create(dir_path).expect(\"Unable to create file\");\n"));
                code.push_str(&format!("file.write_all(\"{}\".as_bytes()).expect(\"Unable to write to file\");\n", escape_string(&file.contents())));
            }
            AbstractAction::CreateDirectory(dir) => {
                code.push_str("let mut dir_path = PathBuf::new();");
                for dir in dir.path() {
                    code.push_str(&format!("dir_path.push(\"{}\");\n", dir));
                }
                code.push_str(&format!("std::fs::create_dir_all(dir_path).expect(\"Unable to create directory\");\n"));
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}