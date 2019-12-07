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

    loop {
        println!("Your guess:");
        let guess = match get_guess() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match secret.cmp(&guess) {
            Less => println!("Guess less."),
            Greater => println!("Guess greater."),
            Equal => {
                println!("Correct!");
                break;
            }
        }
    }
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
