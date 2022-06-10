use crate::{
    graph_star::{create_input_vec, find_center},
    nearest_valid_point::nearest_valid_point,
    random_vector::random_duplicate_vector,
};
mod graph_star;
pub mod nearest_valid_point;
mod random_vector;

// region:      --- main ---

fn main() {
    main_nearest_valid_point();

    // let x = 3;
    // let y = 4;
    // let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
    // let points_array = points.clone();
    // let len = points_array.len();
    // for i in 0..len {
    //     let point: Vec<i32> = points_array[i].clone();
    //     let manhattan_distance = (x - point[0]).abs() + (y - point[1]).abs();
    //     println!("manhattan_distance: {}", manhattan_distance);
    // }
}

// endregion:    --- main ---
#[allow(dead_code)]
fn main_nearest_valid_point() {
    println!("\n\nHello, world!");
    let mut x = 3;
    let mut y = 4;
    println!("\n\nx: {}, y: {}", x, y);

    let mut points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
    println!("points: {:?}", points);
    let nearest_point = nearest_valid_point(x, y, points);
    println!("\n2 -> nearest_point is: {}", nearest_point);

    x = 3;
    y = 4;
    println!("\n\nx: {}, y: {}", x, y);

    points = vec![vec![3, 4]];
    println!("points: {:?}", points);
    let nearest_point = nearest_valid_point(x, y, points);
    println!("\n0 -> nearest_point is: {}", nearest_point);

    x = 3;
    y = 4;
    println!("\n\nx: {}, y: {}", x, y);
    points = vec![vec![2, 3]];
    println!("points: {:?}", points);
    let nearest_point = nearest_valid_point(x, y, points);
    println!("\n-1 -> nearest_point is: {}", nearest_point);
}

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

#[allow(dead_code)]
fn main_find_center() {
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

// endregion:    --- graph_star ---
