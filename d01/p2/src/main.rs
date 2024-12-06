use util::read_input;
use std::collections::HashMap;

fn solution(left: Vec<i32>, right: Vec<i32>) -> i32 {
  let mut right_freq = HashMap::new();
  for &n in &right {
    *right_freq.entry(n).or_insert(0) += 1;
  }

  left.iter()
    .map(|&num| num * right_freq.get(&num).unwrap_or(&0))
    .sum()
}

fn main() {
  let (left, right) = read_input();
  let result = solution(left, right);

  println!("{}", result);
}
