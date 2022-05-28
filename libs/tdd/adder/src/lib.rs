/// adder module

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// add_two - add two numbers
/// - a: first number
/// - a :i32
/// - returns: i32
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// greeting - return a greeting
/// - name: name of the person
/// - name: &str
/// - returns: a greeting
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!") // add a bug
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        // # Add a Bug
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Checking Results with the assert! Macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    /// # Testing Equality with the assert_eq! and assert_ne! Macros
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    /// # Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was '{}'",
            result
        );
    }

    /// # Checking for Panics with should_panic
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    /// # Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

/*
You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).

`assert!` is sot that test evaluates to true given some condition


Because the tests module is an inner module,
we need to bring the code under test in the outer module into the scope of the inner module.
We use a glob here so anything we define in the outer module is available to this tests module.
*/

/*
pub fn adder() -> String {
    "adder".into()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 4,
        };

        let smaller = Rectangle {
            width: 2,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn it_works() {
        assert_eq!(adder(), "adder".to_string());
    }
    #[test]
    fn result_works() {
        let result: i32 = 2 + 2;
        assert_eq!(result, 4 as i32);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }
}


*/
