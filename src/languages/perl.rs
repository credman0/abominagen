use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("perl".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(base_dir.clone(), "main.pl".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    wrapped_actions
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("#!/usr/bin/perl\n");
    full_src.push_str("use strict;\n");
    full_src.push_str("use warnings;\n");
    full_src.push_str("use File::Path qw(make_path);\n");
    full_src.push_str("use File::Slurp;\n");
    full_src.push_str(&code);
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let path: Vec<String> = file.path().path();
                let full_path = path.join("/") + "/" + &file.name();
                code.push_str(&format!("write_file('{}', '{}');\n", full_path, escape_string(&file.contents())));
            }
            AbstractAction::CreateDirectory(dir) => {
                let dir_path = dir.path().join("/");
                code.push_str(&format!("make_path('{}');\n", dir_path));
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}