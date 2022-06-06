// region:      --- functions_system

pub(crate) fn functions_system() {
    println!("\n\nregion:      --- functions_system");

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(five());
    println!("The value of x -> plus_one(five()) is: {}", x); // 6

    let x = 3;
    let y = is_odd(x);
    println!("x is {}, y -> is_odd(x) is: {}", x, y); // x is 3, y -> is_odd(x) is: true

    let tup: (u8, u16, u32) = (255, 65535, 4294967295); // tuple these are the max values for each type
    let int_to_float = tuple_demo(tup);
    // {:?} is a pretty printer for debug or implement std::fmt::Display
    println!(
        "Integer in tuple converted to Floats is: {:?}",
        int_to_float
    ); // (65790.0, 4294967295.0)

    let factorial_number: u64 = 5;
    let get_factorial: u64 = factorial(factorial_number);
    println!(
        "The factorial of {} is: {}",
        factorial_number, get_factorial
    ); // The factorial of 5 is: 120
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 // no ; because it's an expression
}

fn is_odd(x: i32) -> bool {
    // x % 2 == 1
    if (x & 1) == 0 {
        false
    } else {
        true
    }
}

fn tuple_demo(t: (u8, u16, u32)) -> (f32, f64) {
    let x: f32 = t.0 as f32 + t.1 as f32;
    let y: f64 = t.2 as f64;
    (x, y)
}

fn factorial(number: u64) -> u64 {
    match number {
        0 => 1,
        1 => 1,
        _ => factorial(number - 1) * number, // recursive call, _ is anything other than 0, 1
    }
}

// endregion:  --- functions_system
