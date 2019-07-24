use crate::project;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::Effect;
use cursive::theme::Style;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use sublime_fuzzy::best_match;
use textwrap::{fill, indent};
use wysgy_core::Node;
pub struct Panes {}

impl Panes {
    pub fn show_node(id: &str, title: &str, label: &str) -> Dialog {
        // read file contents of node "label"
        let n = project::Project::curr()
            .unwrap()
            .get_node(&label.to_string())
            .unwrap();
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
        let tview = Dialog::around(TextView::new(styled_label)).button("close", |s| {
            s.pop_layer();
        });
        tview
    }

    pub fn searchable_nodes(id: String, title: &str) -> Dialog {
        let eview = Dialog::around(EditView::new().on_edit(move |s, e, _u| {
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
            info!("length of nodes vec: {}", tmp.len());
            tmp = tmp.into_iter().filter(|(n, s)| s.clone() != 0).collect();
            for i in tmp.iter().take(5) {
                sv.add_item(i.0.clone().label, i.0.clone());
            }
        }))
        .title(title);
        eview
    }

    pub fn editable_node(_name: String, _id: String, title: &str) {
        let _eview = Dialog::around(EditView::new().on_submit(move |_s, _e| {}))
            .title(title)
            .button("save", |_s| {});
    }
}
