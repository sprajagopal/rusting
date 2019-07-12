use wysgy_core;

static TEST_FILES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/");

#[test]
fn it_errors_on_nonexisting_editor(){
    let fname = String::from(TEST_FILES_DIR) + &String::from("test_json_file.txt"); 
    let ed = String::from("noexist");
    if let Ok(_) = wysgy_core::editor(&fname, &ed) {
        panic!(format!("This editor ({}) does not exist", &ed));
    }
}

