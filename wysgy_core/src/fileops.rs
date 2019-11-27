use crate::converter::Converter;
use crate::errors::WysgyResult;
use crate::files::{CONFIG_JSON_CONTENTS, GV_TEMPLATE_CONTENTS};
use crate::node::Node;
use crate::rel::Rel;
use crate::wysgy::Wysgy;
use glob::glob;
use prettytable::Table;
use serde_json::{json, Value};
use std::cmp::max;
use std::error;
use std::fs::{self, DirEntry};
use std::io::{BufRead, ErrorKind};
use std::path::{PathBuf, Path};
use std::process::{Child, Command};

struct FileLoc {}

impl FileLoc {
    fn nodes_dir(w: &Wysgy) -> PathBuf {
        let nd = w.path.clone();
        nd.push("nodes");
        nd
    }

    fn rels_dir(w: &Wysgy) -> PathBuf {
        let rd = w.path.clone();
        rd.push("rels");
        rd
    }

    fn max_file_id(path: PathBuf) -> WysgyResult<usize> {
        info!("Finding largest id...");
        let max_id: usize = 0;
        // find largest id in folder
        for fl in fs::read_dir(path)? {
            max_id = max(
                fl.expect("No dir entry")
                    .path()
                    .file_stem()
                    .expect("No file stem")
                    .to_str()
                    .expect("No str")
                    .parse::<usize>()?,
                max_id,
            );
        }
        Ok(max_id)
    }

    fn apply_to_files_until_true(path: &Path, f: fn(&DirEntry)-> bool) -> WysgyResult<()>{
        if path.is_dir() {
            for e in fs::read_dir(path)? {
                if e?.path().is_dir() {
                    FileLoc::apply_to_files_until_true(path, f)
                } else {
                    if f(&e?) {
                        
                    }
                }
            }
        }
        Ok(())
    }
}

// Interface between file system and graph structure
//  get a node back from a given id
//  get a rel back from a given id
//  get all nodes
//  save a node with a given id
//  save a rel with a given id
//  add a new node at correct location with new id
//  add a new rel at correct location based on src and dst with new id
pub trait FileOps {
    /// For a given id, returns the node stored in the file system
    /// Involves looking for a `id.wnode` in `nodes` folder
    fn get_node(&self, id: String) -> WysgyResult<Node>;
    /// For a given id, returns the rel stored in the file system
    /// Involves looking for a `id.wrel` in subfolders of `rels` folder
    fn get_rel(&self, id: String) -> WysgyResult<Rel>;
    /// Loops through `*.wnode` in `nodes` folder and converts each file to a Node
    /// Returns the Vec<Node>
    fn get_all_nodes(&self) -> WysgyResult<Vec<Node>>;
    /// Loops through `*.wrel` in `rels` folder and converts each file to a Rel based on the path
    /// src > dst > id.wrel
    fn get_all_rels(&self) -> WysgyResult<Vec<Rel>>;
    /// Overwrites an existing node based on given Node
    fn save_node(&self, id: String, n: Node) -> WysgyResult<()>;
    /// Overwrites an existing edge based on given Rel
    fn save_rel(&self, id: String, r: Rel) -> WysgyResult<()>;
    /// Create and add a new node with the given key-value
    fn add_node(&self, kv: Value) -> WysgyResult<Node>;
    /// Create and add a new edge with given source and destination ids, and key-value
    fn add_rel(&self, kv: Value, src_id: String, dst_id: String) -> WysgyResult<Rel>;
    /// Delete a node with given id
    fn del_node(&self, id: String) -> WysgyResult<()>;
    /// Delete an edge with given id
    fn del_rel(&self, id: String) -> WysgyResult<()>;
    fn ext(s: String) -> String;
    fn nodes_dir(&self) -> PathBuf;
    fn create_dir(&self, name: &String) -> Result<(), Box<error::Error>>;
    fn create_file(&self, name: String, contents: &str) -> Result<(), Box<dyn error::Error>>;
    fn rel_dir(&self) -> PathBuf;
    fn new_node_id(&self) -> String;
    fn new_rel_id(&self, src_id: String, dst_id: String) -> String;
    fn node(&self, s: &String) -> String;
    fn file(&self, s: &String) -> String;
    fn rel(&self, s: &String, d: &String) -> Option<String>;
    fn delete(&self) -> std::io::Result<()>;
    fn create(name: String) -> Result<Self, Box<error::Error>>;
    fn open(name: String) -> Result<Self, Box<error::Error>>;
}

impl FileOps for Wysgy {
    fn get_node(&self, id: String) -> WysgyResult<Node> {
        let node_path = FileLoc::nodes_dir(self);
        node_path.push(format!("{}.wnode", id));
        let node_contents = fs::read_to_string(node_path)?;
        Ok({
            kv: Converter::kv_to_json(&node_contents, "\n")?,
            id: id
        })
    }

    fn get_rel(&self, id: String) -> WysgyResult<Rel> {
        let rel_name = format!("{}.wrel", id);
        FileLoc::apply_to_files(, |e|{e == })
    }

    fn add_node(&self, kv: Value) -> WysgyResult<Node> {
        let n = Node {
            id: FileOps::new_node_id(self),
            kv: kv,
        };
        Ok(n)
    }

    fn add_rel(&self, kv: Value, src_id: String, dst_id: String) -> WysgyResult<Rel> {
        let r = Rel {
            src_id: src_id,
            dst_id: dst_id,
            kv: kv,
            id: FileOps::new_rel_id(self, src_id, dst_id),
        };
        Ok(r)
    }
    
    fn new_node_id(&self) -> String {
        FileLoc::max_file_id(FileLoc::nodes_dir(self))
            .expect("No nodes id")
            .to_string()
    }

    fn new_rel_id(&self, src_id: String, dst_id: String) -> String {
        let path = FileLoc::rels_dir(self);
        path.push(src_id);
        path.push(dst_id);
        FileLoc::max_file_id(path)
            .expect("No rels id")
            .to_string()
    }



    fn ext(s: String) -> String {
        let new_s = s + &String::from("");
        new_s
    }

    fn nodes_dir(&self) -> PathBuf {
        let mut f = self.path.clone();
        f.push("nodes");
        f
    }

    fn rel_dir(&self) -> PathBuf {
        let mut f = self.path.clone();
        f.push("rels");
        f
    }

    fn node(&self, s: &String) -> String {
        let mut f = self.nodes_dir();
        info!("Finding largest id...");
        // find largest id in folder
        let max_id = Project::max_file_id(&(f.to_str().unwrap().to_string() + "/*.wnode"));
        let new_id = max_id + 1;
        info!("New node is {}", new_id);
        f.push(format!("{}.wnode", new_id));
        f.to_str().unwrap().to_string()
    }

    fn file(&self, s: &String) -> String {
        let mut f = PathBuf::from(&self.path);
        let fname = PathBuf::from(s.clone());
        f.push("files");
        f.push(fname.file_name().unwrap().to_str().unwrap());
        f.to_str().unwrap().to_string()
    }

    fn rel(&self, s: &String, d: &String) -> Option<String> {
        let mut r = self.rel_dir();
        r.push(s);
        r.push(d);
        if let Some(dir_to_make) = &r.to_str() {
            fs::create_dir(dir_to_make).unwrap();
            let max_id = Project::max_file_id(&(r.to_str().unwrap().to_string() + "*.wrel"));
            let new_id = max_id + 1;
            info!("New rel is {}", new_id);
            r.push(format!("{}.wrel", new_id));
            Some(r.to_str().unwrap().to_string())
        } else {
            info!("Failed to create rel: {} -> {}", s, d);
            None
        }
    }
}
