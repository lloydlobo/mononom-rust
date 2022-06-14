// cspell:ignore curr
// region:      --- USE | MOD ---
use crate::{
    climbing_stairs::{fibo_basic, fibo_intermediate},
    // even_digits::find_numbers,
    graph_star::{create_input_vec, find_center},
    linked_lists::ListNode,
    nearest_valid_point::nearest_valid_point,
    random_vector::random_duplicate_vector,
};
// use linked_lists::remove_elements;
use sums::two_sum;
mod climbing_stairs;
mod even_digits;
mod graph_star;
pub mod linked_lists;
pub mod nearest_valid_point;
mod random_vector;
mod robot_origin;
pub mod sums;
mod trim_mean;
// endregion:   --- USE | MOD ---

// region:      --- main ---
fn main() {
    // let head = create_list_elements();
    // println!("remove_element: {:?}", remove_elements(head, 6));
    // clearscreen::clear().expect("failed to clear screen");

    // println!( "find_numbers: {:?}", find_numbers(vec![ 12, 10, 14, 1, 15, 20, 4000, 30, 40, 50, 60, 70, 80, 90, 100 ])); let trim_mean = trim_mean::trim_mean( vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, ], 1,); println!("trim_mean: {}", trim_mean); let new_vec = [ 4, 8, 4, 10, 0, 7, 1, 3, 7, 8, 8, 3, 4, 1, 6, 2, 1, 1, 8, 0, 9, 8, 0, 3, 9, 10, 3, 10, 1, 10, 7, 3, 2, 1, 4, 9, 10, 7, 6, 4, 0, 8, 5, 1, 2, 1, 6, 2, 5, 0, 7, 10, 9, 10, 3, 7, 10, 5, 8, 5, 7, 6, 7, 6, 10, 9, 5, 10, 5, 5, 7, 2, 10, 7, 7, 8, 2, 0, 1, 1, ]; let new_vec_trim_mean = trim_mean::trim_mean(new_vec.to_vec(), 5); println!("new_vec_trim_mean: {}", new_vec_trim_mean); // new_vec_trim_mean: 5.277777777777778 // assert!(new_vec_trim_mean == 5.29167);
    // climb_stairs
    // println!("climb_stairs: {}", climbing_stairs::climb_stairs(10));
    // println!("climb_stairs: {}", climbing_stairs::climb_stairs(10));
    println!("fibonacci_basic: {}", fibo_basic(93)); // this works fine!!! n near millions
    println!("fibonacci_intermediate: {}", fibo_intermediate(93)); // returns 0 at n > 93 (prev.checked_add(curr) helps to wrap the value to 0 after the stack overflows)
}

// endregion:    --- main ---

fn create_list_elements() -> Option<Box<ListNode>> {
    let head: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: None,
                                    // end of list
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    head
} // println!("head: {:?}", head); // copilot you are amazing!!!! (: // end of list

#[allow(dead_code)]
fn main_two_sum() {
    // cspell:disable
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output_two_sum = two_sum(nums, target);
    println!("output_two_sum: {:?}", output_two_sum);
    // cspell:enable
}

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

// let new_number = 1___________0______________1________________u8;
// println!("new_number: {}", new_number); // new_number: 101
// #[allow(unused_variables)]
// let new_number_dense: i64 = 9876543210;
// #[allow(unused_variables)]
// let new_number_concise: i64 = 9_876_543_210;

// // f32, f64  --> divide by 8 to get the number of bits
// #[allow(unused_variables)]
// let new_float = 5.5;
// let new_float_f32 = 5.2 as f32;
// let sum_new_float: f32 = new_float + new_float_f32; // rust changes the trait of new_float to f32 from the default f64
// println!("sum_new_float: {}", sum_new_float); // sum_new_float: 11.7
