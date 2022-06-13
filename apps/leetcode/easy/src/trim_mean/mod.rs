///////////////////////////////////////////////////////////////////////////////
///
/// @title Trim Mean
///
/// @author anonymous
/// @date 20022-06-13
/// @url https://leetcode.com/problems/trim-mean/
///
/// @problem_description
/// Given an integer array arr, return the mean of the remaining integers
/// after removing the smallest 5% and the largest 5% of the elements.
/// Answers within 10-5 of the actual answer will be considered accepted.
///
/// @constraints
/// 20 <= arr.length <= 1000
/// arr.length is a multiple of 20.
/// 0 <= arr[i] <= 105
///////////////////////////////////////////////////////////////////////////////
use core::panic;

/// Returns the mean of the values of an array trimmed by `cent` percent from both ends.
///
/// Given an integer array arr, return the mean of the remaining integers
/// after removing the smallest `cent` (5%) and the largest `cent` (5%) of the elements.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use trim_mean::trim_mean;
/// let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
/// let cent = 5;
/// let output_trim_mean = trim_mean(arr, cent);
/// assert!(output_trim_mean == 10.5);
/// ```
/// # Performance
///
/// - Time Complexity:
/// O(n)
/// - Space Complexity:
/// O(1)
/// - Runtime:
/// 0 ms, faster than 100.00% of Rust online submissions for Mean of Array After Removing Some Elements.
/// - Memory Usage:
/// 2.2 MB, less than 66.67% of Rust online submissions for Mean of Array After Removing Some Elements.
///
/// # Arguments
///
/// * `arr` - The array of values to be trimmed.
/// * `cent` - The percentage of values to be trimmed.
///
/// # Return
///
/// The mean of the values of an array trimmed by cent% from both ends.
/// * -> f64
pub fn trim_mean(arr: Vec<i32>, cent: usize) -> f64 {
    let mut percent: usize = cent;

    if let Some(value) = validate_arr(&arr, &mut percent) {
        return value;
    }

    let mut arr: Vec<i32> = arr;
    arr.sort();

    let trim: usize = (percent * arr.len()) / 100;
    let arr_trim_start_end: &[i32] = &arr[trim..arr.len() - trim];

    let sum: i32 = arr_trim_start_end.iter().sum::<i32>();
    let arr_new_len: usize = arr_trim_start_end.len();

    let mean: f64 = sum as f64 / arr_new_len as f64;
    mean
}

/// Validates the constraints of the problem with the given array.
///
/// # Arguments
///
/// * `arr` - The array of values to be trimmed.
/// * `percent` - The percentage of values to be trimmed,
/// which is borrowed from central function's `cent` argument.
///
/// # Return
///
/// ## Cases:
///
/// 1. If `arr` is empty or `arr` is not a multiple of 20, or `arr` length is less than 20 or greater than 1000, return `None`.
/// 2. If `percent` is greater than 50, return `None`.
/// 3. If `percent` is less than 0, return `None`.
/// 4. If `percent` is equal to 50, return `None`.
/// 5. If `percent` is equal to 0, return the sum of all values of `arr`.
///
fn validate_arr(arr: &Vec<i32>, percent: &mut usize) -> Option<f64> {
    if arr.len() % 20 != 0 {
        panic!("The length of the array must be a multiple of 20.");
    }
    if arr.len() < 20 && arr.len() > 1001 {
        panic!("The length of the array must be between 20 and 1000.");
    }
    if *percent > 50
        || *percent < 0 as usize
        || *percent == f32::INFINITY as usize
        || *percent == f32::NAN as usize
    {
        panic!("The percentage must be between 0 and 50.");
    }
    // panic if usize is a fractional float like 1 / 2 as usize
    // if *percent < 9 as usize / 10 as usize && *percent != 0 {
    //     panic!("The percentage must be between 0 and 50.");
    // }

    // comparison is useless due to type limits -- when usize < 0, it is always false
    if *percent == 50 {
        return Some(0.0);
    }
    if *percent == 0 {
        return Some(arr.iter().sum::<i32>() as f64);
    }

    // if we get here, we have a valid array and a valid percent
    // but if the values of `arr` are less than 0 or greater than 104
    let invalid_arr = arr.iter().any(|v| v < &0 || v > &104); // tip: its `any` function is a bit faster than `all`

    if invalid_arr {
        panic!("Invalid array! The values of the array must be between 0 and 104.");
    }
    None
} // end of trim_mean

///////////////////////////////////////////////////////////////////////////////
///
/// @title Unit Tests
///
/// @date 20022-06-13
///
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let cent: usize = 5;
        let output_trim_mean: f64 = trim_mean(
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ],
            cent,
        );
        assert!(output_trim_mean == 10.5);
    } // test_basic

    #[test]
    fn test_trim_mean() {
        let percent: usize = 10;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]; // arr
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 10.5);
    } // test_trim_mean

    #[test]
    fn test_trim_mean_2() {
        let percent: usize = 20;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]; // arr
        let output_trim_mean = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 10.5);
    } // test_trim_mean_2

    #[test]
    fn test_trim_mean_3() {
        let percent: usize = 5;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]; // arr
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 50.5);
    } // test_trim_mean_3

    #[test]
    fn test_trim_mean_4() {
        let percent: usize = 49;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]; // arr
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 50.5);
    } // test_trim_mean_4

    #[test]
    #[should_panic(expected = "The percentage must be between 0 and 50.")]
    fn test_trim_mean_cent_more_than_49() {
        let percent: usize = 59;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]; // arr
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 50.5);
        // add assertion that trim_mean will panic
    } // test_trim_mean_cent_more_than_49

    #[test]
    #[should_panic(expected = "The percentage must be between 0 and 50.")]
    fn test_trim_mean_panics() {
        let percent: usize = 0;
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]; // arr
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 50.5);
    } // test_trim_mean_panics

    #[test]
    #[should_panic(expected = "Invalid array! The values of the array must be between 0 and 104.")]
    fn test_trim_mean_invalid_values() {
        let percent: usize = 5;
        // here some values in the array are greater than 104 and less than 0
        // create array of size of multiple of 20, where some values elements are greater or less than 0 and 104, e.g. [-1, 105, 2 , 2999 ...]
        let cent: usize = percent;
        let arr: Vec<i32> = vec![
            -1, 105, 2, 2999, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
            67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
            89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]; // arr

        // let valid_arr = arr.iter().any(|v| v <= &0 && v >= &104);
        // println!("true or false: {:?}", valid_arr);
        println!("arr.len(): {}", arr.len());
        // assert_eq!(arr.len(), 20 * 20); // panic message "assertion failed"
        // assert!(arr.iter().all(|&x| x >= 0 && x <= 104));
        let output_trim_mean: f64 = trim_mean(arr, cent);
        assert_eq!(output_trim_mean, 52.5);
    }
} // mod tests
