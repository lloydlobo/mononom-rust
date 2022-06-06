fn main() {
    println!("Hello, world!");

    print_ln_system();
    mut_system();
    shadow_variables_system();
}

fn shadow_variables_system() {
    print!("\n\nregion: shadow_variables_system\n\n");
}

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
