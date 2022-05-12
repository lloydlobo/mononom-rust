use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// CREDITS: https://youtu.be/H0xBSbnQYds => Let's Get Rusty

// It's generating a random number between 1 and 100 and then it's comparing the user input to the random number.
fn main() {
    println!("Guess the number");

    // Create secret number using the rand package and the thread_rng function

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    //  Add a loop
    let mut count: i32 = 0;
    loop {
        println!("Please input your guess.");
        // Creating a mutable variable called guess and assigning it the value of a new empty string.
        let mut guess: String = String::new();

        // It's reading the input from the user and storing it in the variable guess.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // It's a match expression that is checking if the user input is a valid number.
        // Trims white spaces, and Parses the input guess string to a number
        // SHADOWING from (String to u32) || converting types while preserving same name
        let guess: u32 = {
            let this = guess.trim().parse();
            match this {
                Ok(num) => num,
                // _ is a catch all value
                Err(_) => continue,
            }
        };
        println!("You guessed: {}", guess);

        /* now create a match expression as err-> mismatched type*/
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "You guessed low".red()),
            Ordering::Greater => println!("{}", "You guessed high".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("You guessed {} times", count.to_string().yellow());
                println!("{}", "Thanks for playing!".blue());

                // It's breaking the loop.
                break;
            }
        }
        // It's adding 1 to the count variable.
        count += 1;
    }
}

// next - add a random number in Cargo.toml under dependencies as "rand"
// after that run `cargo build` to install the dependency -> it shows up in Cargo.lock
// after generating we need to compare the random secret_number to the user guess => bring in Ordering (it's an enum)
//  Create a match expression that is checking if the user input is a valid number => Trims white spaces, and Parses the input guess string to a number
// Loop:: Now we want the user to keep guessing if they get it wrong. and add everything in the loop.
// It never ends -> modify the Equal ordering to break the loop.
// add tries with variable count
// Address invalid inputs like strings (characters, letters, symbols, etc)
/* thread 'main' panicked at 'Please type a number: invalid digit found in string', apps/guessing-game/src/main.rs:34:31
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace */
// Err(error) => panic!("Please type a number: {}", error), replaced to =>  Err(_) => continue, where _ is a catch all error value
// NOW ADD COLOR to the output with dependencies
// [dependencies]
// rand = "0.8.4"
// colored = "2.0.0"
