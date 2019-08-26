use crate::callbacks::Callbacks;
use crate::panes::Panes;
use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use std::error;
use std::rc::Rc;
use wysgy_core::Node;

pub struct Layouts {}

impl Layouts {
    pub fn editable_node_list() -> Result<Dialog, Box<dyn error::Error>> {
        info!("Creating editable node list");
        let nodes = project::Project::nodes(None).unwrap();
        let _hpanes = LinearLayout::horizontal();
        let mut panes = LinearLayout::vertical();
        let all_nodes_view = Dialog::around(
            SelectView::<Node>::new()
                .with(|list| {
                    for n in &nodes {
                        list.add_item(n.clone().label, n.clone());
                    }
                })
                .on_submit(|s, e| {
                    info!("Selecting {}", e.label.clone());
                    s.add_layer(Panes::show_node(&e.label.clone(), &e.label.clone()).unwrap());
                })
                .scrollable(),
        )
        .title("Nodes list");
        let search = Panes::searchable_nodes("to_edit".to_string(), "select node")?;
        // add a new node placeholder
        panes.add_child(search);

        fn get_label(s: &mut Cursive, id: &str) -> Option<String> {
            s.call_on_id(id, |v: &mut SelectView<Node>| {
                match v.selected_id() {
                    Some(selid) => {
                        let label = v.get_item(selid).unwrap().1.clone().label;
                        info!("Editing {}", label);
                        Some(label)
                    }
                    None => {
                        // create a new node now with this name
                        info!("No node found in selectview");
                        None
                    }
                }
            })
            .unwrap()
        }

        Ok(Dialog::around(panes)
            .button("edit", |s| {
                let label = get_label(s, "to_edit");
                info!("{:?}", label);
                match label {
                    Some(label) => {
                        project::Project::edit_node(&label);
                        info!("edit finished");
                    }
                    None => {
                        info!("fetching new node label");
                        s.call_on_id("to_edit_editview", |v: &mut EditView| match Rc::try_unwrap(
                            v.get_content(),
                        ) {
                            Ok(val) => project::Project::edit_node(&val),
                            Err(e) => project::Project::edit_node(&e),
                        });
                    }
                }
            })
            .button("new", |s| {
                info!("fetching new node label");
                s.call_on_id("to_edit_editview", |v: &mut EditView| match Rc::try_unwrap(
                    v.get_content(),
                ) {
                    Ok(val) => project::Project::edit_node(&val),
                    Err(e) => panic!("Error in unwrapping new node label"),
                });
            })
            .button("delete", |s| {
                Callbacks::confirm_delete(s, |s: &mut Cursive| {
                    info!("deleting node");
                    let label = get_label(s, "to_edit");
                    match label {
                        Some(label) => {
                            project::Project::curr().unwrap().remove_node(&label);
                        }
                        None => {}
                    }
                });
            }))
    }

    pub fn new_rels_list() -> Result<Dialog, Box<dyn error::Error>> {
        info!("Creating nodes list...");
        let mut panes = LinearLayout::horizontal();

        const id_sview_src: &str = "nlist/sview_src";
        const id_sview_dst: &str = "nlist/sview_dst";

        let eview_src = Panes::searchable_nodes(id_sview_src.to_string(), "src")?;
        let eview_dst = Panes::searchable_nodes(id_sview_dst.to_string(), "dst")?;

        panes.add_child(eview_src);
        panes.add_child(DummyView);
        panes.add_child(eview_dst);

        fn create_dst_node_with_dst_id(s: &mut Cursive) -> String {
            // add new node, generate id with given text in search box
            let id = id_sview_dst.to_string() + "_editview";
            info!("Looking for {}", id);
            s.call_on_id(&id, |e: &mut EditView| {
                let newlabel = format!("{}", e.get_content());
                project::Project::curr().unwrap().edit_node(&newlabel);
                newlabel
            })
            .expect(&format!("Call on id for {} failed", id))
        }

        fn get_label_of_sview(s: &mut Cursive, id: &str) -> Option<String> {
            s.call_on_id(id, |v: &mut SelectView<Node>| match v.selected_id() {
                Some(selid) => Some(v.get_item(selid).unwrap().1.clone().label),
                None => {
                    info!("src label not found for adding new rels");
                    None
                }
            })
            .unwrap()
        }

        fn get_src_dst(s: &mut Cursive, id_ssrc: &str, id_sdst: &str) -> (String, String) {
            info!("call on button");
            let src_label = get_label_of_sview(s, id_ssrc).unwrap();
            let dst_label =
                get_label_of_sview(s, id_sdst).unwrap_or_else(|| create_dst_node_with_dst_id(s));
            (src_label, dst_label)
        }

        fn create_rel(s: &mut Cursive, src_label: &String, dst_label: &String) {
            info!("{:?} - {:?}", src_label, dst_label);
            project::Project::curr()
                .unwrap()
                .add_json_relationship(src_label, dst_label);
        }

        Ok(Dialog::around(panes)
            .button("create rel", |s| {
                let (src_label, dst_label) = get_src_dst(s, id_sview_src, id_sview_dst);
                create_rel(s, &src_label, &dst_label);
            })
            .button("delete rel", |s| {
                Callbacks::confirm_delete(s, |s: &mut Cursive| {
                    let (src_label, dst_label) = get_src_dst(s, id_sview_src, id_sview_dst);
                    info!("{:?} - {:?}", src_label, dst_label);
                    project::Project::curr()
                        .unwrap()
                        .remove_rel(&src_label, &dst_label);
                });
            })
            .button("new dst node", |s| {
                let dst_label = create_dst_node_with_dst_id(s);
                let src_label = get_label_of_sview(s, id_sview_src).unwrap();
                create_rel(s, &src_label, &dst_label);
            }))
    }

    fn refresh(s: &mut Cursive) {
        Layouts::pop_all(s);
        Layouts::add_all(s);
    }

    pub fn pop_all(s: &mut Cursive) {
        s.pop_layer();
    }

    pub fn add_by_types(s: &mut Cursive) {
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

    pub fn add_all(s: &mut Cursive) {
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
