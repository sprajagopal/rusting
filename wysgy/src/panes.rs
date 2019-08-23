use crate::project;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::Effect;
use cursive::theme::Style;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use serde_json::Value;
use std::error;
use sublime_fuzzy::best_match;
use textwrap::{fill, indent};
use wysgy_core::Node;
pub struct Panes {}

impl Panes {
    pub fn show_rels(id: &str, title: &str, label: &str) -> Result<Dialog, Box<dyn error::Error>> {
        let rel_nodes = project::Project::curr()?.fetch_related_nodes(&label.to_string(), &None)?;
        let tview_id = "tview_node";
        let mut hlayout = LinearLayout::horizontal();
        let tview = TextView::new("").with_id(tview_id);
        let sview = SelectView::<Node>::new()
            .with(|list| {
                for rn in rel_nodes {
                    list.add_item(
                        format!("{} {}", rn.1.clone(), &rn.0.clone().label),
                        rn.0.clone(),
                    );
                }
            })
            .on_select(move |s, e| {
                s.call_on_id(tview_id, |tv: &mut TextView| {
                    tv.set_content(Panes::style_node(&e.clone().label).unwrap());
                });
            })
            .on_submit(|s, e| {
                s.pop_layer();
                s.add_layer(
                    Panes::show_rels(
                        &format!("showrel_{}", e.clone().label),
                        &format!("showing relationships for {}", e.clone().label),
                        &e.clone().label,
                    )
                    .unwrap(),
                );
            })
            .with_id(id);
        Ok(
            Dialog::around(hlayout.child(sview).child(DummyView).child(tview))
                .title(format!("Showing rels of {}", label)),
        )
    }

    pub fn style_node(label: &str) -> Result<StyledString, Box<dyn error::Error>> {
        // read file contents of node "label"
        let n = project::Project::curr()?.get_node(&label.to_string())?;
        info!("{:?}", n);
        let mut styled_label = StyledString::plain("");
        let mut keylens: Vec<usize> =
            n.kv.as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| k.to_string().len())
                .collect();
        keylens.sort();
        let max_keylen = keylens[keylens.len() - 1];
        let key_width = max_keylen + 3;
        let val_width = 30;
        let val_indent = 0;
        info!("Max key length found : {}", max_keylen);

        for (k, v) in n.kv.as_object().unwrap().iter() {
            styled_label.append(StyledString::styled(
                format!("{:width$}\n", k.to_string() + ":", width = key_width),
                Style::from(Color::Dark(BaseColor::Red)).combine(Effect::Bold),
            ));
            let field = v.as_str().unwrap().clone().trim();
            let value = indent(&fill(field, val_width), &" ".repeat(val_indent));
            let padded_value = value;
            let len = padded_value.len();
            styled_label.append(StyledString::plain(padded_value + "\n"));
        }
        Ok(styled_label)
    }

    pub fn show_node(title: &str, label: &str) -> Result<Dialog, Box<dyn error::Error>> {
        let tview = Dialog::around(TextView::new(Panes::style_node(label)?)).button("close", |s| {
            s.pop_layer();
        });
        Ok(tview)
    }

    pub fn list_nodes(id: String, title: &str) -> Dialog {
        let nodes = project::Project::nodes(None).unwrap();
        let sview = SelectView::<Node>::new()
            .with(|list| {
                for n in nodes {
                    list.add_item(n.clone().label, n.clone());
                }
            })
            .on_submit(|s, e| {})
            .with_id(id);
        Dialog::around(sview)
    }

    pub fn searchable_nodes(id: String, title: &str) -> Result<Dialog, Box<dyn error::Error>> {
        pub fn style_kv(content: &Value) -> Result<StyledString, Box<dyn error::Error>> {
            let mut styled_label = StyledString::plain("");

            for (k, v) in content.as_object().unwrap().iter() {
                styled_label.append(StyledString::styled(
                    format!("{}:", k.to_string()),
                    Style::from(Effect::Bold),
                ));
                styled_label.append(StyledString::plain(format!(
                    "{}; ",
                    v.as_str().unwrap().clone().trim()
                )));
            }
            Ok(styled_label)
        }
        let nodes = project::Project::nodes(None).unwrap();
        project::Project::curr()?.export()?;
        let sview = SelectView::<Node>::new()
            .with(|list| {
                for n in nodes {
                    list.add_item(style_kv(&n.clone().kv).unwrap(), n.clone());
                }
            })
            .on_submit(|s, e| {
                info!("Selecting {}", e.label.clone());
                s.add_layer(Panes::show_node(&e.label.clone(), &e.label.clone()).unwrap());
            })
            .with_id(id.clone())
            .fixed_size((40, 20))
            .scrollable();

        let eview_id = id.clone() + "_editview";
        info!("eview_id: {}", eview_id);
        let eview = EditView::new()
            .on_edit(move |s, e, u| {
                let nodes = project::Project::nodes(None).unwrap();
                match s.find_id::<SelectView<Node>>(&id.clone()) {
                    Some(mut sv) => {
                        sv.clear();
                        if u == 0 {
                            // show all nodes
                            for n in &nodes {
                                sv.add_item(style_kv(&n.clone().kv).unwrap(), n.clone());
                            }
                        }
                        info!("submit: {}", e);
                        let mut tmp = nodes
                            .iter()
                            .map(|n| {
                                let score = match best_match(e, &n.to_string()) {
                                    None => 0,
                                    Some(a) => a.score(),
                                };
                                (n, score)
                            })
                            .collect::<Vec<(&Node, isize)>>();
                        tmp.sort_by(|a, b| b.1.cmp(&a.1));
                        info!("length of nodes vec: {}", tmp.len());
                        tmp = tmp.into_iter().filter(|(n, s)| s.clone() != 0).collect();

                        for i in tmp.iter().take(5) {
                            sv.add_item(style_kv(&i.0.clone().kv).unwrap(), i.0.clone());
                        }
                    }
                    None => debug!("id: {} NOT FOUND", id),
                }
            })
            .with_id(eview_id);

        let mut l = LinearLayout::vertical();
        l.add_child(sview);
        l.add_child(DummyView);
        l.add_child(eview);
        Ok(Dialog::around(l).title(title))
    }
}
