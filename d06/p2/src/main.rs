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
) -> (Guard, HashSet<Guard>) {
  while guard_in_bounds(grid_size, guard.0) && !visited.contains(&guard) {
    visited.insert(guard);

    let next_pos = guard.1.step(guard.0);

    guard = if obstacles.contains(&next_pos) {
      (guard.0, guard.1.rotate())
    } else {
      (next_pos, guard.1)
    };
  }

  (guard, visited)
}

fn traverse_with_obstacle(
  grid_size: (usize, usize),
  obstacles: HashSet<Point>,
  guard: Guard,
) -> usize {
  let mut successful_steps: usize = 0;
  let mut checked_positions = HashSet::new();
  let mut visited_positions = HashSet::new();
  let mut current_guard = guard;

  while guard_in_bounds(grid_size, current_guard.0) && !visited_positions.contains(&current_guard) {
    let next_pos = current_guard.1.step(current_guard.0);

    if !checked_positions.contains(&next_pos)
      && next_pos != guard.0
      && guard_in_bounds(grid_size, next_pos)
    {
      let mut updated_walls = obstacles.clone();
      updated_walls.insert(next_pos);
      checked_positions.insert(next_pos);
  
      let (final_position, _) = traverse(grid_size, &updated_walls, current_guard, visited_positions.clone());
      if guard_in_bounds(grid_size, final_position.0) {
        successful_steps += 1;
      }
    }

    visited_positions.insert(current_guard);
    current_guard = if obstacles.contains(&next_pos) {
      (current_guard.0, current_guard.1.rotate())
    } else {
      (next_pos, current_guard.1)
    };
  }

  successful_steps
}

fn main() {
    let grid = read_input();
    let (guard, obstacles) = parse_map(&grid);
    let result = traverse_with_obstacle((grid[0].len(), grid.len()), obstacles, guard);

    println!("{}", result);
}