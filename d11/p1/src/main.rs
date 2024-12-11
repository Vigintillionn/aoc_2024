use util::read_input;

const BLINKS: usize = 25;

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
  let mut stones = read_input();

  for _ in 0..BLINKS {
    let mut new_stones = Vec::new();
    for &stone in &stones {
      if stone == 0 {
        new_stones.push(1);
      } else if has_even_digits(stone) {
        let (left, right) = split_stone(stone);
        new_stones.push(left);
        new_stones.push(right);
      } else {
        new_stones.push(stone * 2024);
      }
    }
    stones = new_stones;
  }

  println!("{:?}", stones.len());
}