use std::collections::HashMap;

fn create_sample_string_int(h: &mut HashMap<String, i32>){
    let intarr = [1,2,3,4,5];
    let keyarr = [String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five")];
    for i in keyarr.iter().zip(intarr.iter()) {
        h.insert(i.0.to_string(), *i.1); // TODO: String reference to string is not through dereference
    }
    println!("  Constructed hashmap is {:?}", h);
}

fn construct_string_int(){
    println!("Function demonstrates creating a string key and int value hashmap.");
    let mut h = HashMap::new();
    h.insert(String::from("first"), 1);
    println!("  Constructed hashmap is {:?}", h);
}

fn construct_string_string(){
    println!("Function demonstrates creating string string hashmaps.");
    let mut h = HashMap::new();
    h.insert(String::from("key"), String::from("value"));
    println!("  Constructed hashmap is {:?}", h);
}

fn construct_int_int(){
    println!("Function demonstrates a int int hashmap.");
    let mut h = HashMap::new();
    h.insert(1,2);
    println!("  Constructed hashmap is {:?}", h);
}

fn fetching(){
    println!("Function demonstrates creating a string key and int value hashmap.");
    let mut h = HashMap::new();
    h.insert(String::from("first"), 1);
    println!("  Constructed hashmap is {:?}", h);
    let key = String::from("second");
    let val = h.get(&key); // reference is needed because of get function definition
    println!("  fetching key {} and found {:?}", key, val);
    match val {
        None => println!("      No value found"),
        Some(i) => println!("       {} found", i)
    }
}
 
fn entry(){
    println!("Function demonstrates entrying a value using or_entry.");
    let mut h : HashMap<String, i32> = HashMap::new();
    create_sample_string_int(&mut h);
    h.insert(String::from("one"), 2);
    println!("  Hashmap changed to {:?}", h);
    h.entry(String::from("twenty")).or_insert(3);
    println!("  Hashmap changed to {:?}", h);
    println!("  entry result was {:?}", h.entry(String::from("thirty")));
}

fn main() {
    println!("Hello, world!");
    construct_string_int();
    construct_string_string();
    construct_int_int();
    fetching();
    let mut h : HashMap<String, i32> = HashMap::new();
    create_sample_string_int(&mut h);
    entry();
}
