use node_create::Project;
use serde_json::json;

#[test]
fn it_creates_deletes_project() {
    match Project::create(String::from("test_notes_proj")) {
        Ok(p) => {
            p.delete();
        }
        Err(e) => panic!(
            "Project creation cannot fail. Maybe already exists? Error {}",
            e
        ),
    }
}

#[test]
fn it_adds_nodes_to_project() {
    Project::create(String::from("test2_notes_proj"));
    match Project::open(String::from("test2_notes_proj")) {
        Ok(p) => {
            p.add_json_node(&String::from("node1"));
            p.delete();
        }
        Err(e) => panic!("Adding nodes to project failed. Error {}", e),
    }
}

#[test]
fn it_fails_to_add_relationships_to_project() {
    Project::create(String::from("test3_notes_proj"));
    match Project::open(String::from("test3_notes_proj")) {
        Ok(p) => {
            match p.add_json_relationship(&String::from("node1"), &String::from("node2")) {
                Ok(_) => panic!("Adding relationships should fail since nodes do not exist"),
                Err(e) => {}
            } 
            p.delete();
        },
        Err(e) => panic!("Opening project failed. Error {}", e),
    }
}

#[test]
fn it_adds_relationships_to_project() {
    Project::create(String::from("test4_notes_proj"));
    match Project::open(String::from("test4_notes_proj")) {
        Ok(p) => {
            p.add_json_node(&String::from("node1"));
            p.add_json_node(&String::from("node2"));
            match p.add_json_relationship(&String::from("node1"), &String::from("node2")) {
                Ok(_) => {},
                Err(e) => panic!("Adding relationships to project failed. Error {}", e),
            } 
            p.delete();
        },
        Err(e) => panic!("Opening project failed. Error {}", e),
    }
}
