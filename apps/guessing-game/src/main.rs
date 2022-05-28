use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::ControlFlow;

/// welcome_user prints a welcome message to the user.
/// # Prints
/// - println! is a macro that prints to the console.
/// - {} is a placeholder for the string that will be printed.
/// - "Welcome to the guessing game!" is the string that will be printed.
/// - colored is a library that provides color to the output.
/// # Color
/// - .bold() is a method that returns a string in bold.
/// - .yellow() is a method that returns a string in yellow.
/// - .blue() is a method that returns a string in blue.
pub(crate) fn welcome_user() {
    println!("{}", "Welcome to the guessing game!".blue());
    println!("{}", "You gotta guess a number...\n".yellow());
}

/// loop_guess_inputs is a function that loops until the user guesses the correct number.
/// - random_number is a variable that stores a random number between 1 and 100.
/// - mut input is a mutable variable that stores the user input.
/// - input: u32
/// - random_number: u32
/// - input is a variable that stores the user input.
/// - ControlFlow: enum (break)
/// - rand: library (random number generator)
/// - std::cmp: library
/// - std::io: library
/// - read_line: function from std::io
/// - trim: function that removes whitespace from the start and end of a string.
///
/// # Panics
/// - panic! is a macro that stops the program and prints a message to the console.
/// Panics if the user enters a non-number.
pub(crate) fn loop_guess_inputs(random_number: u32) {
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

        if let ControlFlow::Break(_) = match_input(input, random_number) {
            break;
        }
        println!("You guessed: {}", input);
    }
}

///  This function is checking if the user input is a valid number.
/// match_input is a function that takes two arguments:
/// - input: u32
/// - random_number: u32
pub(crate) fn match_input(input: u32, random_number: u32) -> ControlFlow<()> {
    match input.cmp(&random_number) {
        Ordering::Less => println!("{}", "Too low".red()),
        Ordering::Greater => println!("{}", "Too high".red()),
        Ordering::Equal => {
            println!("{}", "You win".green());
            return ControlFlow::Break(());
        }
    };
    ControlFlow::Continue(())
}

/// This is a guessing game. The user has to guess a number between 1 and 100.
/// The program will tell the user if the guess is too high or too low.
/// main function is the entry point of a Rust program.
/// -ControlFlow is a trait that has a single method, break.
/// - colored is a library that provides color to the output.
///
/// - random_number is a variable that stores a random number between 1 and 100.
/// - input is a variable that stores the user input.
///
/// - random_number: u32
/// - input: u32
/// - ControlFlow: enum
/// - colored: library
/// - rand: library
/// - std::cmp: library
/// - std::io: library
/// - std::ops: library
/// - std::thread: library
fn main() {
    welcome_user();
    let random_number = rand::thread_rng().gen_range(1..101);
    loop_guess_inputs(random_number);
}
