pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    println!("points lengths is: {}", n);
    let index = -1;

    return index;
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
