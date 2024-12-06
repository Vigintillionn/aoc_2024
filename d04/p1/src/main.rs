use util::read_input;

const DIRECTIONS: [(isize, isize); 8] = [
  (-1, -1), (-1, 0), (-1, 1),
  (0, -1),           (0, 1),
  (1, -1),  (1, 0),  (1, 1),
];

fn search_in_direction(
  grid: &Vec<Vec<char>>,
  x: usize,
  y: usize,
  dx: isize,
  dy: isize,
) -> bool {
  let word = "XMAS".chars().collect::<Vec<_>>();
  let n = grid.len();
  let m = grid[0].len();

  for (i, &ch) in word.iter().enumerate() {
    let nx = x as isize + i as isize * dx;
    let ny = y as isize + i as isize * dy;

    if nx < 0 || ny < 0 {
      return false;
    }

    let (nx, ny) = (nx as usize, ny as usize);

    if nx >= n || ny >= m || grid[nx][ny] != ch {
      return false;
    }
  }

  true
}

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
  let mut count = 0;

  for x in 0..grid.len() {
    for y in 0..grid[0].len() {
      for &(dx, dy) in DIRECTIONS.iter() {
        if search_in_direction(grid, x, y, dx, dy) {
          count += 1;
        }
      }
    }
  }

  count
}

fn main() {
  let grid = read_input();
  let result = count_xmas(&grid);

  println!("{}", result);
}
