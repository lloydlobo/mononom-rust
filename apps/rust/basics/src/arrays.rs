// region:      --- arrays_system

pub(crate) fn arrays_system() {
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
