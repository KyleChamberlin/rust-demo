extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to: Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}.", secret_number);

    println!("We are generating a number between 1 and 100.");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let mut guess = guess.trim();

        if guess == "quit" || guess == "exit" {
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                println!("To stop playing please type 'quit'.");
                continue
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less     => println!("Too small... maybe next time."),
            Ordering::Greater  => println!("Too big! try again."),
            Ordering::Equal    => {
                println!("YAY!!!! you won!");
                break;
            }
        }
    }
}
