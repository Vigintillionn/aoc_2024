use util::read_input;
use regex::Regex;

fn solution(string: String) -> i32 {
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

  let mut total_sum = 0;

  for cap in re.captures_iter(&string) {
    let a = cap[1].parse::<i32>().unwrap();
    let b = cap[2].parse::<i32>().unwrap();

    total_sum += a * b;
  }

  total_sum
}

fn main() {
  let input = read_input();
  let result = solution(input);

  println!("{}", result);
}
