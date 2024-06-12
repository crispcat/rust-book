use std::cmp::Ordering;
use std::io;
use rand::{
    Rng,
    thread_rng
};

const ATTEMPTS: u32 = 5;

fn main() {

    let secret = thread_rng().gen_range(1..=100);
    println!("Try to guess a number I guessed (1-100) ...");

    for attempts in (0..ATTEMPTS).rev() {

        let guess : u32 = loop {

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read stdin");

            match guess.trim().parse() {
                Ok(num) => break num,
                Err(_) => { println!("Guess a number!"); }
            };
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too little! You have {attempts} attempts."),
            Ordering::Equal => { println!("You won, perish..."); break; },
            Ordering::Greater => println!("Too big! You have {attempts} attempts.")
        }
    }

    println!("It was {secret}.");
}