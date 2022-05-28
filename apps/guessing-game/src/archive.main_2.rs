use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret random number: {}", secret_number);

    loop {
        println!("Type your number here");

        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read the line you typed");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // shadowing from string to number

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low".red()),
            Ordering::Greater => println!("{}", "To high".red()),
            Ordering::Equal => {
                println!("{}", "You guessed right! You win!".green());
                break;
            }
        }

        println!("You guessed: {}", guess_number);
    }
}

// let guess_number: u32 = guess_number.trim().parse().expect("Please type a number");
