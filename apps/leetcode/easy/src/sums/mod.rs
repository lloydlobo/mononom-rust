// cspell:disable
/* 1. Two Sum
Easy
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.

Example 1:
Input: nums = [2,7,11,15], target = 9 Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:
Input: nums = [3,2,4], target = 6 Output: [1,2]
Example 3:
Input: nums = [3,3], target = 6 Output: [0,1]

Constraints:
    2 <= nums.length <= 104 | -109 <= nums[i] <= 109 | -109 <= target <= 109
    Only one valid answer exists.
 */

/// # two_sum
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// param: nums: Vec<i32>, target: i32
/// return: Vec<i32>
/// # Example
/// ```rust
/// let nums = vec![2, 7, 11, 15];
/// let target = 9;
/// let output_two_sum = two_sum(nums, target);
/// assert_eq!(output_two_sum, vec![0, 1]);
/// ```
#[allow(dead_code)]
pub(crate) fn two_sum_o_n2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output_two_sum = Vec::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                output_two_sum.push(i as i32);
                output_two_sum.push(j as i32);
            }
        }
    }
    return output_two_sum;
}
/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
/// Memory Usage: 2.4 MB, less than 48.68% of Rust online submissions for Two Sum.
pub(crate) fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output_two_sum = Vec::new();
    let mut hash_map = std::collections::HashMap::new();
    for i in 0..nums.len() {
        let seen = target - nums[i];
        if hash_map.contains_key(&seen) {
            output_two_sum.push(hash_map[&seen]);
            output_two_sum.push(i as i32);
        }
        // If the map did have this key present, the value is updated, and the old value is returned.
        // The key is not updated, though; this matters for types that can be == without being identical.
        // The algorithm is randomly seeded, and a reasonable best-effort is made to generate this seed from a high quality, secure source of randomness provided by the host without blocking the program
        // - source https://doc.rust-lang.org/std/collections/struct.HashMap.html
        hash_map.insert(nums[i], i as i32);
    }
    return output_two_sum;
}
/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
Memory Usage: 2.5 MB, less than 32.06% of Rust online submissions for Two Sum.
 */
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default, Hash)]
pub(crate) struct TwoSum {
    pub(crate) nums: Vec<i32>,
    pub(crate) target: i32,
}
impl TwoSum {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_cache: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut output_indices = Vec::new();
        for i in 0..nums.len() {
            let cached: i32 = target - nums[i];
            if hash_cache.contains_key(&cached) {
                output_indices.push(i as i32);
                output_indices.push(hash_cache[&cached]);
            }
            hash_cache.insert(nums[i], i as i32);
        }
        output_indices
    }
}
/*
     Runtime: 94 ms, faster than 75.44% of TypeScript online submissions for Two Sum.
    Memory Usage: 45.7 MB, less than 25.02% of TypeScript online submissions for Two Sum.
export function twoSum(nums: number[], target: number): number[] {
    const seen = new Map();

    for (let i = 0; i < nums.length; i += 1) {
      const remaining = target - nums[i];

      if (seen.has(remaining)) {
        return [seen.get(remaining), i];
      } else {
        seen.set(nums[i], i);
      } // end if
    } // end for
    return [];
  }
   */

// #[cfg(tests)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_two_sum() {

//         let nums = vec![2, 7, 11, 15];
//         let target = 9;
//         let output_two_sum = two_sum(nums, target);
//         assert_eq!(output_two_sum, vec![0, 1]);
//     }
// }
