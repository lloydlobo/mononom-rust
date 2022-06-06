// region:      --- constants_system

pub(crate) fn constants_system() {
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
