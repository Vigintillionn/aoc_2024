use util::read_input;

fn solution(left: Vec<i32>, right: Vec<i32>) -> i32 {
  let mut left_sorted = left.clone();
  let mut right_sorted = right.clone();
  left_sorted.sort_unstable();
  right_sorted.sort_unstable();

  left_sorted
    .iter()
    .zip(right_sorted.iter())
    .map(|(l, r)| (l-r).abs())
    .sum()
}

fn main() {
    let (left, right) = read_input();
    let result = solution(left, right);

    println!("{}", result);
}
