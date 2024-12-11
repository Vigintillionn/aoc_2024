use std::collections::HashMap;
use util::read_input;

const BLINKS: usize = 75;

fn has_even_digits(mut n: u64) -> bool {
  let mut digit_count = 0;
  while n > 0 {
    digit_count += 1;
    n /= 10;
  }
  digit_count % 2 == 0
}

fn split_stone(n: u64) -> (u64, u64) {
  let digits: Vec<_> = n.to_string().chars().collect();
  let mid = digits.len() / 2;

  let left = digits[..mid].iter().collect::<String>().parse::<u64>().unwrap();
  let right = digits[mid..].iter().collect::<String>().parse::<u64>().unwrap();

  (left, right)
}

fn main() {
  let stones = read_input();
  let mut stone_count: HashMap<u64, u64> = HashMap::new();

  for &stone in &stones {
    *stone_count.entry(stone).or_insert(0) += 1;
  }

  for _ in 0..BLINKS {
    let mut new_stone_counts: HashMap<u64, u64> = HashMap::new();

    for (&stone, &count) in &stone_count {
      if stone == 0 {
        *new_stone_counts.entry(1).or_insert(0) += count;
      } else if has_even_digits(stone) {
        let (left, right) = split_stone(stone);
        *new_stone_counts.entry(left).or_insert(0) += count;
        *new_stone_counts.entry(right).or_insert(0) += count;
      } else {
        *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
      }
    }

    stone_count = new_stone_counts;
  }

  let result: u64 = stone_count.values().sum();
  println!("{:?}", result);
}