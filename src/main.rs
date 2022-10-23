use std::cmp::Ordering;
use std::io;

use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(0..100);
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
