use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ListView, SelectView};
use cursive::Cursive;
use gag::Gag;
use serde_json::{json, Value};
use std::cmp::max;
use textwrap::fill;
use wysgy_core::{Converter, Node};

fn get_node(s: &mut Cursive, v: &Node) {
    let keywidth = 20;
    let valwidth = 40;

    fn filled_text(a: &String, b: &String, awidth: usize, bwidth: usize) -> String {
        let txt = fill(a, awidth);
        let anum = a.len() / awidth;
        let bnum = b.len() / bwidth;
        let newl = "\n".repeat(max(anum, bnum) - anum);
        txt + "\n\n" + &newl
    }

    match s.find_id::<TextView>("keyview") {
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

    match s.find_id::<TextView>("labelview") {
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
    get_node(s, n);
}

fn refresh(s: &mut Cursive) {
    pop_all(s);
    add_all(s);
}

fn pop_all(s: &mut Cursive) {
    s.pop_layer();
}

fn node_new(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(|s, e| {
                    let name = s
                        .call_on_id("node_new", |view: &mut EditView| view.get_content())
                        .unwrap();
                    let prj = project::Project::curr().unwrap();
                    prj.add_json_node(&name);
                    s.pop_layer();
                })
                .with_id("node_new")
                .fixed_width(10),
        )
        .title("Enter node name"),
    );
}

fn add_by_types(s: &mut Cursive) {
    s.add_global_callback('q', |s| s.quit());
    s.add_global_callback('r', |s| refresh(s));
    s.add_global_callback('n', |s| node_new(s));
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
                    get_node(s, &nodes[0]);
                }
            })
            .scrollable(),
    );
    panes.add_child(DummyView);
    panes.add_child(
        SelectView::<Node>::new()
            .on_select(get_node)
            .on_submit(edit_active_node)
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

fn add_all(s: &mut Cursive) {
    s.add_global_callback('q', |s| s.quit());
    s.add_global_callback('r', |s| refresh(s));
    s.add_global_callback('n', |s| node_new(s));
    let nodes = project::Project::nodes(None).unwrap();
    let mut panes = LinearLayout::horizontal();
    panes.add_child(
        SelectView::<Node>::new()
            .on_select(get_node)
            .on_submit(edit_active_node)
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

pub fn curses() {
    fn dummy_button(s: &mut Cursive) {}
    let mut siv = Cursive::default();

    let mut print_gag = Gag::stdout().unwrap();

    add_by_types(&mut siv);
}
