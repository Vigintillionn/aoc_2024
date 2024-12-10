use util::read_input;

type Node = (usize, bool);

fn parse_input(input: &str) -> Vec<Node> {
  input.trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .enumerate()
    .map(|(i, d)| (d as usize, i % 2 == 0))
    .collect()
}

fn build_disk(disk: &[Node]) -> Vec<u64> {
  let mut output: Vec<u64> = Vec::new();
  let mut id = 0;

  for &c in disk.iter() {
    let (val, is_file) = c;
    let char = if is_file {
      let res = id as u64;
      id += 1;
      res
    } else {
      u64::MAX
    };

    for _ in 0..val {
      output.push(char);
    }
  }

  output
}

fn format_disk(disk: &mut Vec<u64>) -> u64 {
  let mut count = 0;
  let mut i = 0;
  let mut j = disk.len() - 1;

  while i < j {
    while disk[i] != u64::MAX {
      count += i as u64 * (disk[i]); 
      i += 1; 
    }
    while disk[j] == u64::MAX { j -= 1; }

    if i < j {
      disk.swap(i, j);
    }
  }

  count
}

fn main() {
  let lines = read_input();
  let disk = parse_input(&lines);

  // compact_disk(&mut disk);

  let mut new_disk = build_disk(&disk);
  let result = format_disk(&mut new_disk);

  println!("{:?}", result);
}
