use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin;
use std::str::FromStr;

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
    loop {
        println!("Your guess:");
        let guess = match get_guess() {
            Ok(n) => n,
            Err(_) => continue,
        };
        num_guesses += 1;

        match secret.cmp(&guess) {
            Less => println!("Guess less."),
            Greater => println!("Guess greater."),
            Equal => {
                println!("Correct!");
                break;
            }
        }
    }

    let num_numbers = max - min + 1;
    let success_rate = 100 * (num_numbers - num_guesses) / (num_numbers - 1);
    println!(
        "You required {} guesses to find the secret number out of a pool of {} candidates, for a success rate of {}%",
        num_guesses, num_numbers, success_rate
    )
}

fn gen_secret(min: i32, max: i32) -> i32 {
    let secret: i32 = thread_rng().gen_range(min, max + 1);
    secret
}

fn get_guess() -> Result<i32, <i32 as FromStr>::Err> {
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Error reading line!");
    let guess = guess.trim().parse();
    guess
}
