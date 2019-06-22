use serde_json::{json, Value};
use std::error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Child, Command};

pub fn editor(fname: &String, editor: &String) -> Result<Child, Box<error::Error>> {
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
    let mut ext = String::new();
    if vec.len() >= 2 {
        ext = String::from(vec[vec.len() - 1]);
    } else {
        ext = String::from("none");
    }
    let j = json!({"filetype" : ext, "path" : &fname});
    println!("  creating a file node {}", j);
    Ok(j)
}

pub struct Project {
    path: PathBuf,
}

impl Project {
    fn ext(s: String) -> String {
        let new_s = s + &String::from("");
        new_s
    }

    fn node(&self, s: &String) -> String {
        let mut f = self.path.clone();
        f.push("nodes");
        f.push(s.clone());
        f.to_str().unwrap().to_string()
    }

    fn file(&self, s: &String) -> String {
        let mut f = PathBuf::from(&self.path);
        f.push("files");
        f.push(s);
        f.to_str().unwrap().to_string()
    }

    fn rel(&self, s: &String, d: &String) -> String {
        let mut r = self.path.clone();
        r.push("rels");
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
        let mut path = PathBuf::from(Project::ext(name));
        if let Some(dir_to_make) = &path.to_str() {
            fs::create_dir(dir_to_make)?;
            println!("Directory created {}", dir_to_make);
            let p = Project { path: path };
            p.create_dir(&String::from("nodes"));
            p.create_dir(&String::from("rels"));
            p.create_dir(&String::from("files"));
            Ok(p)
        } else {
            Result::Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "absolute path to string conversion failed.",
            )))
        }
    }

    pub fn open(name: String) -> Result<Project, Box<error::Error>> {
        let mut path = PathBuf::from(Project::ext(name));

        // check that path exists
        let abs_path = path.canonicalize()?;

        Ok(Project { path: abs_path })
    }

    pub fn delete(&self) -> std::io::Result<()> {
        println!("Removing directory {:?}", &self.path);
        fs::remove_dir_all(&self.path)
    }

    pub fn add_json_node(&self, label: &String) -> Result<(), Box<error::Error>> {
        let mut path = PathBuf::from(self.node(label));
        println!("Creating node at {}", &path.to_str().unwrap());
        fs::write(&path.to_str().unwrap(), "")?;
        Ok(())
    }

    pub fn add_json_node_with_data(
        &self,
        label: &String,
        kv: &String,
    ) -> Result<(), Box<error::Error>> {
        let mut path = PathBuf::from(self.node(label));
        println!("Creating node at {}", &path.to_str().unwrap());
        fs::write(&path.to_str().unwrap(), kv)?;
        Ok(())
    }

    pub fn add_file_node(&self, label: &String, fname: &String) -> Result<(), Box<error::Error>> {
        let path = self.file(fname);
        fs::copy(fname.clone(), path.clone())?;
        println!("Creating file node ({}) at ({})", label, path);
        self.add_json_node_with_data(label, &format!("fname: {}", fname));
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
            let mut path = PathBuf::from(self.rel(&src, &dst));
            println!(
                "Creating rel {}->{} at {}",
                src,
                dst,
                &path.to_str().unwrap()
            );
            fs::write(&path.to_str().unwrap(), "");
            Ok(())
        } else {
            Err(String::from("Src or dst node missing").into())
        }
    }
}
