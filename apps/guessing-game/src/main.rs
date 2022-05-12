use rand::Rng;
use std::io;

// We create a mutable variable called guess and assign it the value of a new empty string. Then we
// read the input from the user and store it in the variable guess. Finally, we print the string "You
// guessed: " and then the value of the variable guess
fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // Creating a mutable variable called guess and assigning it the value of a new empty string.
    let mut guess = String::new();
    // Create secret number using the rand package and the thread_rng function
    let secret_number = rand::thread_rng().gen_range(0..101);
    println!("The secret number is: {}", secret_number);

    // Reading the input from the user and storing it in the variable guess.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Printing the string "You guessed: " and then the value of the variable guess.
    println!("You guessed: {}", guess);
}

// next - add a random number in Cargo.toml under dependencies as "rand"
// after that run `cargo build` to install the dependency -> it shows up in Cargo.lock
