// cspell:ignore fibo prev curr
#[allow(unused)]
use std::collections::HashMap;

pub(crate) fn climb_stairs(n: i32) -> i32 {
    match n {
        n => {
            if n <= 45 && n >= 1 {
                recursion(n)
            } else {
                panic!("n must be <= 45")
            }
        }
    }
}

pub fn recursion(n: i32) -> i32 {
    let cache: i32;
    let mut hash_cache = HashMap::new();
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 3,
        _ => {
            if hash_cache.contains_key(&n) {
                println!("cache hit");
                cache = *hash_cache.get(&n).unwrap();
            } else {
                cache = recursion(n - 1) + recursion(n - 2);
                hash_cache.insert(n, cache);
            }
            // println!("{}", cache);
            cache
        }
    }
}

//     1 2 3 4 5  6  7  8
// 0,1,1,2,3,5,8,13,21,34

/*
Input: 1,2,3,4,5,6,7,8,9,10,45

Output: 1 2 3 5 8 13 21 34 55 89 1836311903
*/

///////////////////////////////////////////////////////////////////////////////
///
/// FIBONACCI SIMPLE METHOD
///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Climbing Stairs.
/// Memory Usage: 2 MB, less than 93.42% of Rust online submissions for Climbing Stairs.
///
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn fibo_stair(n: u64) -> u64 {
    match n {
        1 => 1,
        2 => 2,
        _ => {
            let mut curr: u64 = 2;
            let mut prev: u64 = 1;
            let mut next: u64 = curr + prev;
            for _ in 3..n {
                prev = curr;
                curr = next;
                next = curr + prev;
            }
            next
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
///
/// Runtime: 2 ms, faster than 19.74% of Rust online submissions for Climbing Stairs.
/// Memory Usage: 2.1 MB, less than 60.86% of Rust online submissions for Climbing Stairs.
///
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn fibo_basic(n: u64) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    let result: u64 = match n {
        0 => 0,
        1 => 1,
        _ => {
            for _ in 1..n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    };
    result
}

pub(crate) fn fibo_intermediate(n: u64) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    for _ in 1..n {
        let result = prev.checked_add(curr);

        match result {
            Some(next) => {
                prev = curr;
                curr = next;
            }
            None => {
                curr = 0;
                break;
            }
        } // match
    } // for
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo_stair() {
        let arr_expect = [
            0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
            10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        ];

        assert_eq!(fibo_stair(4 as u64), 5);

        for i in 4..arr_expect.len() {
            assert_eq!(fibo_stair(i as u64), arr_expect[i]);
        }
    }

    #[test]
    fn test_fibo_basic() {
        let arr_expect = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269,
        ];
        for i in 0..arr_expect.len() {
            assert_eq!(fibo_basic(i as u64), arr_expect[i]);
            assert!(fibo_basic(i as u64) <= arr_expect[i]);
            assert!(fibo_basic(i as u64) >= arr_expect[i]);
            assert_eq!(fibo_basic(i as u64), arr_expect[i]);
        }
    }

    #[test]
    fn test_fibo_intermediate() {
        let arr_expect = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269,
        ];
        for i in 1..arr_expect.len() {
            assert_eq!(fibo_intermediate(i as u64), arr_expect[i]);
            assert!(fibo_intermediate(i as u64) <= arr_expect[i]);
            assert!(fibo_intermediate(i as u64) >= arr_expect[i]);
            assert_eq!(fibo_intermediate(i as u64), arr_expect[i]);
        }
    }

    #[test]
    #[should_panic]
    fn test_fibo_recursion_climb_stairs() {
        // it panics when n is too large 0 = n > 45
        climb_stairs(46);
        climb_stairs(0);
    }
}
