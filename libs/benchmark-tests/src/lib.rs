// pub fn benchmark_tests() -> String {
//     "benchmark_tests".into()
// }

pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::bubble_sort(arr); // this seems to be quite faster than selection_sort
//    sorting::selection_sort(arr);
}

mod sorting {
    /// sorting algorithm       time:   [7.8252 ns 7.8345 ns 7.8431 ns]
    ///                        change: [+89.837% +91.856% +93.251%]
    /// (p = 0.00 < 0.05)
    ///                        Performance has regressed.
    /// Found 3 outliers among 100 measurements (3.00%)
    ///  3 (3.00%) high mild
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i..arr.len() {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    /// sorting algorithm       time:   [4.0485 ns 4.0789 ns 4.1194 ns]                        change: [-0.6276% +0.4626% +1.7415%] (p = 0.47 > 0.05)
    ///                         No change in performance detected.
    /// Found 11 outliers among 100 measurements (11.00%)
    ///   3 (3.00%) high mild
    ///   8 (8.00%) high severe
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        for i in 0..arr.len() {
            for j in 0..arr.len() - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::sorting::{bubble_sort, selection_sort};

    // #[test]
    // fn it_works() {
    // assert_eq!(benchmark_tests(), "benchmark_tests".to_string());
    // }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
}
