use std::io::{self, BufRead};

pub type Point = (isize, isize);
pub type Guard = (Point, Direction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn step(&self, pos: Point) -> Point {
    match self {
      Direction::Up => (pos.0, pos.1 - 1),
      Direction::Down => (pos.0, pos.1 + 1),
      Direction::Left => (pos.0 - 1, pos.1),
      Direction::Right => (pos.0 + 1, pos.1),
    }
  }

  pub fn rotate(&self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }

  pub fn from_char(c: char) -> Option<Direction> {
    match c {
      '^' => Some(Direction::Up),
      'v' => Some(Direction::Down),
      '<' => Some(Direction::Left),
      '>' => Some(Direction::Right),
      _ => None,
    }
  }
}

pub fn read_input() -> Vec<String> {
  let stdin = io::stdin();
  stdin.lock().lines().map(|line| line.unwrap()).collect()
}

pub fn guard_in_bounds(grid_size: (usize, usize), coords: Point) -> bool {
  coords.0 >= 0 && coords.0 < grid_size.0 as isize && coords.1 >= 0 && coords.1 < grid_size.1 as isize
}