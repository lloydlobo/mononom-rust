pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let points_array = points.clone();
    let n = points_array.len();
    let mut min_distance = std::i32::MAX;
    let mut nearest_point = 0;
    for i in 0..n {
        let point: Vec<i32> = points_array[i].clone();
        let manhattan_distance = (x - point[0]).abs() + (y - point[1]).abs();
        if manhattan_distance < min_distance {
            min_distance = manhattan_distance;
            nearest_point = i;
        }
    }
    nearest_point as i32
}

// let mut index = -1;
// let max = std::i32::MAX; // 2147483647

// let mut max_distance = max;

// for i in 0..n {
//     let p = points_array[i].clone();
//     let point = p;

//     if point[0] == x && point[1] == y {
//         let x_distance = (x - point[0]).abs();
//         let y_distance = (y - point[1]).abs();
//         let manhattan_distance = x_distance + y_distance;
//         // let manhattan_distance: i32 = (x - point[0]).abs() + (y - point[1]).abs();

//         if manhattan_distance < max_distance {
//             index = i;
//             max_distance = manhattan_distance;
//         }
//     }
// }
// return index;
// }
// if max_distance == std::i32::MAX {
//     return -1;
// } else {
//     return index;

#[cfg(test)]
mod tests {
    // #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_main() {
        let mut x = 3;
        let mut y = 4;
        // [[1,2],[3,1],[2,4],[2,3],[4,4]]
        let mut points_array = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        let mut result = nearest_valid_point(x, y, points_array);
        assert!(result == 2);

        x = 2;
        y = 2;
        points_array = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        result = nearest_valid_point(x, y, points_array);
        assert!(result == -1);
    }
}

/*
....###....########...######..##.....##.####.##.....##.########
...##.##...##.....##.##....##.##.....##..##..##.....##.##......
..##...##..##.....##.##.......##.....##..##..##.....##.##......
.##.....##.########..##.......#########..##..##.....##.######..
.#########.##...##...##.......##.....##..##...##...##..##......
.##.....##.##....##..##....##.##.....##..##....##.##...##......
.##.....##.##.....##..######..##.....##.####....###....########
 */

/*
function nearestValidPoint(x: number, y: number, points_array: number[][]): number {
  const n = points_array.length;
  let index = -1;
  let D = Number.MAX_VALUE;
  for (let i = 0; i < n; i += 1) {
    if (points_array[i][0] === x || points_array[i][1] === y) {
      const manD = Math.abs(x - points_array[i][0]) + Math.abs(y - points_array[i][1]);
      if (manD < D) {
        index = i;
        D = manD;
      }
    }
  }
  return index;
}
 */
