use util::read_input;
use regex::Regex;

fn solution(input: String) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

  let mut enabled = true;
  let mut total_sum = 0;

  for caps in re.captures_iter(&input) {
      let matched = caps.get(0).unwrap().as_str();
      match matched {
          "do()" => enabled = true,
          "don't()" => enabled = false,
          _ => {
              if enabled {
                  let x: i32 = caps[1].parse().unwrap();
                  let y: i32 = caps[2].parse().unwrap();
                  total_sum += x * y;
              }
          }
      }
  }

  total_sum
}

fn main() {
  let input = read_input();
  let result = solution(input);

  println!("{}", result);
}
