use std::io;

// We create a mutable variable called guess and assign it the value of a new empty string. Then we
// read the input from the user and store it in the variable guess. Finally, we print the string "You
// guessed: " and then the value of the variable guess
fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // Creating a mutable variable called guess and assigning it the value of a new empty string.
    let mut guess = String::new();

    // Reading the input from the user and storing it in the variable guess.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Printing the string "You guessed: " and then the value of the variable guess.
    println!("You guessed: {}", guess);
}
