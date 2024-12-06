use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<i32>> {
  let stdin = io::stdin();
  stdin.lock()
    .lines()
    .map(|line| {
      line.unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
    })
    .collect()
}