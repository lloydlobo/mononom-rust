use rand::Rng;

// create a custom function which takes in argument `Vec<Vec<i32>>` -> `i32` where one of the edges is a duplicate. The function should return the duplicate edge.

/// # Random Duplicate Vector
/// * The function takes in a vector of vectors of integers.
/// * The function returns a vector of vectors of integers.
/// # Examples
/// * length = 5
/// * output = [[1, 1], [1], [2, 3], [3], [4, 4], [4], [4, 5], [5], [2, 1], [1]]
pub(crate) fn random_duplicate_vector(length: i32) -> Vec<Vec<i32>> {
    let mut vec = Vec::new();
    for _ in 0..length {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();

        if generate_random_number(0, 2) == 0 {
            vec1.push(length);
            vec2.push(rand::thread_rng().gen_range(1..(length + 1)));
        } else {
            vec1.push(rand::thread_rng().gen_range(1..(length + 1)));
            vec2.push(length);
        }
        vec1.push(vec2[0]);
        vec.push(vec1);
    }
    vec
}

/// # Generate Random Number
/// * The function takes in two integers of minimum and maximum range
/// * The function returns a random number between the minimum and maximum range
/// # Examples
/// * input = 0, 2
/// * output = 0 or 1
pub(crate) fn generate_random_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}

// let random_vector = random_duplicate_vector(5); // [[1, 1], [1], [2, 3], [3], [4, 4], [4], [4, 5], [5], [2, 1], [1]]

#[cfg(test)]
mod tests {
    use crate::random_vector::{generate_random_number, random_duplicate_vector};

    #[test]
    fn test_random_duplicate_vector() {
        let vec = random_duplicate_vector(5);
        assert_eq!(vec.len(), 5);

        let vec = random_duplicate_vector(10);
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn test_generate_random_number() {
        let result = generate_random_number(0, 2);
        assert!(result == 0 || result == 1);

        let result = generate_random_number(0, 3);
        assert!(result == 0 || result == 1 || result == 2);
    }
}
