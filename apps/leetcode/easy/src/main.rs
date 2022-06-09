use crate::{
    graph_star::{create_input_vec, find_center},
    random_vector::random_duplicate_vector,
};

mod graph_star;
mod random_vector;

fn main() {
    let graph = create_input_vec();
    let result = find_center(graph);
    println!("find_center of graph: {:?}", result);
    let mut random_vector = random_duplicate_vector(5);
    println!("random_vector: {:?}", random_vector);
    // pop the first element
    random_vector.pop();
    println!("random_vector.pop(): {:?}", random_vector);
    let random_vector_star_center = find_center(random_vector); // doesn't work as it's not a star graph // update: n < n - 1 --> pop an element
    println!(
        "find_center of random_vector: {:?}",
        random_vector_star_center
    );
}

// region:      --- graph_star ---

/* https://leetcode.com/submissions/detail/718045529/
Success
Details
Runtime: 22 ms, faster than 66.67% of Rust online submissions for Find Center of Star Graph.
Memory Usage: 9 MB, less than 20.83% of Rust online submissions for Find Center of Star Graph.
*/

// endregion:    --- graph_star ---
