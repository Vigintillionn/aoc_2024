use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<char>> {
  let stdin = io::stdin();

  stdin.lock()
    .lines()
    .map(|line| line.unwrap().chars().collect())
    .collect()
}