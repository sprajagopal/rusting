use crate::callbacks::Callbacks;
use crate::layouts::Layouts;
use crate::panes::Panes;
use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use gag::Gag;
use log4rs;
use std::cmp::max;
use std::error;
use sublime_fuzzy::best_match;
use textwrap::fill;
use wysgy_core::Node;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

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
    log_init();
    let mut siv = Cursive::default();
    Layouts::node_list(&mut siv);
}

#[test]
fn it_creates_node_list() {
    log_init();
    info!("Node list view.");
    let mut s = Cursive::default();
    s.add_global_callback('q', |s| s.quit());
    Layouts::node_list(&mut s);
    s.run();
}
