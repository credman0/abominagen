use crate::codeabstractions::{AbstractAction, AbstractFile, AbstractDirectory};

pub fn wrap_actions(actions:Vec<AbstractAction>) -> Vec<AbstractAction> {
    let code = to_code(actions);
    let mut wrapped_actions = Vec::new();
    let base_dir = AbstractDirectory::new().push("csharp".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(base_dir.clone()));
    let src_dir = base_dir.clone().push("src".to_string());
    wrapped_actions.push(AbstractAction::CreateDirectory(src_dir.clone()));
    let main_src = wrap_code(code);
    let main_file = AbstractFile::new(src_dir, "Program.cs".to_string(), main_src);
    wrapped_actions.push(AbstractAction::CreateFile(main_file));
    let csproj = generate_csproj(base_dir.clone());
    wrapped_actions.push(AbstractAction::CreateFile(csproj));
    let build_sh = generate_build_script(base_dir);
    wrapped_actions.push(AbstractAction::CreateFile(build_sh));
    wrapped_actions
}

fn generate_build_script(base_dir:AbstractDirectory) -> AbstractFile {
    let contents = r#"#!/bin/sh
dotnet build
"#;
    AbstractFile::new(base_dir, "build.sh".to_string(), contents.to_string())
}

fn generate_csproj(base_dir:AbstractDirectory) -> AbstractFile {
    let contents = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net5.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
    AbstractFile::new(base_dir, "csharp.csproj".to_string(), contents.to_string())
}

fn wrap_code(code:String) -> String {
    let mut full_src = String::new();
    full_src.push_str("using System;\n");
    full_src.push_str("using System.IO;\n");
    full_src.push_str("class Program {\n");
    full_src.push_str("    static void Main() {\n");
    full_src.push_str(&code);
    full_src.push_str("    }\n");
    full_src.push_str("}\n");
    full_src
}

fn to_code(actions:Vec<AbstractAction>) -> String {
    let mut code = String::new();
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let path: Vec<String> = file.path().path();
                let full_path = path.join("\\") + "\\" + &file.name();
                code.push_str(&format!("        File.WriteAllText(\"{}\", \"{}\");\n", full_path, escape_string(&file.contents())));
            }
            AbstractAction::CreateDirectory(dir) => {
                let dir_path = dir.path().join("\\");
                code.push_str(&format!("        Directory.CreateDirectory(\"{}\");\n", dir_path));
            }
        }
    }
    code
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\'", "\\\'").replace("\n", "\\n")
}