fn main() {
    println!("Hello, world!");

    let counter = 0;
    let result = loop {
        let counter = counter + 1;
        break counter;
    };
    println!("Result is {}", result);
}
