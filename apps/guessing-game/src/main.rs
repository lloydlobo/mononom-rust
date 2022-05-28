use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Welcome to the guessing game!".blue());
    println!("{}", "You gotta guess a number...\n".yellow());

    let random_number = rand::thread_rng().gen_range(1..101);
    // println!("{} is the random number", random_number);
    loop {
        println!("{}", "\nType a number between 1 and 100\n".cyan().bold());
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input number!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input.cmp(&random_number) {
            Ordering::Less => println!("{}", "Too low".red()),
            Ordering::Greater => println!("{}", "Too high".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        };
        println!("You guessed: {}", input);
    }
}
