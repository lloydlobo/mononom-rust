// region:      --- for_loops_system

pub fn for_loops_system() {
    println!("\n\nregion:      --- for_loops_system");

    // for loop isn't like in c, python, or java, it's like in rust. lol
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value of a is: {}", element); // prints the value of a in the array -> 10, 20, 30, 40, 50
    }

    // for loop with a range - number >= 1 and number < 4 // i.e. 1, 2, 3
    // - rev() is a method that returns a reversed iterator over a slice
    for number in (1..4).rev() {
        println!("The number is: {}", number); // prints the number in the range -> 3, 2, 1
    } // This is only possible if the iterator has an end, so rev() only works on DoubleEndedIterators.
}

// endregion:   --- for_loops_system
