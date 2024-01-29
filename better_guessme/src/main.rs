use std::{io};
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Option<Guess> {
       if value < 1 || value > 100 {
           println!("Guess value must be between 1 and 100, got {}", value);
           return None;
       }
       Some(Guess{value})
    }

    pub fn value(&self) -> i32 {
        self.value
    }
    
}


fn main() {
    println!("=== Better Guess me Start! ===");
    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut input_number = String::new();

    loop {   
        println!("Enter your number: ");
        input_number.clear();
        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read the input.");

        let input_number = match input_number.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Some(guess) => guess,
                None => continue,
            },
            Err(_) => {
                println!("The Game need number, got '{}' ", input_number);
                continue;
            },
        };

        match input_number.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater=> println!("Too big!"),
        }
    }
}
