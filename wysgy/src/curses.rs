use crate::project;
use cursive::traits::*;
use cursive::views::TextView;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

pub fn curses() {
    let nodes = project::Project::nodes().unwrap();
    println!("{:?}", nodes);
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    let select = SelectView::<String>::new()
        .with_id("select")
        .fixed_size((10, 5));

    fn dummy_button(s: &mut Cursive) {}

    let mut vert_layout = LinearLayout::vertical();
    vert_layout = nodes.into_iter().fold(vert_layout, |acc, e| {
        acc.child(Button::new(e.clone(), dummy_button))
    });
    siv.add_layer(Dialog::around(vert_layout).title("wysgy nodes"));

    siv.run();
}
