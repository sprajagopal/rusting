use node_create;

#[test]
fn it_creates_file_node() {
    if let Ok(_) = node_create::existing_file_node(&String::from("study.pdf")) {

    } else {
        panic!("This file exists and node should have been created!");
    }
}
