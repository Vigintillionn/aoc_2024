use util::read_input;

fn evaluate_expression(numbers: &[i64], operators: &[char]) -> i64 {
  let mut result = numbers[0];
  for (i, &operator) in operators.iter().enumerate() {
    match operator {
      '+' => result += numbers[i + 1],
      '*' => result *= numbers[i + 1],
      '|' => {
        let concatenated = format!("{}{}", result, numbers[i + 1]);
        result = concatenated.parse().unwrap();
      }
      _ => unreachable!(),
    }
  }
  result
}

fn generate_operator_combinations(len: usize) -> Vec<Vec<char>> {
  let mut combinations = vec![];
  let total_combinations = 3usize.pow(len as u32);
  
  for i in 0..total_combinations {
    let mut combination = vec![];
    let mut current: i32 = i as i32;
    for _ in 0..len {
      let operator = match current % 3 {
        0 => '+',
        1 => '*',
        2 => '|',
        _ => unreachable!(),
      };
      combination.push(operator);
      current /= 3;
    }
    combinations.push(combination);
  }
  combinations
}

fn process_equations(lines: Vec<String>) -> i64 {
  let mut total = 0;

  for line in lines {
    let mut parts = line.split(": ");
    let target: i64 = parts.next().unwrap().parse().unwrap();
    let numbers: Vec<i64> = parts.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect(); 

    let operators = generate_operator_combinations(numbers.len() - 1);
    let mut is_valid = false;

    for operator in operators {
      if evaluate_expression(&numbers, &operator) == target {
        is_valid = true;
        break;
      }
    }

    if is_valid {
      total += target;
    }
  }

  total
}

fn main() {
  let lines = read_input();
  let result = process_equations(lines);

  println!("{}", result);
}
