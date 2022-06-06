// region:      --- tuples_system

pub(crate) fn tuples_system() {
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
