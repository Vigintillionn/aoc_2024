use std::io::{self, BufRead};

pub fn read_input() -> (Vec<i32>, Vec<i32>) {
  let stdin = io::stdin();
  let mut left = Vec::new();
  let mut right = Vec::new();

  for line in stdin.lock().lines() {
    let line = line.unwrap();
    let numbers: Vec<i32> = line
      .split_whitespace()
      .map(|x| x.parse::<i32>().unwrap())
      .collect();
    if numbers.len() == 2 {
      left.push(numbers[0]);
      right.push(numbers[1]);
    }
  }

  (left, right)
}