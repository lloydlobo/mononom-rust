// region:      --- shadow_variables_system

pub(crate) fn shadow_variables_system() {
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
