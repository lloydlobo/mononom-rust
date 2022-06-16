pub(crate) fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    println!("k: {}, grid: {:?}", k, grid);
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_grid() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let result = shift_grid(grid, k);
        assert_ne!(result, vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
    }
}

// ARCHIVE

// impl Solution {
// pub(crate) fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
//     let len = grid.len();
//     let grid_borrow_flatten = grid.iter().flat_map(|x| x.iter()).collect::<Vec<i32>>();
//     let new_shifted_grid = vec![];
//     for i in 0..k {
//         let last = grid_borrow_flatten.pop().unwrap();
//         grid_borrow_flatten.insert(0, last);
//     }

//     if grid[0].len() == 1 {
//         let count = 1;
//         for i in 0..grid_borrow_flatten.len() {
//             let new_grid = vec![grid_borrow_flatten[i..i + count].to_vec()];
//             new_shifted_grid.push(new_grid);
//             count += 1;
//         }
//     } else {
//         for i in 0..len {
//             let count = grid[0].len();
//             new_shifted_grid.push(grid_borrow_flatten[i * count..i * count + count].to_vec());
//             count += len;
//         }
//     }
//     new_shifted_grid
// }
// }
