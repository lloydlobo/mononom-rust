fn main() {
    println!("Hello, world!");

    print_ln_system();
    mut_system();
    shadow_variables_system();
    constants_system();
}

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

// region:      --- mut

fn print_ln_system() {
    println!("\n\nregion: print_ln_system()\n");
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

// endregion:   --- println!

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
