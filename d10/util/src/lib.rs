use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<char>> {
  let stdin = io::stdin();
  stdin.lock()
    .lines()
    .filter_map(|line| line.ok())
    .map(|line| line.chars().collect())
    .collect()
}