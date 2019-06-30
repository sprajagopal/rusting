use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ListView, SelectView};
use cursive::Cursive;

fn get_node(s: &mut Cursive, v: &String) {
    println!("{}", v);
}

pub fn curses() {
    fn dummy_button(s: &mut Cursive) {}
    let nodes = project::Project::nodes().unwrap();
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::around(
        SelectView::<String>::new()
            .on_submit(get_node)
            .with(|list| {
                for n in nodes {
                    list.add_item(n.clone(), n.clone());
                    println!("{}", &n);
                }
            })
            .scrollable(),
    ));
    siv.run();
}
