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

struct Callbacks {}

impl Callbacks {
    fn get_node(s: &mut Cursive, v: &Node, kview: &str, lview: &str) {
        let keywidth = 20;
        let valwidth = 40;

        fn filled_text(a: &String, b: &String, awidth: usize, bwidth: usize) -> String {
            let txt = fill(a, awidth);
            let anum = a.len() / awidth;
            let bnum = b.len() / bwidth;
            let newl = "\n".repeat(max(anum, bnum) - anum);
            txt + "\n\n" + &newl
        }

        match s.find_id::<TextView>(kview) {
            None => {}
            Some(mut tv) => {
                let keystr =
                    v.kv.as_object()
                        .unwrap()
                        .iter()
                        .fold(String::new(), |acc, (k, v)| {
                            format!(
                                "{}{}",
                                acc,
                                filled_text(
                                    &(k.clone() + ":"),
                                    &v.as_str().unwrap().to_string(),
                                    keywidth,
                                    valwidth
                                )
                            )
                        });
                tv.set_content(keystr);
            }
        }

        match s.find_id::<TextView>(lview) {
            None => {}
            Some(mut tv) => {
                let keystr =
                    v.kv.as_object()
                        .unwrap()
                        .iter()
                        .fold(String::new(), |acc, (k, v)| {
                            format!(
                                "{}{}",
                                acc,
                                filled_text(
                                    &format!("{}", v.as_str().unwrap().to_string().trim()),
                                    &k,
                                    valwidth,
                                    keywidth
                                )
                            )
                        });
                tv.set_content(keystr);
            }
        }
    }

    fn edit_active_node(s: &mut Cursive, n: &Node) {
        project::Project::edit_node(&n.label);
        let new_n = project::Project::update_node(&n.label).expect("No node found to update");
        let mut sview = s
            .find_id::<SelectView<Node>>("selection")
            .expect("No view found");
        let id = sview.selected_id().expect("No selection id");
        match sview.get_item_mut(id) {
            None => {
                println!("{}", id);
            }
            Some(_curr_n) => {
                let _node = new_n.clone();
            }
        }
    }

    fn node_new(s: &mut Cursive) {
        s.add_layer(
            Dialog::around(
                EditView::new()
                    .on_submit(|s, _e| {
                        let name = s
                            .call_on_id("node_new", |view: &mut EditView| view.get_content())
                            .unwrap();
                        let prj = project::Project::curr().unwrap();
                        match prj.add_json_node(&name) {
                            Ok(_e) => {}
                            Err(_e) => {}
                        }
                        s.pop_layer();
                    })
                    .with_id("node_new")
                    .fixed_width(10),
            )
            .title("Enter node name"),
        );
    }
}

struct Layouts {}

impl Layouts {
    fn node_list(s: &mut Cursive) {
        info!("Creating nodes list...");
        let nodes = project::Project::nodes(None).unwrap();
        let mut hpanes = LinearLayout::horizontal();
        let mut panes = LinearLayout::vertical();
        let sview = Dialog::around(
            SelectView::<Node>::new()
                .with(|list| {
                    for n in nodes {
                        list.add_item(n.clone().label, n);
                    }
                })
                .on_select(|_s, _n| {})
                .scrollable(),
        )
        .title("Add a new relationship");
        let eview_src = Dialog::around(EditView::new().on_submit(|s, e| {
            let nodes = project::Project::nodes(None).unwrap();
            let mut tmp = nodes
                .iter()
                .map(|n| (n, best_match(e, &n.label).unwrap().score()))
                .collect::<Vec<(&Node, isize)>>();
            tmp.sort_by(|a, b| b.1.cmp(&a.1));
            let mut sv = s.find_id::<SelectView<Node>>("nlist/sview_src").unwrap();
            sv.clear();
            debug!("length of nodes vec: {}", tmp.len());
            for i in tmp.iter().take(5) {
                sv.add_item(i.0.clone().label, i.0.clone());
            }
        }))
        .title("Src node");

        let eview_dst = Dialog::around(EditView::new().on_submit(|_s, _e| {
            // check if node exists
        }))
        .title("Dst node");

        panes.add_child(sview);
        panes.add_child(DummyView);
        panes.add_child(eview_src);
        //  panes.add_child(DummyView);
        //  panes.add_child(eview_dst);

        let mut spanes = LinearLayout::vertical();
        let sview_src = SelectView::<Node>::new()
            .on_select(|_s, _e| {})
            .with_id("nlist/sview_src");
        let sview_dst = SelectView::<Node>::new()
            .on_select(|_s, _e| {})
            .with_id("nlist/sview_dst");
        spanes.add_child(sview_src);
        spanes.add_child(sview_dst);

        hpanes.add_child(panes);
        hpanes.add_child(DummyView);
        hpanes.add_child(spanes);

        s.add_layer(Dialog::around(hpanes));
        s.run();
    }

    fn refresh(s: &mut Cursive) {
        Layouts::pop_all(s);
        Layouts::add_all(s);
    }

    fn pop_all(s: &mut Cursive) {
        s.pop_layer();
    }

    fn add_by_types(s: &mut Cursive) {
        s.add_global_callback('q', |s| s.quit());
        s.add_global_callback('r', |s| Layouts::refresh(s));
        s.add_global_callback('n', |s| Callbacks::node_new(s));
        let types = project::Project::types().unwrap();
        let mut panes = LinearLayout::horizontal();
        panes.add_child(
            SelectView::<String>::new()
                .with(|list| {
                    for t in types {
                        list.add_item(t.clone(), t.clone());
                    }
                })
                .on_select(|s, val| {
                    let mut view = s.find_id::<SelectView<Node>>("selection").unwrap();
                    view.clear();
                    let nodes = project::Project::nodes(Some(val.to_string())).unwrap();
                    for n in &nodes {
                        view.add_item(n.clone().label, n.clone());
                    }
                    if nodes.len() != 0 {
                        Callbacks::get_node(s, &nodes[0], "keyview", "labelview");
                    }
                })
                .scrollable(),
        );
        panes.add_child(DummyView);
        panes.add_child(
            SelectView::<Node>::new()
                .on_select(|s, n| Callbacks::get_node(s, n, "keyview", "labelview"))
                .on_submit(Callbacks::edit_active_node)
                .with_id("selection")
                .scrollable(),
        );
        panes.add_child(DummyView);
        panes.add_child(TextView::new("KeyView").with_id("keyview").scrollable());
        panes.add_child(DummyView);
        panes.add_child(TextView::new("LabelView").with_id("labelview").scrollable());
        panes.add_child(DummyView);
        panes.add_child(
            TextView::new("ConnectView")
                .with_id("connectview")
                .scrollable(),
        );
        let mut layout = LinearLayout::vertical();
        layout.add_child(panes);

        s.add_layer(Dialog::around(layout));
        s.run();
    }

    fn add_all(s: &mut Cursive) {
        s.add_global_callback('q', |s| s.quit());
        s.add_global_callback('r', |s| Layouts::refresh(s));
        s.add_global_callback('n', |s| Callbacks::node_new(s));
        let nodes = project::Project::nodes(None).unwrap();
        let mut panes = LinearLayout::horizontal();
        panes.add_child(
            SelectView::<Node>::new()
                .on_select(|s, n| Callbacks::get_node(s, n, "keyview", "labelview"))
                .on_submit(Callbacks::edit_active_node)
                .with(|list| {
                    for n in nodes {
                        list.add_item(n.clone().label, n);
                    }
                })
                .with_id("selection")
                .scrollable(),
        );
        panes.add_child(DummyView);
        panes.add_child(TextView::new("KeyView").with_id("keyview").scrollable());
        panes.add_child(DummyView);
        panes.add_child(TextView::new("LabelView").with_id("labelview").scrollable());
        let mut layout = LinearLayout::vertical();
        layout.add_child(panes);

        s.add_layer(Dialog::around(layout));
        s.run();
    }
}

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
