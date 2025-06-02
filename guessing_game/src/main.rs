use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guessing game.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to input from the use.");
        
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}