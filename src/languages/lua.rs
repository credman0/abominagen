use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("lua".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(base_dir, "main.lua".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    wrapped_actions
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str(&code);
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let directory = file.path();
                if !directory.path().is_empty() {
                    code.push_str("local dir_path = ''\n");
                    for dir in directory.path() {
                        code.push_str(&format!("dir_path = dir_path .. '{}/'\n", dir));
                    }
                    code.push_str(&format!("dir_path = dir_path .. '{}'\n", file.name()));
                } else {
                    code.push_str(&format!("local dir_path = '{}'\n", &file.name()));
                }
                code.push_str("local file = io.open(dir_path, 'w')\n");
                code.push_str(&format!("file:write('{}')\n", escape_string(&file.contents())));
                code.push_str("file:close()\n");
            }
            AbstractAction::CreateDirectory(dir) => {
                code.push_str("local dir_path = ''\n");
                for dir in dir.path() {
                    code.push_str(&format!("dir_path = dir_path .. '{}/'\n", dir));
                }
                code.push_str("os.execute('mkdir -p ' .. dir_path)\n");
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}