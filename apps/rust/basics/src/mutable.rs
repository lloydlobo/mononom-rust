// region:      --- mut

pub fn mut_system() {
    print!("\n\nregion: mut_system()\n\n");
    let mut x = 5; // this will break the second line after this line --> add mut to the variable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// endregion:   --- mut
