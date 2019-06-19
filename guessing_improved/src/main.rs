use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value : i32) -> Guess{
        if value < 1 || value > 101 {
            panic!("Guess not within bounds");
        }
        
        Guess {
            value
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    let sec_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("numbers only dummy");
                continue;
            }
        };

        match guess.value().cmp(&sec_num) {
            Ordering::Less => println!("higher"),
            Ordering::Greater => println!("lower"),
            Ordering::Equal => {
                println!("righto");
                break;
            }
        }
    }
}
