// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
#[macro_use]
extern crate clap;
use clap::App;
use node_create;
use serde_json::{json, Value};
use std::env;
use std::error;
use std::path::{Path, PathBuf};
use std::process::exit;

struct Project;
impl Project {
    pub fn open(s: &String) -> Result<node_create::Project, Box<error::Error>> {
        println!("Opening project path: {}", s);
        let prj = node_create::Project::open(s.to_string())?;
        Ok(prj)
    }

    pub fn new(s: &String) -> Result<node_create::Project, Box<error::Error>> {
        let prj = node_create::Project::create(s.to_string())?;
        Ok(prj)
    }

    pub fn curr() -> Result<node_create::Project, Box<error::Error>> {
        let pwd = env::current_dir().unwrap();
        let path = pwd.canonicalize().unwrap().to_str().unwrap().to_string();
        println!("Project path: {}", path);
        if PathBuf::from(path.clone()).exists() {
            Project::open(&path)
        } else {
            Project::new(&path)
        }
    }
}

fn kv_to_json(s: &String) -> Result<Value, Box<dyn error::Error>> {
    let args = s.split(";").collect::<Vec<&str>>();
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
    println!("{}", json_str);
    Ok(serde_json::from_str(&json_str).unwrap())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("file") {
        let prj = Project::curr().unwrap();
        println!("Using input file: {}", matches.value_of("INPUT").unwrap());
        let f = matches.value_of("INPUT").unwrap();
        let flabel = Path::new(f)
            .file_stem() // only the file name is needed
            .unwrap() // unwrap the Option to Osstr
            .to_str() // convert to Option<&str>
            .unwrap() // unwrap the option
            .to_string(); // convert to string
        match prj.add_file_node(&flabel, &f.to_string()) {
            Ok(_) => {}
            Err(e) => {
                println!("File doesn't exist. {}", e);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("json") {
        let prj = Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        let label = matches.value_of("label").unwrap();
        let json_str = kv_to_json(&s).unwrap();
        prj.add_json_node_with_data(&label.to_string(), &json_str);
    } else if let Some(matches) = matches.subcommand_matches("project") {
        let p = matches.value_of("INPUT").unwrap().to_string();
        Project::new(&p);
    } else if let Some(matches) = matches.subcommand_matches("rel") {
        let src = matches.value_of("src").unwrap().to_string();
        let dst = matches.value_of("dst").unwrap().to_string();
        let prj = Project::curr().unwrap();
        prj.add_json_relationship(&src, &dst).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("cnts") {
        let prj = Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        let json_str = kv_to_json(&s).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("show") {
        let prj = Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        println!("Reading file nodes {}", s);
        prj.read_file_node(&s).unwrap();
    }
}
