// cspell:disable
// pub fn find_numbers(nums: Vec<i32>) -> i32 {
//     let mut count: i32 = 0;
//     for i in 0..nums.len() {
//         if String::len(&ToString::to_string(&nums[i])) % 2 == 0 {
//             count += 1;
//         }
//     }
//     count
// }
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Find Numbers with Even Number of Digits.
// Memory Usage: 2.1 MB, less than 38.89% of Rust online submissions for Find Numbers with Even Number of Digits.
//

/// Returns the number of even digits in the given vector.
///
/// # Performance
///
/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Find Numbers with Even Number of Digits.
/// Memory Usage: 2.1 MB, less than 76.39% of Rust online submissions for Find Numbers with Even Number of Digits.
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;

    for n in nums {
        let str: String = n.to_string();
        if str.len() % 2 == 0 {
            count += 1;
        }
    }
    count
}
