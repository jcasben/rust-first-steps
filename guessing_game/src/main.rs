use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
 * Custom type for validating the user input.
 */
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please, input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => println!("To big!"),
        }
    }
}
