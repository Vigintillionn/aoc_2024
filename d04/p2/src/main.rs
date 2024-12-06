use util::read_input;

fn is_xmas_pattern(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
  let directions = [
      (-1, -1),         (-1, 1),
      (1, -1),          (1, 1),
  ];

  let n = grid.len();
  let m = grid[0].len();

  if grid[x][y] != 'A' {
      return false;
  }

  let mut arms = vec![];
  for &(dx, dy) in &directions {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 {
      return false;
    }

    let (nx, ny) = (nx as usize, ny as usize);

    if nx >= n || ny >= m {
      return false;
    }

    arms.push(grid[nx as usize][ny as usize]);
  }

  (arms[0] == 'M' && arms[1] == 'S' && arms[2] == 'M' && arms[3] == 'S') ||
  (arms[0] == 'S' && arms[1] == 'M' && arms[2] == 'S' && arms[3] == 'M') ||
  (arms[0] == 'M' && arms[1] == 'M' && arms[2] == 'S' && arms[3] == 'S') ||
  (arms[0] == 'S' && arms[1] == 'S' && arms[2] == 'M' && arms[3] == 'M')
}

fn count_xmas_patterns(grid: &Vec<Vec<char>>) -> usize {
  let mut count = 0;

  for x in 0..grid.len() {
    for y in 0..grid[0].len() {
      if is_xmas_pattern(grid, x, y) {
        count += 1;
      }
    }
  }

  count
}

fn main() {
  let grid = read_input();
  let result = count_xmas_patterns(&grid);

  println!("{}", result);
}
