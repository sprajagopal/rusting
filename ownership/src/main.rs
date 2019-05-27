fn taking_over(s: String){
    println!("Just printing {}", s);
}

fn main() {
    println!("Hello, world!");
    let s = String::from("something");
    taking_over(s);
    println!("Can I still use s here? Trying to print...");
    println!("{}", s);
}
