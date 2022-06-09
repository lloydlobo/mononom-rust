use crate::{
    graph_star::{create_input_vec, find_center},
    random_vector::random_duplicate_vector,
};

mod graph_star;
mod random_vector;

// region:      --- main ---

fn main() {
    let graph = create_input_vec();
    let result = find_center(graph);

    let mut random_vector = random_duplicate_vector(100);
    random_vector.pop(); // pop the first element
    let random_vector_star_center = find_center(random_vector); // doesn't work as it's not a star graph // update: n < n - 1 --> pop an element

    println!("find_center of graph: {:?}", result);
    println!(
        "find_center of random_vector.pop(): {:?}",
        random_vector_star_center
    );
}

// endregion:    --- main ---

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_find_center() {
        let result = find_center_graph();
        assert!(result == 1);

        let (random_vector_clone_pop, random_vector_star_center) =
            find_center_random_vector_graph();

        assert!(random_vector_star_center == 100);
        assert!(random_vector_star_center != 1);
        assert!(random_vector_clone_pop.len() == 99);
    }

    fn find_center_random_vector_graph() -> (Vec<Vec<i32>>, i32) {
        let random_vector: Vec<Vec<i32>> = random_duplicate_vector(100);
        let mut random_vector_clone: Vec<Vec<i32>> = random_vector.clone();
        random_vector_clone.pop();
        let random_vector_clone_pop: Vec<Vec<i32>> = random_vector_clone.clone().to_vec();
        // if we pop this again the len() is 98 --> it's borrowing the vector
        let random_vector_star_center: i32 = find_center(random_vector_clone);
        (random_vector_clone_pop, random_vector_star_center)
    }

    fn find_center_graph() -> i32 {
        let graph = create_input_vec();
        let result = find_center(graph);
        result
    }
}

// region:      --- graph_star ---

/* https://leetcode.com/submissions/detail/718045529/
Success
Details
Runtime: 22 ms, faster than 66.67% of Rust online submissions for Find Center of Star Graph.
Memory Usage: 9 MB, less than 20.83% of Rust online submissions for Find Center of Star Graph.
*/

// endregion:    --- graph_star ---
