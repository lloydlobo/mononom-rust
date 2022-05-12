use rand::Rng;
use std::cmp::Ordering;
use std::io;
// CREDITS: https://youtu.be/H0xBSbnQYds => Let's Get Rusty

// It's generating a random number between 1 and 100 and then it's comparing the user input to the random number.
fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    // Create secret number using the rand package and the thread_rng function

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // Creating a mutable variable called guess and assigning it the value of a new empty string.
    let mut guess: String = String::new();

    // It's reading the input from the user and storing it in the variable guess.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // It's a match expression that is checking if the user input is a valid number.
    // Trims white spaces, and Parses the input guess string to a number
    let guess: u32 = {
        let this = guess.trim().parse();
        match this {
            Ok(guess) => guess,
            Err(error) => panic!("Please type a number: {}", error),
        }
    };

    /* now create a match expression as err-> mismatched type*/
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed lower than: {}", secret_number),
        Ordering::Equal => println!("You win! : {}", secret_number),
        Ordering::Greater => println!("You guessed higher than: {}", secret_number),
    }

    println!("You guessed: {}", guess);
}

// next - add a random number in Cargo.toml under dependencies as "rand"
// after that run `cargo build` to install the dependency -> it shows up in Cargo.lock
// after generating we need to compare the random secret_number to the user guess => bring in Ordering (it's an enum)
//  Create a match expression that is checking if the user input is a valid number => Trims white spaces, and Parses the input guess string to a number
// Now we want the user to keep guessing if they get it wrong.
