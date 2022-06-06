// region:      --- strings_system

pub(crate) fn strings_system() {
    println!("\n\nregion:      --- strings_system");
    // All strings in rust are UTF-8 encoded., so -->
    // str - static string literal
    let howdy = "Howdy🖐"; // 6 long? --> no, 4 bytes for emoji --> 5 + 4 = 9 bytes
    println!("Some str stuff: {} {}", howdy.len(), howdy.is_empty(),); // 9 false --> //Returns true if self has a length of zero bytes.
    println!("The bytes of howdy: {:?}", howdy.as_bytes()); // The bytes of howdy: [72, 111, 119, 100, 121, 240, 159, 150, 144]

    // strings
    let mut hello = String::from("Hello"); // string from a str

    hello.push('w'); // push a char literal --> use single quotes
    hello.push_str("orld!"); // push a str literal --> use double quotes
    println!("String: {}", hello); // String: Helloworld!

    hello.insert(5, ','); // insert a char at a given index
    println!("String: {}", hello); // String: Hello,world!
}
// endregion:   --- strings_system
