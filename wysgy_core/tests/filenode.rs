use wysgy_core;

#[test]
fn it_creates_file_node() {
    if let Ok(_) = wysgy_core::existing_file_node(&String::from("study.pdf")) {

    } else {
        panic!("This file exists and node should have been created!");
    }
}
