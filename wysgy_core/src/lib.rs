use glob::glob;
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};
use serde_json::{json, Value};
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
    println!("  Formed json {}", raw);
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
    println!("  creating a file node {}", j);
    Ok(j)
}

pub struct Converter {}

impl Converter {
    pub fn kv_to_json(s: &String, delimiter: &str) -> Result<Value, Box<dyn error::Error>> {
        let args = s.split(delimiter).collect::<Vec<&str>>();
        let mut json_str = String::from("{");
        let mut aiter = args.iter().peekable();
        while let Some(i) = aiter.next() {
            let currarg = i.split(":").collect::<Vec<&str>>();
            if currarg.len() != 2 {
                Err(": is a delimiter and cannot part of a value in key-value pairs. Example \" key : valuehasa:somewhere \"")?
            } else {
                json_str.push_str(&format!("\"{}\":\"{}\"", currarg[0], currarg[1]));
            }
            if aiter.peek() == None {
                break;
            }
            json_str.push_str(",");
        }
        json_str.push_str("}");
        Ok(serde_json::from_str(&json_str).unwrap())
    }

    fn json_to_table(j: &Value) -> Result<Table, Box<dyn error::Error>> {
        let mut table = Table::new();
        for (k, v) in j.as_object().unwrap().iter() {
            table.add_row(Row::new(vec![
                Cell::new(k.as_str()),
                Cell::new(v.as_str().unwrap()),
            ]));
        }
        Ok(table)
    }
}

pub struct Project {
    path: PathBuf,
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

    fn create_dir(&self, name: &String) -> Result<(), Box<error::Error>> {
        let mut path = self.path.clone();
        path.push(name);
        if let Some(dir_to_make) = &path.to_str() {
            fs::create_dir(dir_to_make)?;
            println!("Directory created {}", dir_to_make);
            Ok(())
        } else {
            Result::Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "absolute path to string conversion failed.",
            )))
        }
    }

    pub fn create(name: String) -> Result<Project, Box<error::Error>> {
        let path = PathBuf::from(Project::ext(name));
        if let Some(dir_to_make) = &path.to_str() {
            fs::create_dir(dir_to_make)?;
            println!("Directory created {}", dir_to_make);
            let p = Project { path: path };
            p.create_dir(&String::from("nodes"))?;
            p.create_dir(&String::from("rels"))?;
            p.create_dir(&String::from("files"))?;
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

        Ok(Project { path: abs_path })
    }

    pub fn delete(&self) -> std::io::Result<()> {
        println!("Removing directory {:?}", &self.path);
        fs::remove_dir_all(&self.path)
    }

    pub fn add_json_node(&self, label: &String) -> Result<(), Box<error::Error>> {
        let path = PathBuf::from(self.node(label));
        println!("Creating node at {}", &path.to_str().unwrap());
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
        println!("Creating node at {}", &path.to_str().unwrap());
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
        println!("Creating file node ({}) at ({})", self.node(label), path);
        self.add_json_node_with_data(
            label,
            &serde_json::from_str(&format!("{{\"fname\": \"{}\"}}", fname)).unwrap(),
        )?;
        println!("opening file for editing: {}", self.node(label));
        editor(&self.node(label), "editor")?;
        Ok(())
    }

    fn rel_to_json(&self, src: &String, dst: &String) -> Result<Value, Box<dyn error::Error>> {
        println!("Reading file {}...", self.rel(src, dst));
        let fstr = fs::read_to_string(self.rel(src, dst))?;
        Ok(Converter::kv_to_json(&fstr, "\n")?)
    }

    fn node_to_json(&self, label: &String) -> Result<Value, Box<dyn error::Error>> {
        println!("Reading file {}...", self.node(label));
        let fstr = fs::read_to_string(self.node(label))?;
        Ok(Converter::kv_to_json(&fstr, "\n")?)
    }

    pub fn read_node(&self, label: &String) -> Result<(), Box<dyn error::Error>> {
        let rjson = self.node_to_json(label)?;
        Converter::json_to_table(&rjson)?.printstd();
        Ok(())
    }

    pub fn read_related_nodes(
        &self,
        label: &String,
        req: &Option<Value>,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut table = Table::new();
        let rels = format!("{}/{}*", self.rel_dir().to_str().unwrap(), label);
        for fl in glob(&rels)? {
            let currfile = fl?.to_str().unwrap().to_string();
            let path = PathBuf::from(currfile.clone());
            let dst = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split("_")
                .collect::<Vec<&str>>();
            let contents = fs::read_to_string(currfile.clone())?;
            let rjson = Converter::kv_to_json(&contents, "\n")?;
            table.add_row(row![dst[0], Converter::json_to_table(&rjson)?, dst[1]]);
        }
        table.printstd();
        Ok(())
    }

    fn is_node_exist(&self, label: &String) -> bool {
        PathBuf::from(self.node(&label)).exists()
    }

    pub fn add_json_relationship(
        &self,
        src: &String,
        dst: &String,
    ) -> Result<(), Box<error::Error>> {
        if self.is_node_exist(src) && self.is_node_exist(dst) {
            let path = PathBuf::from(self.rel(&src, &dst));
            println!(
                "Creating rel {}->{} at {}",
                src,
                dst,
                &path.to_str().unwrap()
            );
            fs::write(&path.to_str().unwrap(), format!("src:{}\ndst:{}", src, dst))?;
            Ok(())
        } else {
            Err(String::from("Src or dst node missing").into())
        }
    }
}
