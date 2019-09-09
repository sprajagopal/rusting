// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
use crate::{curses, project};
use clap::App;

#[allow(dead_code)]
pub fn cli() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let p = matches.value_of("INPUT").unwrap().to_string();
        project::Project::new(&p).unwrap();
    } else {
        curses::curses();
    }
}
