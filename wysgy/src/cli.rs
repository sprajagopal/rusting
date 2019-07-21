// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
use crate::project;
use clap::App;
use serde_json::Value;
use std::path::Path;
use wysgy_core::{self, Converter};

#[allow(dead_code)]
pub fn cli() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("file") {
        let prj = project::Project::curr().unwrap();
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
                debug!("File doesn't exist. {}", e);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("note") {
        let prj = project::Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        let label = matches.value_of("label").unwrap();
        let json_str = Converter::kv_to_json(&s, ";").unwrap();
        prj.add_json_node_with_data(&label.to_string(), &json_str)
            .unwrap();
    } else if let Some(matches) = matches.subcommand_matches("project") {
        let p = matches.value_of("INPUT").unwrap().to_string();
        project::Project::new(&p).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("link") {
        let src = matches.value_of("src").unwrap().to_string();
        let dst = matches.value_of("dst").unwrap().to_string();
        let prj = project::Project::curr().unwrap();
        prj.add_json_relationship(&src, &dst).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("showrel") {
        let prj = project::Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        let rjson: Option<Value>;
        if let Some(req) = matches.value_of("type") {
            rjson = Some(Converter::kv_to_json(&req.to_string(), ";").unwrap());
        } else {
            rjson = None;
        }
        prj.read_related_nodes(&s, &rjson).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("show") {
        let prj = project::Project::curr().unwrap();
        let s = matches.value_of("INPUT").unwrap().to_string();
        prj.read_node(&s).unwrap();
    }
}
