use glob::glob;
use std::env;
use std::error;
use std::path::PathBuf;
use std::vec::Vec;
use wysgy_core;
pub struct Project;
impl Project {
    pub fn open(s: &String) -> Result<wysgy_core::Project, Box<error::Error>> {
        println!("Opening project path: {}", s);
        let prj = wysgy_core::Project::open(s.to_string())?;
        Ok(prj)
    }

    pub fn new(s: &String) -> Result<wysgy_core::Project, Box<error::Error>> {
        let prj = wysgy_core::Project::create(s.to_string())?;
        Ok(prj)
    }

    pub fn curr() -> Result<wysgy_core::Project, Box<error::Error>> {
        let pwd = env::current_dir().unwrap();
        let path = pwd.canonicalize().unwrap().to_str().unwrap().to_string();
        println!("Project path: {}", path);
        if PathBuf::from(path.clone()).exists() {
            Project::open(&path)
        } else {
            Project::new(&path)
        }
    }

    pub fn nodes() -> Result<Vec<String>, Box<error::Error>> {
        let prj = Project::curr()?;
        Ok(prj
            .nodes_list()
            .unwrap()
            .into_iter()
            .map(|e| {
                PathBuf::from(&e)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>())
    }
}
