use std::path::PathBuf;

use include_dir::{include_dir, Dir};

static SELF_DIR: Dir = include_dir!("src/");
static BASE_DIR_NAME: &str = "abominagen";

#[derive(Debug, Clone)]
pub enum AbstractAction {
    CreateFile(AbstractFile),
    CreateDirectory(AbstractDirectory),
}

#[derive(Debug, Clone)]
pub struct AbstractFile {
    path:AbstractDirectory,
    name:String,
    contents:String
}

impl AbstractFile {
    pub fn new(path:AbstractDirectory, name:String, contents:String) -> AbstractFile {
        AbstractFile {
            path,
            name,
            contents
        }
    }

    pub fn path(&self) -> AbstractDirectory {
        self.path.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn contents(&self) -> String {
        self.contents.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AbstractDirectory {
    path:Vec<String>
}

impl AbstractDirectory {
    pub fn new() -> AbstractDirectory {
        AbstractDirectory {
            path:Vec::new()
        }
    }
    
    pub fn push(mut self, dir:String) -> Self {
        self.path.push(dir);
        self
    }

    pub fn path(&self) -> Vec<String> {
        self.path.clone()
    }
}

fn create_dir_actions_from_dir(dir: &Dir) -> Vec<AbstractAction> {
    let mut actions = Vec::new();
    for file_dir in dir.dirs() {
        let mut dir_path:Vec<String> = file_dir.path().into_iter().map(|s| s.to_str().expect("Invalid dir path").to_string()).collect();
        dir_path.insert(0, BASE_DIR_NAME.to_string());
        let dir = AbstractDirectory { path: dir_path };
        actions.push(AbstractAction::CreateDirectory(dir));
        actions.extend(create_dir_actions_from_dir(&file_dir));
    }
    actions
}

fn create_file_actions_from_dir(dir: &Dir) -> Vec<AbstractAction> {
    let mut actions = Vec::new();
    for file in dir.files() {
        let mut file_path:Vec<String> = file.path().into_iter().map(|s| s.to_str().expect("Invalid file path").to_string()).collect();
        file_path.insert(0, BASE_DIR_NAME.to_string());
        file_path.pop(); // Remove filename from end of path
        let contents = file.contents_utf8().unwrap();
        let file = AbstractFile::new(
            AbstractDirectory { path: file_path },
            file.path().file_name().unwrap().to_str().unwrap().to_string(),
            contents.to_string(),
        );
        actions.push(AbstractAction::CreateFile(file));
    }
    for dir in dir.dirs() {
        actions.extend(create_file_actions_from_dir(dir));
    }
    actions
}

pub fn self_creation_actions() -> Vec<AbstractAction> {
    let mut actions = vec![];
    actions.push(AbstractAction::CreateDirectory(AbstractDirectory::new().push(BASE_DIR_NAME.to_string())));
    actions.extend(create_dir_actions_from_dir(&SELF_DIR));
    actions.extend(create_file_actions_from_dir(&SELF_DIR));
    actions.push(AbstractAction::CreateFile(AbstractFile::new(AbstractDirectory::new().push(BASE_DIR_NAME.to_string()), "Cargo.toml".to_string(), include_str!("../../Cargo.toml").to_string())));
    actions
}

pub fn execute_actions(actions:Vec<AbstractAction>) {
    for action in actions {
        match action {
            AbstractAction::CreateFile(file) => {
                let directory = file.path();
                let mut file_path = file.path().path().iter().fold(PathBuf::new(), |acc: PathBuf, dir| acc.join(dir));
                if !directory.path().is_empty() {
                    std::fs::create_dir_all(&file_path).expect("Unable to create directory");            
                }
                file_path.push(&file.name());
                std::fs::write(file_path, file.contents()).expect("Unable to write to file");
            }
            AbstractAction::CreateDirectory(dir) => {
                std::fs::create_dir_all(dir.path().iter().fold(PathBuf::new(), |acc: PathBuf, dir| acc.join(dir))).expect("Unable to create directory");            
            }
        }
    }
}