use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let sec_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("numbers only dummy");
                continue;
            }
        };

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("higher"),
            Ordering::Greater => println!("lower"),
            Ordering::Equal => {
                println!("righto");
                break;
            }
        }
    }
}
