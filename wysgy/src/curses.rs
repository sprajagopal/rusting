use crate::callbacks::Callbacks;
use crate::info::Info;
use crate::layouts::Layouts;
use crate::panes::Panes;
use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use gag::Gag;
use log::LevelFilter;
use log4rs;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::cmp::max;
use std::env::current_dir;
use std::error;
use sublime_fuzzy::best_match;
use textwrap::fill;
use wysgy_core::node::Node;

fn log_init() -> Result<(), Box<dyn error::Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {l} - {m}\n",
        )))
        .build("wysgy.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    info!("Hello, world!");

    Ok(())
}

pub fn curses() {
    match log_init() {
        Ok(a) => {}
        Err(a) => panic!("Error creating log file"),
    }

    let mut siv = Cursive::default();
    siv.add_layer(match Layouts::editable_node_list() {
        Ok(r) => r,
        Err(e) => {
            info!("{:?}", e);
            panic!("{:?}", e)
        }
    });

    // callback based change in view
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('r', |s| {
        s.pop_layer();
        s.add_layer(Layouts::new_rels_list().unwrap());
    });
    siv.add_global_callback('g', |s| {
        project::Project::curr().unwrap().export();
    });
    siv.add_global_callback('n', |s| {
        s.pop_layer();
        s.add_layer(Layouts::editable_node_list().unwrap());
    });
    siv.add_global_callback('?', |s| {
        Info::show_info(s);
    });
    Info::show_info(&mut siv);
    siv.run();
}

#[test]
fn it_creates_node_list() {
    log_init();
    info!("Node list view.");
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    s.add_layer(Layouts::editable_node_list().unwrap());
    s.run();
}

#[test]
fn it_creates_editable_node_list() {
    log_init();
    info!("Edit view of a node");
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    s.add_layer(Layouts::editable_node_list().unwrap());
    s.run();
}

#[test]
fn it_shows_node() {
    log_init();
    let node_name = std::env::args().nth(3).expect("no node name given");
    info!("Show view of node {}", node_name);
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    s.add_layer(Panes::show_node("show_node", &node_name).unwrap());
    s.run();
}

#[test]
fn it_shows_rels() {
    log_init();
    let node_name = std::env::args().nth(3).expect("no node name given");
    info!("Show relationships view of node {}", node_name);
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    s.add_layer(Panes::show_rels("showrels", "show_rels", &node_name).unwrap());
    s.run();
}

#[test]
fn it_creates_new_rels_list() {
    log_init();
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    s.add_layer(Layouts::new_rels_list().unwrap());
    s.run();
}
