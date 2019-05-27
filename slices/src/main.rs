use std::io;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item as char == ' ' {
            return &s[0..i];
        }
    }
    return &s;
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("that's not a string");
    println!("First word found ends at {}", first_word(&s[1..]));

    first_word(&String::from("something else"));
}
