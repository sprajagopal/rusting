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

pub struct Panes {}

impl Panes {
    pub fn searchable_nodes(id: String, title: &str) -> Dialog {
        let eview = Dialog::around(EditView::new().on_edit(move |s, e, u| {
            info!("submit: {}", e);
            let nodes = project::Project::nodes(None).unwrap();
            info!("list of nodes found");
            let mut tmp = nodes
                .iter()
                .map(|n| {
                    let score = match best_match(e, &n.label) {
                        None => 0,
                        Some(a) => a.score(),
                    };
                    (n, score)
                })
                .collect::<Vec<(&Node, isize)>>();
            tmp.sort_by(|a, b| b.1.cmp(&a.1));
            info!("finding id now... {}", id);
            let mut sv = s.find_id::<SelectView<Node>>(&id.clone()).unwrap();
            sv.clear();
            debug!("length of nodes vec: {}", tmp.len());
            for i in tmp.iter().take(5) {
                sv.add_item(i.0.clone().label, i.0.clone());
            }
        }))
        .title(title);
        eview
    }

    pub fn editable_node(name: String, id: String, title: &str) {
        let eview = Dialog::around(EditView::new().on_submit(move |s, e| {}))
            .title(title)
            .button("save", |s| {});
    }
}
