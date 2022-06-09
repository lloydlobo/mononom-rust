use crate::{
    graph_star::{create_input_vec, find_center},
    random_vector::random_duplicate_vector,
};

mod graph_star;
mod random_vector;

fn main() {
    let graph = create_input_vec();
    let result = find_center(graph);
    println!("{:?}", result);

    let random_vector = random_duplicate_vector(5); // [[1, 1], [1], [2, 3], [3], [4, 4], [4], [4, 5], [5], [2, 1], [1]]
    println!("{:?}", random_vector)
}

// region:      --- graph_star ---

/* https://leetcode.com/submissions/detail/718045529/
Success
Details
Runtime: 22 ms, faster than 66.67% of Rust online submissions for Find Center of Star Graph.
Memory Usage: 9 MB, less than 20.83% of Rust online submissions for Find Center of Star Graph.
*/

// endregion:    --- graph_star ---
