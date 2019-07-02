use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ListView, SelectView};
use cursive::Cursive;
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
        txt + "\n" + &newl
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
                            filled_text(&v.as_str().unwrap().to_string(), &k, valwidth, keywidth)
                        )
                    });
            tv.set_content(keystr);
        }
    }
}

pub fn curses() {
    fn dummy_button(s: &mut Cursive) {}
    let nodes = project::Project::nodes().unwrap();
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let mut panes = LinearLayout::horizontal();
    panes.add_child(
        SelectView::<Node>::new()
            .on_select(get_node)
            .with(|list| {
                for n in nodes {
                    list.add_item(n.clone().label, n);
                }
            })
            .scrollable(),
    );
    panes.add_child(DummyView);
    panes.add_child(TextView::new("KeyView").with_id("keyview").scrollable());
    panes.add_child(DummyView);
    panes.add_child(TextView::new("LabelView").with_id("labelview").scrollable());
    siv.add_layer(Dialog::around(panes));
    siv.run();
}
