// region:      --- expressions_and_statements

#[allow(unused_variables)]
#[allow(unused_parens)]
pub(crate) fn expressions_and_statements_system() {
    println!("\n\nregion:      --- expressions_and_statements");

    let a = 3 + 7;
    let b = (3 + 7); // ; marks end of statement, while (3 + 7) is expression
    let c = { 3 + 7 };

    let y = {
        let mut x = 3;
        x = x * 2;
        // there is no ";" a it is an expression
        x + 1
    }; // end of statement // you can do calculations in braces
    println!("The value of y is: {}", y); // prints: The value of y is: 7
}

// endregion:   --- expressions_and_statements
