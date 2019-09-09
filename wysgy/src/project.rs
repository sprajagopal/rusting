use std::env;
use std::error;
use std::path::PathBuf;
use std::vec::Vec;
use wysgy_core::{self, node::Node};
pub struct Project;
impl Project {
    pub fn open(s: &String) -> Result<wysgy_core::Project, Box<error::Error>> {
        info!("Opening project path: {}", s);
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
        info!("Project path: {}", path);
        if PathBuf::from(path.clone()).exists() {
            Project::open(&path)
        } else {
            Project::new(&path)
        }
    }

    pub fn nodes(t: Option<String>) -> Result<Vec<Node>, Box<error::Error>> {
        let prj = Project::curr()?;
        Ok(prj.nodes_list(t).unwrap())
    }

    pub fn types() -> Result<Vec<String>, Box<error::Error>> {
        let prj = Project::curr()?;
        prj.types_list()
    }

    pub fn edit_node(lbl: &String) {
        let prj = Project::curr().unwrap();
        prj.edit_node(lbl);
    }

    pub fn update_node(lbl: &String) -> Result<Node, Box<error::Error>> {
        let prj = Project::curr().unwrap();
        prj.get_node(lbl)
    }
}
