/// # find_center
/// * It finds the center of a star graph.
/// * returns the center node i32
/// * returns -1 if there is no center (not yet implemented)
/// # Parameters
/// * `graph`: a vector of vectors of integers.
/// * `edges`: the number of edges in the graph.
/// edges is a vector of vectors of integers.
/// # Examples
/// Example 1:
/// Input: edges = [[1,2],[5,1],[1,3],[1,4]] // Output: 1
/// Example 2:
/// Input: edges = [[1,2],[2,3],[4,2]] // Output: 2
/// # Constraints
/// - 3 <= n <= 105
/// * edges.length == n - 1
/// * edges[i].length == 2
/// * 1 <= ui, vi <= n
/// * ui != vi
/// # Notes
/// * The graph is a star graph.
/// # Approach
/// * find the center node
pub(crate) fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    if edges[0][1] == edges[1][0] || edges[0][0] == edges[1][0] {
        return edges[1][0];
    } else {
        return edges[1][1];
    }
}

/// # create_input_vec creates a vector of vectors of integers.
/// # Arguments
/// * `graph` - a vector of vectors of i32s
/// * `edges` - the number of edges in the graph.
/// edges is a vector of vectors of integers.
/// # Examples
/// Output: edges = [[1,2],[5,1],[1,3],[1,4]]
pub(crate) fn create_input_vec() -> Vec<Vec<i32>> {
    let mut graph = Vec::new();
    let vec = vec![1, 2];
    let vec1 = vec![5, 1];
    let vec2 = vec![1, 3];
    let vec3 = vec![1, 4];
    graph.push(vec);
    graph.push(vec1);
    graph.push(vec2);
    graph.push(vec3);
    graph
}

/* https://leetcode.com/submissions/detail/718045529/
Success
Details
Runtime: 22 ms, faster than 66.67% of Rust online submissions for Find Center of Star Graph.
Memory Usage: 9 MB, less than 20.83% of Rust online submissions for Find Center of Star Graph.
*/

#[cfg(test)]
mod tests {
    use crate::graph_star::{create_input_vec, find_center};

    #[test]
    fn test_find_center() {
        let graph = create_input_vec();
        let result = find_center(graph);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_custom_find_center() {
        let vec = vec![3, 8];
        let vec1 = vec![7, 8];
        let vec2 = vec![8, 2];
        let vec3 = vec![12, 8];
        let vec4 = vec![8, 6];
        let vec5 = vec![8, 7];
        let edges = vec![vec, vec1, vec2, vec3, vec4, vec5];

        let result = find_center(edges);
        assert_eq!(result, 8);
    }
}

// fn create_input_custom_vector(vec: Vec<Vec<i32>>, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut graph = vec;
//     graph.push(vec![edges[0][0], edges[0][1]]);

//     graph
// }
