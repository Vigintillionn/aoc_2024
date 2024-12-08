use std::io::{self, BufRead};

pub fn read_input() -> Vec<String> {
  let stdin = io::stdin();
  stdin.lock()
    .lines()
    .map(|line| line.unwrap())
    .collect()
}