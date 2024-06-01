use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("java".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let src_dir = base_dir.clone().push("src".to_string()).push("main".to_string()).push("java".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(src_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(src_dir, "Main.java".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    let build_gradle = generate_build_gradle(base_dir);
    wrapped_actions.push(AbstractAction::CreateFile(build_gradle));
    wrapped_actions
}

fn generate_build_gradle(base_dir:AbstractDirectory) -> AbstractFile {
    let contents = String::from("
        apply plugin: 'java'
        apply plugin: 'application'

        mainClassName = 'Main'

        repositories {
            mavenCentral()
        }

        dependencies {
        }
    ");
    AbstractFile::new(base_dir, "build.gradle".to_string(), contents)
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("import java.io.*;\n");
    full_src.push_str("public class Main {\n");
    full_src.push_str("public static void main(String[] args) {\n");
    full_src.push_str(&code);
    full_src.push_str("}\n}\n");
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let path: Vec<String> = file.path().path();
                let full_path = path.join("/") + "/" + &file.name();
                code.push_str(&format!("try (PrintWriter writer = new PrintWriter(new File(\"{}\"))) {{\n", full_path));
                code.push_str(&format!("writer.write(\"{}\");\n", escape_string(&file.contents())));
                code.push_str("} catch (FileNotFoundException e) {\n    e.printStackTrace();\n}\n");
            }
            AbstractAction::CreateDirectory(dir) => {
                let dir_path = dir.path().join("/");
                code.push_str(&format!("new File(\"{}\").mkdirs();\n", dir_path));
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}