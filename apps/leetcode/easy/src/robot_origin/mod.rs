///////////////////////////////////////////////////////////////////////////////
///
/// robot_origin
///
///
/// # Performance
/// Runtime: 4 ms, faster than 62.96% of Rust online submissions for Robot Return to Origin.
/// Memory Usage: 2.2 MB, less than 11.11% of Rust online submissions for Robot Return to Origin.
///
///////////////////////////////////////////////////////////////////////////////

///  Returns true if the robot returns to the origin.
///
/// It is also known as the "Robot Return to Origin" problem.
/// The robot is located at the origin of a 2D grid at (0, 0).
/// The robot can only move either up, down, left, or right.
///
/// Each move has similar requirements:
/// - The robot can move to an adjacent cell, or stay in the same cell.
/// - The robot cannot move outside the grid.
/// - The robot cannot move to the same cell twice.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use robot_origin::*;
/// let result = robot_returns_to_origin("UD");
/// assert!(result);
/// ```
///
/// ```
/// use robot_origin::*;
/// let result = robot_returns_to_origin("LL");
/// assert!(!result);
/// ```
///
/// # Panics
///
/// ```
/// use robot_origin::*;
/// let result = robot_returns_to_origin("");
/// assert!(!result);
/// ```
pub(crate) fn robot_returns_to_origin(moves: String) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for char in moves.chars() {
        match char {
            'U' => y += 1,
            'R' => x += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            _ => unreachable!(), // add macro `!` // This is useful any time that the compiler can't determine that some code is unreachable.
        } // end match
    } // end for
    (x == 0 && y == 0) as bool
}

///////////////////////////////////////////////////////////////////////////////
///
/// Robot return to origin
///
/// # Performance
/// Runtime: 5 ms, faster than 40.74% of Rust online submissions for Robot Return to Origin.
/// Memory Usage: 2.3 MB, less than 11.11% of Rust online submissions for Robot Return to Origin.Runtime: 6 ms, faster than 18.52% of Rust online submissions for Robot Return to Origin.
///
///
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn judge_circle(moves: String) -> bool {
    println!("moves: {}", moves);
    let mut xy: [i32; 2] = [0, 0];
    for char in moves.chars() {
        // Double-quotes "" create string literals, while single-quotes '' create character literals. See Literal Expressions in the Rust reference.
        if char == 'U' {
            xy[1] += 1;
        } else if char == 'R' {
            xy[0] += 1;
        } else if char == 'D' {
            xy[1] -= 1;
        } else if char == 'L' {
            xy[0] -= 1;
        }
    }

    xy[0] == 0 && xy[1] == 0
}

/*
Yes, indexing into a string is not available in Rust. The reason for this is that Rust strings are encoded in UTF-8 internally, so the concept of indexing itself would be ambiguous, and people would misuse it: byte indexing is fast, but almost always incorrect (when your text contains non-ASCII symbols, byte indexing may leave you inside a character, which is really bad if you need text processing), while char indexing is not free because UTF-8 is a variable-length encoding, so you have to traverse the entire string to find the required code point.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_returns_to_origin() {
        let string: String = "UD".to_string();
        let result = robot_returns_to_origin(string);
        assert!(result);
    }

    #[test]
    #[should_panic]
    fn test_robot_returns_to_origin_panics() {
        let string: String = "".to_string();
        let result = robot_returns_to_origin(string);
        assert!(!result);
    }

    #[test]
    fn test_judge_circle() {
        let string: String = "UD".to_string();
        let result = judge_circle(string);
        assert!(result);
    }
}
