// region:      --- loop_while_system

pub(crate) fn loop_while_system() {
    println!("\n\nregion:      --- loop_while_system");

    // loops
    let mut counter = 0;
    let result_loop_times2 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(
        "The result of loop of counter = {} times 2 is: {}",
        counter, result_loop_times2
    ); // counter = 10 times 2 is: 20

    // while loops
    counter = 3;
    while counter != 0 {
        println!("counter is: {}", counter);
        counter -= 1;
    }
}

// endregion:   --- loop_while_system
