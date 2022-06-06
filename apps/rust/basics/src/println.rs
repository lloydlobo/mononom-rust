// region:      --- println_system

pub(crate) fn println_system() {
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
