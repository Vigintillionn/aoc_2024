use std::cmp::Ordering;

use util::read_input;

fn compare(x: i32, y: i32, rules: &[(i32, i32)]) -> Ordering {
  if rules.contains(&(x, y)) {
    return Ordering::Less;
  } else if rules.contains(&(y, x)) {
    return Ordering::Greater;
  } else {
    return Ordering::Equal;
  }
}

fn solution(input: &str) -> i32 {
  let sections: Vec<&str> = input.split("\n\n").collect();
  
  let rules: Vec<(i32, i32)> = sections[0]
    .lines()
    .map(|line| {
      let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
      (parts[0], parts[1])
    })
    .collect();

  let updates: Vec<Vec<i32>> = sections[1]
    .lines()
    .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
    .collect();

  updates.iter()
    .filter(|update| {
      update.iter()
        .zip(update.iter().skip(1))
        .all(|(x, y)| compare(*x, *y, &rules) == Ordering::Less)
    })
    .map(|update: &Vec<i32>| update[update.len() / 2])
    .sum()
}

fn main() {
  let input = read_input();
  let result = solution(&input);

  println!("{}", result);
}
