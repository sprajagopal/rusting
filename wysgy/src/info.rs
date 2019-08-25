use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::Cursive;
pub struct Info {}

impl Info {
    fn dialog(s: &str) -> Dialog {
        Dialog::around(TextView::new(s)).button("ok", |s| {
            s.pop_layer();
        })
    }

    pub fn show_info(s: &mut Cursive) {
        s.add_layer(Info::dialog("Wysgy is a GUI wrapper around graphviz. It has two primary views. The nodeview shows all the nodes and allows you to create more. The rels view shows the nodes side by side and allows you create edges between any two. \n\t\t? - display help\n\t\tr - create edges\n\t\tn - create nodes"))
    }

    pub fn show_node_create(s: &mut Cursive) {
        s.add_layer(Info::dialog("Nodes are key-value pairs. A valid node should be of this form:\n\t\tkey: value\n\t\tkey2:value2\n\t\t..."))
    }

    pub fn show_rel_create(s: &mut Cursive) {
        s.add_layer(Info::dialog("In this view, relationships can be created between nodes. After selecting src and dst node in the list, <create rel> opens a new edge file, which is a key-value pair i.e. key: value\nkey2: value2\n\n<delete rel> deletes any existing edge between selected nodes\n\n<new dst node> will create a new dst node with label given in the search bar of dst list."));
    }
}
