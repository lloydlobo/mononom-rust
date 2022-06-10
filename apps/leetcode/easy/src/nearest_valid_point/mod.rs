/// nearest valid point
/// # About
/// - It is a function that returns the nearest valid point to the given point.
/// - param: `x`: x coordinate of the given point
/// - param: `y`: y coordinate of the given point
/// - param: `points`: a list of points
/// # Example
/// ```
/// let mut x = 3;
/// let mut y = 4;
/// println!("\n\nx: {}, y: {}", x, y);
/// let mut points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
/// println!("points: {:?}", points);
/// let nearest_point = nearest_valid_point(x, y, points);
/// println!("\n2 -> nearest_point is: {}", nearest_point);
/// ```
/// - x: 3, y: 4
/// - points: [[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]]
/// - 2 -> nearest_point is: 2

/// - Runtime: 30 ms, faster than 13.73% of Rust online submissions for Find Nearest Point That Has the Same X or Y Coordinate.
/// - Memory Usage: 3.7 MB, less than 15.69% of Rust online submissions for Find Nearest Point That Has the Same X or Y Coordinate.
// pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
//     let points_array = points.clone();
//     let n = points_array.len();
//     let mut min_distance = std::i32::MAX;
//     let mut nearest_point = -1 as i32;
//     for i in 0..n {
//         let point: Vec<i32> = points_array[i].clone();
//         if x == point[0] || y == point[1] {
//             let manhattan_distance = (x - point[0]).abs() + (y - point[1]).abs();
//             if manhattan_distance < min_distance {
//                 min_distance = manhattan_distance;
//                 nearest_point = i as i32;
//             }
//         }
//     }
//     nearest_point as i32
// }

/// Runtime: 27 ms, faster than 31.37% of Rust online submissions for Find Nearest Point That Has the Same X or Y Coordinate.
/// Memory Usage: 2.8 MB, less than 84.31% of Rust online submissions for Find Nearest Point That Has the Same X or Y Coordinate.
pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut min = std::i32::MAX;
    let mut loc = -1;
    for i in 0..n {
        let point = points[i].clone();
        if x == point[0] || y == point[1] {
            let dist = (x - point[0]).abs() + (y - point[1]).abs();
            if dist < min {
                min = dist;
                loc = i as i32;
            }
        }
    }
    loc
}

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
