// region:      --- types_system

pub(crate) fn types_system() {
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
