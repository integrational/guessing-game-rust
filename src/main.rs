use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin;

use rand::{thread_rng, Rng};

fn main() {
    let min = 10;
    let max = 90;

    let secret = gen_secret(min, max);

    println!(
        "Guess the secret number between {} amd {} inclusive.",
        min, max
    );

    let mut num_guesses = 0;
    let mut correct_guess = false;
    while !correct_guess {
        let guess = read_guess();
        num_guesses += 1;
        correct_guess = eval_guess(secret, guess);
    }

    let num_numbers = max - min + 1;
    let success_rate = 100 * (num_numbers - num_guesses) / (num_numbers - 1);
    println!(
        "You required {} guesses to find the secret number among {} candidates, for a success rate of {}%.",
        num_guesses, num_numbers, success_rate
    )
}

fn gen_secret(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min, max + 1)
}

fn read_guess() -> i32 {
    println!("Your guess:");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Error reading line!");
    match guess.trim().parse() {
        Ok(n) => n,
        Err(_) => read_guess(),
    }
}

fn eval_guess(secret: i32, guess: i32) -> bool {
    match secret.cmp(&guess) {
        Less => {
            println!("Guess less.");
            false
        }
        Greater => {
            println!("Guess greater.");
            false
        }
        Equal => {
            println!("Correct!");
            true
        }
    }
}
