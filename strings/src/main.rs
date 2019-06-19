fn mutable_string(){
    println!("Function demonstrates mutability of string");
    let mut s = String::new();
    println!("  String was {}.", s);
    s = String::from("something something");
    println!("  String is now {}.", s);
}

fn concat(x: String, y: String) -> String {
    println!("Function demonstrates concatenation of string.");
    let s = x + &y; // second argument has to be reference because of implementation of '+' operator
    // THIS STATEMENT BORROWS AND WONT COMPILE ==> println!("  String {} and String {} combined to give {}", x, y, s);
    println!("  String result is {}", s);
    s
}

fn format(arr: [String; 3]) -> String {
    println!("Function demonstrates format function");
    let mut s = String::new();
    for i in &arr {
        s = format!("{} {}", s, *i);
    }
    println!("  Final string is {}", s);
    s
}

fn format_unknown_len(arr : Vec<String>) -> String {
    println!("Function demonstrates format function using a vector");
    let mut s = String::new();
    for i in &arr {
        s = format!("{} {}", s, *i);
    }
    println!("  Final string is {}", s);
    s
}

fn main() {
    mutable_string();
    let x = String::from("hello");
    let y = String::from("world");
    concat(x, y);
    let arr : [String; 3] = [String::from("1"), String::from("2"), String::from("3")];
    let mut vec : Vec<String> = Vec::new();
    vec.push(String::from("Something"));
    vec.push(String::from("new"));
    format(arr);
    format_unknown_len(vec);
}
