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
        panes.add_child(search);

        Ok(Dialog::around(panes).button("edit", |s| {
            let is_exist = s.call_on_id("to_edit", |v: &mut SelectView<Node>| {
                match v.selected_id() {
                    Some(selid) => {
                        let label = v.get_item(selid).unwrap().0.to_string();
                        info!("Editing {}", label);
                        project::Project::edit_node(&label);
                        Some(label)
                    }
                    None => {
                        // create a new node now with this name
                        info!("No node found in selectview");
                        None
                    }
                }
            });
            info!("{:?}", is_exist);
            match is_exist {
                Some(Some(label)) => {
                    info!("edit finished");
                }
                Some(None) => {
                    info!("fetching new node label");
                    s.call_on_id("to_edit_editview", |v: &mut EditView| match Rc::try_unwrap(
                        v.get_content(),
                    ) {
                        Ok(val) => project::Project::edit_node(&val),
                        Err(e) => project::Project::edit_node(&e),
                    });
                }
                None => {}
            }
        }))
    }

    pub fn new_rels_list() -> Result<Dialog, Box<dyn error::Error>> {
        info!("Creating nodes list...");
        let mut panes = LinearLayout::vertical();

        let id_sview_src = "nlist/sview_src";
        let id_sview_dst = "nlist/sview_dst";

        let eview_src = Panes::searchable_nodes(id_sview_src.to_string(), "src")?;
        let eview_dst = Panes::searchable_nodes(id_sview_dst.to_string(), "dst")?;

        panes.add_child(eview_src);
        panes.add_child(DummyView);
        panes.add_child(eview_dst);

        Ok(Dialog::around(panes).button("create rel", move |s| {
            info!("call on button");
            let src_label = s
                .call_on_id(id_sview_src, |v: &mut SelectView<Node>| {
                    match v.selected_id() {
                        Some(selid) => Some(v.get_item(selid).unwrap().0.to_string()),
                        None => {
                            info!("src label not found for adding new rels");
                            None
                        }
                    }
                })
                .unwrap()
                .unwrap();
            let dst_label = s
                .call_on_id(id_sview_dst, |v: &mut SelectView<Node>| {
                    match v.selected_id() {
                        Some(selid) => Some(v.get_item(selid).unwrap().0.to_string()),
                        None => {
                            info!("dst label not found for adding new rels");
                            None
                        }
                    }
                })
                .unwrap()
                .unwrap();
            info!("{:?} - {:?}", src_label, dst_label);
            project::Project::curr()
                .unwrap()
                .add_json_relationship(&src_label, &dst_label);
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
