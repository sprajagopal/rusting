pub mod converter;
mod files;
pub mod node;
use glob::glob;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate log;
use crate::converter::Converter;
use crate::files::{CONFIG_JSON_CONTENTS, GV_TEMPLATE_CONTENTS};
use crate::node::Node;
use prettytable::Table;
use serde_json::{json, Value};
use std::cmp::max;
use std::error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::PathBuf;
use std::process::{Child, Command};

pub fn editor(fname: &str, editor: &str) -> Result<Child, Box<error::Error>> {
    let mut cmd = Command::new(editor).arg(fname).spawn()?;
    cmd.wait()?;
    Ok(cmd)
}

pub fn file_to_string(fname: &String) -> Result<String, Box<error::Error>> {
    let file = File::open(fname)?;
    let mut raw = String::new();
    raw.push_str("{");
    let mut iter = BufReader::new(file).lines().peekable();
    while let Some(Ok(line)) = iter.next() {
        let vec = line.split(":").collect::<Vec<&str>>();
        raw.push_str(&format!("\"{}\":\"{}\"", vec[0].trim(), vec[1].trim()));
        if let None = iter.peek() {
            break;
        }

        raw.push_str(",");
    }
    raw.push_str("}");
    info!("  Formed json {}", raw);
    Ok(raw)
}

pub fn file_to_dict(fname: &String) -> Result<Value, Box<error::Error>> {
    let raw = file_to_string(fname)?;
    let v = serde_json::from_str(&raw)?;
    Ok(v)
}

pub fn existing_file_node(fname: &String) -> Result<Value, Box<error::Error>> {
    let vec = fname.split(".").collect::<Vec<&str>>();
    let ext;
    if vec.len() >= 2 {
        ext = String::from(vec[vec.len() - 1]);
    } else {
        ext = String::from("none");
    }
    let j = json!({"filetype" : ext, "path" : &fname});
    info!("  creating a file node {}", j);
    Ok(j)
}

pub struct Project {
    path: PathBuf,
    editor: String,
}

impl Project {
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
        f.push(s.clone());
        f.to_str().unwrap().to_string()
    }

    fn file(&self, s: &String) -> String {
        let mut f = PathBuf::from(&self.path);
        let fname = PathBuf::from(s.clone());
        f.push("files");
        f.push(fname.file_name().unwrap().to_str().unwrap());
        f.to_str().unwrap().to_string()
    }

    fn rel(&self, s: &String, d: &String) -> String {
        let mut r = self.rel_dir();
        r.push(format!("{}_{}", s.clone(), d));
        r.to_str().unwrap().to_string()
    }

    pub fn remove_node(&self, label: &String) -> Result<(), Box<dyn error::Error>> {
        fs::remove_file(self.node(label))?;
        Ok(())
    }

    pub fn remove_rel(&self, src: &String, dst: &String) -> Result<(), Box<dyn error::Error>> {
        fs::remove_file(self.rel(src, dst))?;
        Ok(())
    }

    fn create_dir(&self, name: &String) -> Result<(), Box<error::Error>> {
        let mut path = self.path.clone();
        path.push(name);
        if let Some(dir_to_make) = &path.to_str() {
            fs::create_dir(dir_to_make)?;
            info!("Directory created {}", dir_to_make);
            Ok(())
        } else {
            Result::Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "absolute path to string conversion failed.",
            )))
        }
    }

    fn create_file(&self, name: String, contents: &str) -> Result<(), Box<dyn error::Error>> {
        let mut path = self.path.clone();
        path.push(&name);
        match path.to_str() {
            Some(file_to_make) => {
                fs::File::create(file_to_make)?;
                fs::write(file_to_make, contents)?;
                Ok(())
            }
            None => panic!("Cannot create file: {}", &contents),
        }
    }

    pub fn create(name: String) -> Result<Project, Box<error::Error>> {
        let path = PathBuf::from(Project::ext(name));
        if let Some(dir_to_make) = &path.to_str() {
            fs::create_dir(dir_to_make).expect(&format!(
                "Project folder \"{}\" already exists.",
                dir_to_make
            ));
            info!("Directory created {}", dir_to_make);
            let p = Project {
                path: path,
                editor: String::from("gedit"),
            };
            p.create_dir(&String::from("nodes"))?;
            p.create_dir(&String::from("rels"))?;
            p.create_dir(&String::from("files"))?;
            p.create_file(String::from("config.json"), CONFIG_JSON_CONTENTS)?;
            p.create_file(String::from("gv.template"), GV_TEMPLATE_CONTENTS)?;
            Ok(p)
        } else {
            Result::Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "absolute path to string conversion failed.",
            )))
        }
    }

    pub fn open(name: String) -> Result<Project, Box<error::Error>> {
        let path = PathBuf::from(Project::ext(name));

        // check that path exists
        let abs_path = path.canonicalize()?;

        Ok(Project {
            path: abs_path,
            editor: String::from("gedit"),
        })
    }

    pub fn delete(&self) -> std::io::Result<()> {
        info!("Removing directory {:?}", &self.path);
        fs::remove_dir_all(&self.path)
    }

    pub fn add_json_node(&self, label: &String) -> Result<(), Box<error::Error>> {
        let path = PathBuf::from(self.node(label));
        info!("Creating node at {}", &path.to_str().unwrap());
        fs::write(&path.to_str().unwrap(), "")?;
        Ok(())
    }

    fn remove_quotes(s: &String) -> String {
        let rlen = s.len() - 1;
        let res = &s.clone()[1..rlen];
        res.to_string()
    }

    pub fn add_json_node_with_data(
        &self,
        label: &String,
        j: &Value,
    ) -> Result<(), Box<error::Error>> {
        let path = PathBuf::from(self.node(label));
        info!("Creating node at {}", &path.to_str().unwrap());
        let kv = j
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| format!("{}:{}", k, Project::remove_quotes(&v.to_string())))
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(&path.to_str().unwrap(), kv)?;
        Ok(())
    }

    pub fn add_file_node(&self, label: &String, fname: &String) -> Result<(), Box<error::Error>> {
        let path = self.file(fname);
        fs::copy(fname.clone(), path.clone())?;
        info!("Creating file node ({}) at ({})", self.node(label), path);
        self.add_json_node_with_data(
            label,
            &serde_json::from_str(&format!("{{\"fname\": \"files/{}\"}}", label)).unwrap(),
        )?;
        info!("opening file for editing: {}", self.node(label));
        editor(&self.node(label), &self.editor)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn rel_to_json(&self, src: &String, dst: &String) -> Result<Value, Box<dyn error::Error>> {
        let fstr = fs::read_to_string(self.rel(src, dst))?;
        Ok(Converter::kv_to_json(&fstr, "\n")?)
    }

    fn node_to_json(&self, label: &String) -> Result<Value, Box<dyn error::Error>> {
        let fstr = fs::read_to_string(self.node(label))?;
        Ok(Converter::kv_to_json(&fstr, "\n")?)
    }

    pub fn get_node_type(&self, label: &String) -> Option<String> {
        let j = self.node_to_json(label).unwrap();
        let filt = j
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _v)| k.to_string() == "type");
        let mut fin = filt.map(|(k, _v)| k.clone());
        fin.next()
    }

    pub fn read_node(&self, label: &String) -> Result<(), Box<dyn error::Error>> {
        let rjson = self.node_to_json(label)?;
        Converter::json_to_table(&rjson)?.printstd();
        Ok(())
    }

    pub fn get_node(&self, label: &String) -> Result<Node, Box<dyn error::Error>> {
        debug!("get node: {}", label);
        Ok(self.node_name_to_struct(label))
    }

    fn fetch_nodes(
        &self,
        glob_filter: String,
        _req: &Option<Value>,
        label_cb: &Fn(PathBuf) -> (String, String),
    ) -> Result<Vec<(Node, String)>, Box<dyn error::Error>> {
        let v = glob(&glob_filter)?
            .map(|fl| {
                let currfile = fl.unwrap().to_str().unwrap().to_string();
                let path = PathBuf::from(currfile.clone());
                let label = label_cb(path);
                let contents = fs::read_to_string(currfile.clone()).unwrap();
                let rjson = Converter::kv_to_json(&contents, "\n").unwrap();
                (
                    Node {
                        label: label.0,
                        kv: rjson,
                    },
                    label.1,
                )
            })
            .collect::<Vec<(Node, String)>>();
        Ok(v)
    }

    pub fn fetch_related_nodes(
        &self,
        label: &String,
        _req: &Option<Value>,
    ) -> Result<Vec<(Node, String)>, Box<dyn error::Error>> {
        let onodes_glob = format!("{}/{}_*", self.rel_dir().to_str().unwrap(), label);
        let inodes_glob = format!("{}/*_{}", self.rel_dir().to_str().unwrap(), label);
        let mut onodes = self.fetch_nodes(onodes_glob, _req, &|p: PathBuf| {
            let fname = p
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split("_")
                .collect::<Vec<&str>>();
            return (fname[1].to_string(), String::from(" >"));
        })?;
        let mut inodes = self.fetch_nodes(inodes_glob, _req, &|p: PathBuf| {
            let fname = p
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split("_")
                .collect::<Vec<&str>>();
            return (fname[0].to_string(), String::from(" <"));
        })?;
        onodes.append(&mut inodes);
        info!("related nodes: {:?}", onodes);
        Ok(onodes)
    }

    pub fn read_related_nodes(
        &self,
        label: &String,
        req: &Option<Value>,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut table = Table::new();
        for n in self.fetch_related_nodes(label, req)? {
            table.add_row(row![Converter::json_to_table(&n.0.kv)?]);
        }
        table.printstd();
        Ok(())
    }

    fn is_node_exist(&self, label: &String) -> bool {
        PathBuf::from(self.node(&label)).exists()
    }

    pub fn edit_node(&self, label: &String) {
        match editor(
            &PathBuf::from(self.node(&label)).to_str().unwrap(),
            &self.editor,
        ) {
            Ok(_e) => {}
            Err(_e) => {}
        }
    }

    pub fn add_json_relationship(
        &self,
        src: &String,
        dst: &String,
    ) -> Result<(), Box<error::Error>> {
        if self.is_node_exist(src) && self.is_node_exist(dst) {
            let path = PathBuf::from(self.rel(&src, &dst));
            info!(
                "Creating rel {}->{} at {}",
                src,
                dst,
                &path.to_str().unwrap()
            );
            if !PathBuf::from(path.clone()).exists() {
                fs::write(&path.to_str().unwrap(), format!("src:{}\ndst:{}", src, dst))?;
            }
            editor(&path.to_str().unwrap(), &self.editor)?;
            Ok(())
        } else {
            Err(String::from("Src or dst node missing").into())
        }
    }

    fn node_name_to_struct(&self, name: &String) -> Node {
        Node {
            label: name.to_string(),
            kv: self.node_to_json(name).unwrap(),
        }
    }

    fn node_mapper(&self, e: &std::result::Result<std::path::PathBuf, glob::GlobError>) -> Node {
        let label = e
            .as_ref()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        self.node_name_to_struct(&label)
    }

    pub fn nodes_list(&self, t: Option<String>) -> Result<Vec<Node>, Box<error::Error>> {
        let nodes_path = self.nodes_dir().to_str().unwrap().to_string();
        let nodes_all: String = nodes_path + &String::from("/*");

        let res = glob(&nodes_all)
            .unwrap()
            .collect::<Vec<std::result::Result<std::path::PathBuf, glob::GlobError>>>();
        let res_vec = res
            .into_iter()
            .map(|e| self.node_mapper(&e))
            .collect::<Vec<Node>>();
        match t {
            None => Ok(res_vec),
            Some(t) => Ok(res_vec.into_iter().filter(|n| n.kv["type"] == t).collect()),
        }
    }

    pub fn rels_list(&self, t: Option<Value>) -> Result<Vec<Node>, Box<error::Error>> {
        let rels_path = self.nodes_dir().to_str().unwrap().to_string();
        let rels_glob = format!("{}/*", self.rel_dir().to_str().unwrap());
        let mut rels = self.fetch_nodes(rels_glob, &t, &|p: PathBuf| {
            let fname = p
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split("_")
                .collect::<Vec<&str>>();
            (fname[1].to_string(), String::from(""))
        })?;
        Ok(rels.iter().map(|(a, b)| a.clone()).collect::<Vec<Node>>())
    }

    pub fn types_list(&self) -> Result<Vec<String>, Box<error::Error>> {
        let mut res_vec = self
            .nodes_list(None)
            .unwrap()
            .into_iter()
            .map(|n| {
                let s = n.kv["type"].to_string();
                let len = s.len();
                s[1..len - 1].trim().to_string()
            })
            .collect::<Vec<String>>();
        res_vec.sort();
        res_vec.dedup();
        Ok(res_vec)
    }

    pub fn export(&self) -> Result<(), Box<error::Error>> {
        let nodes_jstr = self
            .nodes_list(None)
            .unwrap()
            .into_iter()
            .map(|n| {
                format!(
                    "{} \"id\" : \"{}\", \"n\" : {} {}",
                    "{",
                    n.label.clone(),
                    n.to_string(),
                    "}"
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let rels_jstr = self
            .rels_list(None)
            .unwrap()
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let config_jstr = fs::read_to_string("config.json")?;
        info!("config: {}", config_jstr);
        let jstr = format!(
            "{} \"config\":{}, \"nodes\": [{}], \"rels\": [{}] {}",
            "{", config_jstr, nodes_jstr, rels_jstr, "}"
        );
        info!("{}", jstr);

        // get project name for apt naming
        let stem = self.path.file_stem().unwrap().to_str().unwrap().to_string();
        fs::write(format!("{}.json", stem), jstr);
        Command::new("jinja2")
            .arg("gv.template")
            .arg(format!("{}.json", stem))
            .arg("--format=json")
            .arg("-o")
            .arg(format!("{}.gv", stem))
            .output()
            .expect("gv gen failed");
        Command::new("dot")
            .arg(format!("{}.gv", stem))
            .args(&["-Kfdp", "-n", "-Tpdf"])
            .arg("-o")
            .arg(format!("{}.pdf", stem))
            .output()
            .expect("svg gen failed");
        info!("Generated svg");
        Ok(())
    }
}
