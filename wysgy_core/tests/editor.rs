use node_create;

static TEST_FILES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/");

#[test]
fn it_errors_on_nonexisting_editor(){
    let fname = String::from(TEST_FILES_DIR) + &String::from("test_json_file.txt"); 
    let ed = String::from("noexist");
    if let Ok(_) = node_create::editor(&fname, &ed) {
        panic!(format!("This editor ({}) does not exist", &ed));
    }
}

