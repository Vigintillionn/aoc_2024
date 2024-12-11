use std::io::{self, BufRead};

pub fn read_input() -> Vec<u64> {
  let stdin = io::stdin();
  stdin.lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<u64>().unwrap())
    .collect()
}