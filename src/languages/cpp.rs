use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("cpp".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(base_dir.clone(), "main.cpp".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    let makefile = generate_makefile(base_dir);
    wrapped_actions.push(AbstractAction::CreateFile(makefile));
    wrapped_actions
}

fn generate_makefile(base_dir:AbstractDirectory) -> AbstractFile {
    let contents = String::from("all:\n\tg++ -o main main.cpp\nrun:\n\t./main\n");
    AbstractFile::new(base_dir, "Makefile".to_string(), contents)
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("#include <fstream>\n");
    full_src.push_str("#include <sys/stat.h>\n");
    full_src.push_str("int main() {\n");
    full_src.push_str(&code);
    full_src.push_str("return 0;\n}\n");
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let path: Vec<String> = file.path().path();
                let full_path = path.join("/") + "/" + &file.name();
                code.push_str(&format!("std::ofstream file(\"{}\");\n", full_path));
                code.push_str(&format!("file << \"{}\";\n", escape_string(&file.contents())));
                code.push_str("file.close();\n");
            }
            AbstractAction::CreateDirectory(dir) => {
                let dir_path = dir.path().join("/");
                code.push_str(&format!("mkdir(\"{}\");\n", dir_path));
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}