use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("typescript".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let src_dir = base_dir.clone().push("src".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(src_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(src_dir, "main.ts".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    let package_json = generate_package_json(base_dir.clone());
    wrapped_actions.push(AbstractAction::CreateFile(package_json));
    let tsconfig_json = generate_tsconfig_json(base_dir);
    wrapped_actions.push(AbstractAction::CreateFile(tsconfig_json));
    wrapped_actions
}

fn generate_package_json(base_dir:AbstractDirectory) -> AbstractFile {
        let contents = r#"{
    "name": "typescriptgen",
    "version": "1.0.0",
    "main": "src/main.ts",
    "scripts": {
        "start": "ts-node src/main.ts"
    },
    "devDependencies": {
        "typescript": "^4.0.0",
        "ts-node": "^10.0.0"
    }
}"#;
        AbstractFile::new(base_dir, "package.json".to_string(), contents.to_string())
}

fn generate_tsconfig_json(base_dir:AbstractDirectory) -> AbstractFile {
    let contents = r#"{
  "compilerOptions": {
    "target": "es5",
    "module": "commonjs",
    "outDir": "dist",
    "strict": true,
    "esModuleInterop": true
  }
}"#;
    AbstractFile::new(base_dir, "tsconfig.json".to_string(), contents.to_string())
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("import * as fs from 'fs';\n");
    full_src.push_str("import * as path from 'path';\n");
    full_src.push_str("\n");
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
                code.push_str(&format!("fs.writeFileSync(path.join(__dirname, '..', '{}'), '{}');\n", full_path, escape_string(&file.contents())));
            }
            AbstractAction::CreateDirectory(dir) => {
                let dir_path = dir.path().join("/");
                code.push_str(&format!("if (!fs.existsSync(path.join(__dirname, '..', '{}'))) {{\n", dir_path));
                code.push_str(&format!("  fs.mkdirSync(path.join(__dirname, '..', '{}'), {{ recursive: true }});\n", dir_path));
                code.push_str("}\n");
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}