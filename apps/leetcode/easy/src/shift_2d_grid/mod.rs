pub(crate) fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let grid_clone = grid.clone();
    let mut grid_new: Vec<Vec<i32>> = Vec::new();
    let mut new_flat_grid_array = Vec::<i32>::new();
    let len = grid.len();

    let mut flat_grid_array: Vec<i32> = grid_clone
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    for _ in 0..k {
        let last = {
            match flat_grid_array.pop() {
                Some(val) => val,
                None => 123,
            }
        };
        new_flat_grid_array.push(last);
        println!("new_flat_grid_array: {:?}", new_flat_grid_array);
    }
    new_flat_grid_array.reverse();
    new_flat_grid_array.extend_from_slice(&flat_grid_array);

    if (grid[0].len() as i32) == 1 {
        for i in 0..len {
            grid_new.push(vec![new_flat_grid_array[i as usize]]);
        }
    } else {
        for i in 0..len {
            let mut count = grid_clone[i].len();
            let new_grid_slice = &new_flat_grid_array[(i * count)..i * count + count];
            grid_new.push(new_grid_slice.to_vec());
            count += len;
        }
    }

    grid_new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_grid() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let result = shift_grid(grid, k);

        assert!(result == vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
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
