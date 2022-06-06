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
}

// region:      --- functions_system
// #[allow(unused_variables)]
fn functions_system() {
    println!("\n\nregion:      --- functions_system");

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(five());
    println!("The value of x -> plus_one(five()) is: {}", x); // 6

    let x = 3;
    let y = is_odd(x);
    println!("x is {}, y -> is_odd(x) is: {}", x, y); // x is 3, y -> is_odd(x) is: true

    let tup: (u8, u16, u32) = (255, 65535, 4294967295); // tuple these are the max values for each type
    let int_to_float = tuple_demo(tup);
    // {:?} is a pretty printer for debug or implement std::fmt::Display
    println!(
        "Integer in tuple converted to Floats is: {:?}",
        int_to_float
    ); // (65790.0, 4294967295.0)

    let factorial_number: u64 = 5;
    let get_factorial = factorial(factorial_number);
    println!(
        "The factorial of {} is: {}",
        factorial_number, get_factorial
    ); // The factorial of 5 is: 120
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 // no ; because it's an expression
}

fn is_odd(x: i32) -> bool {
    // x % 2 == 1
    if (x & 1) == 0 {
        false
    } else {
        true
    }
}

fn tuple_demo(t: (u8, u16, u32)) -> (f32, f64) {
    let x: f32 = t.0 as f32 + t.1 as f32;
    let y: f64 = t.2 as f64;
    (x, y)
}

fn factorial(number: u64) -> u64 {
    match number {
        0 => 1,
        1 => 1,
        _ => factorial(number - 1) * number, // recursive call, _ is anything other than 0, 1
    }
}

// endregion:  --- functions_system

// region:      --- expressions_and_statements

#[allow(unused_variables)]
#[allow(unused_parens)]
fn expressions_and_statements_system() {
    println!("\n\nregion:      --- expressions_and_statements");

    let a = 3 + 7;
    let b = (3 + 7); // ; marks end of statement, while (3 + 7) is expression
    let c = { 3 + 7 };

    let y = {
        let mut x = 3;
        x = x * 2;
        // there is no ";" a it is an expression
        x + 1
    }; // end of statement // you can do calculations in braces
    println!("The value of y is: {}", y); // prints: The value of y is: 7
}

// endregion:   --- expressions_and_statements

// region:      --- arrays_system

fn arrays_system() {
    println!("\n\nregion:      --- arrays_system");

    // arrays (all must be of same type, fixed length)
    let a = [1, 2, 3, 4, 5];
    let b: [u16; 5] = [1, 2, 3, 4, 5];
    let c = [0; 5]; // want array 5 long and all to be zero 0
    println!("a b c: {:?} {:?} {:?}", a, b, c); // a b c: [1, 2, 3, 4, 5] [1, 2, 3, 4, 5] [0, 0, 0, 0, 0]

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first = a[0]; // 1
    let nov = months[10];
    println!("Two elements of the arrays: {} {}", first, nov); // 1 November
}

// endregion:   --- arrays_system

// region:      --- tuples_system

fn tuples_system() {
    println!("\n\nregion:      --- tuples_system");

    // tuples (can have different types, fixed length)
    let tup: (i32, f64, u8, f32) = (500, 6.4, 1, 29.29);
    let tup2 = (1500, 3.4); // did not give type as compiler infers it
    println!("tup and tup2: {:?} {:?}", tup, tup2); // tup and tup2: (500, 6.4, 1, 29.29) (1500, 3.4)

    let (w, x, y, z) = tup;
    println!("w,x,y,z are: {} {} {} {}", w, x, y, z); // w,x,y,z are: 500 6.4 1 29.29 --> no need for brackets like debug, just individual variables

    let five_hundred = tup.0;
    let three_point_four = tup2.1;
    let one = tup.2;
    let twonine_29 = tup.3;
    println!(
        "From the tuples: {} {} {} {}",
        five_hundred, three_point_four, one, twonine_29
    ); // From the tuples: 500 3.4 1 29.29
}

// endregion:   --- tuples_system

// region:      --- strings_system

fn strings_system() {
    println!("\n\nregion:      --- strings_system");
    // All strings in rust are UTF-8 encoded., so -->
    // str - static string literal
    let howdy = "Howdy🖐"; // 6 long? --> no, 4 bytes for emoji --> 5 + 4 = 9 bytes
    println!("Some str stuff: {} {}", howdy.len(), howdy.is_empty(),); // 9 false --> //Returns true if self has a length of zero bytes.
    println!("The bytes of howdy: {:?}", howdy.as_bytes()); // The bytes of howdy: [72, 111, 119, 100, 121, 240, 159, 150, 144]

    // strings
    let mut hello = String::from("Hello"); // string from a str

    hello.push('w'); // push a char literal --> use single quotes
    hello.push_str("orld!"); // push a str literal --> use double quotes
    println!("String: {}", hello); // String: Helloworld!

    hello.insert(5, ','); // insert a char at a given index
    println!("String: {}", hello); // String: Hello,world!
}
// endregion:   --- strings_system

// region:      --- types_system

fn types_system() {
    println!("\n\nregion:      --- types_system");
    // signed integers: i8, i16, i32, i64, isize
    // unsigned integers: u8, u16, u32, u64, usize
    let x128: u128 = 0xFAFBFCFD_FEF1F2F3_F4F5F6F7_F8F9FAFB;
    let x64: i64 = 123456;
    // 32 bit & 64 bit floating point numbers: f32, f64
    let x = 2.0; // f64 is the default
    let y: f32 = 3.0;
    println!(
        "The value of x128, x64, x, y is: {} {} {} {}",
        x128, x64, x, y
    ); // The value of x128, x64, x, y is: 333615396748568137220584888834868247291 123456 2 3

    // characters and booleans: char, bool
    let c = "z";
    let z = "Z";
    let hand = "🖐";
    let job_done = false;
    println!("Some chars: {} {} {} {}", c, z, hand, job_done); // Some chars: z Z 🖐 false
}

// endregion:   --- types_system

// region:      --- constants_system

fn constants_system() {
    println!("\n\nregion: constants_system");

    // const are not the same as immutable --> they are immutable by default
    // when using 'const' keyword, type must be annotated explicitly
    // constants may only be set to expressions
    // Always use UPPERCASE for constants
    const FHD_WIDTH: u32 = 1920;
    const APPROX_PI: f32 = 22.0 / 7.0;
    println!("FHD_WIDTH: {}, APPROX_PI: {}", FHD_WIDTH, APPROX_PI); // FHD_WIDTH: 1920, APPROX_PI: 3.142857
}

// endregion:   --- constants_system

// region:      --- shadow_variables_system

fn shadow_variables_system() {
    print!("\n\nregion: shadow_variables_system\n\n");
    // shadow
    let y: i32 = 5;
    let y = y + 1;
    let y = y + 2;
    println!("The value of y is: {}", y); // 8

    // shadow and change type --> advantage of changing types
    let abc: &str = "ABC";
    let abc: usize = abc.len(); // len is in bytes? --> The pointer-sized unsigned integer type.
                                // The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    println!("The value of abc is: {}", abc) // 3
}

// endregion:   --- shadow_variables_system

// region:      --- mut

fn mut_system() {
    print!("\n\nregion: mut_system()\n\n");
    let mut x = 5; // this will break the second line after this line --> add mut to the variable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// endregion:   --- mut

// region:      --- println_system

fn println_system() {
    println!("\n\nregion: println_system()\n");
    let x = 7;
    let y = 1;

    println!("x is {}, y is {}", x, y);
    println!(
        "x is {value_x}, y is {something}",
        value_x = x,
        something = y
    ); // helpful for formatting when printing many variables
    println!("debug {:?}", (3, 4));
    println!("debug_this {:?}", [x, y]); // debug_this [7, 1]

    println!("y is {1}, x is {0}", x, y); // y is 1, x is 7 (order is reversed)
    println!("y is {1}, x is {0}", y, x);
}

// endregion:   --- println_system

/* REFERENCE
 ___ ___ ___ ___ ___ ___ _  _  ___ ___
| _ \ __| __| __| _ \ __| \| |/ __| __|
|   / _|| _|| _||   / _|| .` | (__| _|
|_|_\___|_| |___|_|_\___|_|\_|\___|___|

/*
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

*/
