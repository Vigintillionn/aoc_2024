use util::read_input;

fn is_monotonic(levels: &[i32]) -> bool {
  let increasing = levels.windows(2).all(|w| w[0] < w[1]);
  let decreasing = levels.windows(2).all(|w| w[0] > w[1]);

  increasing || decreasing
}

fn valid_difference(levels: &[i32]) -> bool {
  levels.windows(2)
    .all(|w| (1..=3).contains(&(w[1] - w[0]).abs()))
}

fn is_safe(levels: &[i32]) -> bool {
  is_monotonic(levels) && valid_difference(levels)
}

fn solution(reports: Vec<Vec<i32>>) -> i32 {
  reports.iter()
    .filter(|report| is_safe(report))
    .count()
    .try_into() 
    .unwrap()
}

fn main() {
    let reports = read_input();
    let result = solution(reports);

    println!("{}", result);
}
