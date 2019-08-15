use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;
use std::cmp::max;
use textwrap::fill;
use wysgy_core::Node;

pub struct Callbacks {}

impl Callbacks {
    pub fn confirm_delete(s: &mut Cursive, f: fn(&mut Cursive)) {
        s.add_layer(
            Dialog::around(TextView::new("Confirm delete {}"))
                .button("no", |s| {
                    s.pop_layer();
                })
                .button("ok", move |s| {
                    info!("deleting node");
                    f(s);
                    s.pop_layer();
                }),
        );
    }

    pub fn get_node(s: &mut Cursive, v: &Node, kview: &str, lview: &str) {
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

    pub fn edit_active_node(s: &mut Cursive, n: &Node) {
        project::Project::edit_node(&n.label);
        let new_n = project::Project::update_node(&n.label).expect("No node found to update");
        let mut sview = s
            .find_id::<SelectView<Node>>("selection")
            .expect("No view found");
        let id = sview.selected_id().expect("No selection id");
        match sview.get_item_mut(id) {
            None => {
                debug!("{}", id);
            }
            Some(_curr_n) => {
                let _node = new_n.clone();
            }
        }
    }

    pub fn node_new(s: &mut Cursive) {
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
