// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
#[macro_use]
extern crate clap;
use clap::App;
use node_create;
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

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("file") {
        let prj = Project::curr().unwrap();
        println!("Using input file: {}", matches.value_of("INPUT").unwrap());
        let f = matches.value_of("INPUT").unwrap();
        let flabel = Path::new(f)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        match prj.add_file_node(&flabel.to_string(), &f.to_string()) {
            Ok(_) => {}
            Err(e) => {
                println!("File doesn't exist. {}", e);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("json") {
        let prj = Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap();
        let args = s.split(" ").collect::<Vec<&str>>();
        let mut a_iter = args.iter().peekable();

        if args.len() % 2 == 0 {
            let mut args_folded: Vec<(String, String)> = Vec::with_capacity(args.len() / 2);
            for i in (0..args.len()).step_by(2) {
                args_folded.push((args[i].to_string(), args[i + 1].to_string()));
            }

            let mut json_str = String::new();
            for i in args_folded {
                json_str.push_str(&format!("{}:{}\n", i.0.trim(), i.1.trim()));
            }
            prj.add_json_node_with_data(&String::from("node1"), &json_str);
        } else {
            println!("Odd length args");
            exit(0x0100);
        }
    } else if let Some(matches) = matches.subcommand_matches("project") {
        let p = matches.value_of("INPUT").unwrap().to_string();
        Project::new(&p);
    }

    // more program logic goes here...
}
