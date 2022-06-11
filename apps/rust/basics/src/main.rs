use crate::{
    arrays::arrays_system, constants::constants_system,
    expressions_statements::expressions_and_statements_system, for_loops::for_loops_system,
    functions::functions_system, loop_while::loop_while_system, mutable::mut_system,
    println::println_system, shadow_variables::shadow_variables_system, strings::strings_system,
    tuples::tuples_system, types::types_system,
};

pub mod arrays;
pub mod constants;
pub mod expressions_statements;
pub mod for_loops;
pub mod functions;
mod hash_maps;
pub mod loop_while;
pub mod mutable;
pub mod println;
pub mod shadow_variables;
pub mod strings;
pub mod tuples;
pub mod types;

/// The main function
/// returns: nothing
/// params:  nothing
/// # Functions
/// - main
/// # Examples
/// ```
/// main();
/// ```
/// # Errors
/// - none
fn main() {
    println!("Hello, world!");

    println_system();
    mut_system();
    shadow_variables_system();
    constants_system();
    types_system();
    strings_system();
    tuples_system();
    arrays_system();
    expressions_and_statements_system();
    functions_system();
    loop_while_system();
    for_loops_system();
    // clear the screen
    println!("\x1B[2J\x1B[1;1H");
    hash_maps::phone_book_hash_map();
}

/* REFERENCE
 ___ ___ ___ ___ ___ ___ _  _  ___ ___
| _ \ __| __| __| _ \ __| \| |/ __| __|
|   / _|| _|| _||   / _|| .` | (__| _|
|_|_\___|_| |___|_|_\___|_|\_|\___|___|


12 Things to Help You Learn Rust
- by Gary Explains

https://www.youtube.com/watch?v=a8abW3RlOn8&t=785s
BIGDummyHead
11 months ago
TOC:  4:06

println! : 4:25
mut : 10:19
shadow variables : 11:40
constants : 13:49
types : 15:03
Strings : 17:05
tuples : 20:08
arrays : 22:35
expressions and statements : 24:01
functions : 26:38
loop and while : 31:32
for : 33:13

*/


