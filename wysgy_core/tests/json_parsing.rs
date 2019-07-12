use wysgy_core::file_to_dict;
use serde_json::{json, };

static TEST_FILES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/");

#[test]
fn it_parses_existing_file() {
    let fname = String::from(TEST_FILES_DIR) + &String::from("test_json_file.txt"); 
    if let Ok(l) = file_to_dict(&fname) {
        assert_eq!(l, json!({"key1": "value1", "key2" : "value2"}));
    } else {
        panic!("This file exists");
    }
}

#[test]
fn it_errors_on_nonexisting_file() {
    let fname = String::from(TEST_FILES_DIR) + &String::from("no_such_file.txt");
    if let Ok(l) = file_to_dict(&fname) {
        panic!(format!("This file ({}) does not exist", &l));
    } else {
    }
}
