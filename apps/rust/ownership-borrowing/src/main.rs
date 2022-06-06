fn main() {
    println!("Hello, world!");
    errors_system();
}

// region:      --- errors_system

fn errors_system() {
    fn borrowing_from_strings() {
        let string_1 = String::from("hello");
        // err: let string_2 = string_1; // this is borrowing, string_1 is moved to string_2

        // clone() --> Returns a copy of the value.
        let string_2 = string_1.clone(); // this is cloning, string_1 is copied to string_2
        println!("string_1 = {}, string_2 = {}", string_1, string_2); // string_1 = hello, string_2 = hello (after using .clone())
    }
    borrowing_from_strings();
    /*
        error[E0382]: borrow of moved value: `string_1`
    10 |     let string_1 = String::from("hello");
       |             -------- move occurs because `string_1` has type `String`, which does not implement the `Copy` trait
    11 |     let string_2 = string_1; // this is borrowing, string_1 is moved to string_2
       |                        -------- value moved here
    12 |         println!("string_1 = {}, string_2 = {}", string_1, string_2);
       |                                                  ^^^^^^^^ value borrowed
    here after move
       |
       = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
        */
}

// endregion:   --- errors_system

/* REFERENCE

   ___  _______________  _____  ___________
  / _ \/ __/ __/ __/ _ \/ __/ |/ / ___/ __/
 / , _/ _// _// _// , _/ _//    / /__/ _/
/_/|_/___/_/ /___/_/|_/___/_/|_/\___/___/

Rust: What is Ownership and Borrowing?

author: Gary Explains
source: https://www.youtube.com/watch?v=79phqVpE7cU


*/
