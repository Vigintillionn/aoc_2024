use util::{guard_in_bounds, read_input, Direction, Guard, Point};
use std::collections::HashSet;

fn find_guard(input: &Vec<String>) -> Guard {
  let mut guard = None;

  for (y, line) in input.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      if let Some(dir) = Direction::from_char(c) {
        guard = Some(((x as isize, y as isize), dir));
      }
    }
  }

  guard.unwrap()
}

fn find_obstacles(input: &Vec<String>) -> HashSet<Point> {
  let mut obstacles = HashSet::new();

  for (y, line) in input.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      if c == '#' {
        obstacles.insert((x as isize, y as isize));
      }
    }
  }

  obstacles
}

fn parse_map(input: &Vec<String>) -> (Guard, HashSet<Point>) {
  let guard = find_guard(input);
  let obstacles = find_obstacles(input);

  (guard, obstacles)
}

fn traverse(
  grid_size: (usize, usize),
  obstacles: &HashSet<Point>,
  mut guard: Guard,
  mut visited: HashSet<Guard>,
) -> HashSet<Guard> {
  while guard_in_bounds(grid_size, guard.0) && !visited.contains(&guard) {
    visited.insert(guard);

    let next_pos = guard.1.step(guard.0);

    guard = if obstacles.contains(&next_pos) {
      (guard.0, guard.1.rotate())
    } else {
      (next_pos, guard.1)
    };
  }

  visited
}

fn main() {
  let grid = read_input();
  let (guard, obstacles) = parse_map(&grid);

  let visited = traverse(
    (grid[0].len(), grid.len()), 
    &obstacles, 
    guard, 
    HashSet::new()
  );
  let result = visited.iter().map(|g| g.0).collect::<HashSet<_>>().len();

  println!("{}", result);
}
