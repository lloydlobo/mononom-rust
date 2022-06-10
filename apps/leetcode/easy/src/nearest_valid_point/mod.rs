pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let index = -1;
    let mut max_distance = std::i32::MAX; // 2147483647
    for _ in 0..n {
        println!("{} {} {}", x, y, points[0][0]);
        max_distance = 1;
    }
    println!(
        "index: {}, len: {}, max_distance: {}",
        index, n, max_distance
    );

    return index;
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[allow(unused_mut)]
    fn test_main() {
        let mut x = 3;
        let mut y = 4;
        // [[1,2],[3,1],[2,4],[2,3],[4,4]]
        let mut points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        let mut result = nearest_valid_point(x, y, points);
        assert!(result == -1);

        x = 2;
        y = 2;
        points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        result = nearest_valid_point(x, y, points);
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
function nearestValidPoint(x: number, y: number, points: number[][]): number {
  const n = points.length;
  let index = -1;
  let D = Number.MAX_VALUE;
  for (let i = 0; i < n; i += 1) {
    if (points[i][0] === x || points[i][1] === y) {
      const manD = Math.abs(x - points[i][0]) + Math.abs(y - points[i][1]);
      if (manD < D) {
        index = i;
        D = manD;
      }
    }
  }
  return index;
}
 */
