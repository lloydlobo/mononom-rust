use rand::Rng;
use std::cmp::Ordering;
use std::io;
// CREDITS: https://youtu.be/H0xBSbnQYds => Let's Get Rusty

// We create a mutable variable called guess and assign it the value of a new empty string. Then we
// read the input from the user and store it in the variable guess. Finally, we print the string "You
// guessed: " and then the value of the variable guess
fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // Creating a mutable variable called guess and assigning it the value of a new empty string.
    let mut guess = String::new();

    // Create secret number using the rand package and the thread_rng function
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let float_rng: f64 = rng.gen();
    println!("A float is: {}", float_rng);

    let secret_number = rng.gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = {
        let this = guess.trim().parse();
        match this {
            Ok(guess) => guess,
            Err(error) => panic!("Please type a number: {}", error),
        }
    };

    println!("You guessed: {}", guess);

    // compare guess
    /* now create a match expression as err-> mismatched type*/
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!: {}", secret_number),
        Ordering::Equal => println!("You win! : {}", secret_number),
        Ordering::Greater => println!("Too big!: {}", secret_number),
    }
}

// next - add a random number in Cargo.toml under dependencies as "rand"
// after that run `cargo build` to install the dependency -> it shows up in Cargo.lock
// after generating we need to compare the random secret_number to the user guess => bring in Ordering (it's an enum)
// pub enum Ordering {
//     /// An ordering where a compared value is less than another.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     Less = -1,
//     /// An ordering where a compared value is equal to another.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     Equal = 0,
//     /// An ordering where a compared value is greater than another.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     Greater = 1,
// }
