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
  let max_id = *disk.iter().filter(|&&v| v != u64::MAX).max().unwrap_or(&0);

  for file_id in (0..=max_id).rev() {
    let file_indices: Vec<_> = disk
      .iter()
      .enumerate()
      .filter(|&(_, &val)| val == file_id)
      .map(|(idx, _)| idx)
      .collect();

    if file_indices.is_empty() {
      continue;
    }

    let file_size = file_indices.len();
    let mut target_start = None;

    for i in 0..=disk.len() - file_size {
      if disk[i..i + file_size].iter().all(|&x| x == u64::MAX) {
        target_start = Some(i);
        break;
      }
    }

    if let Some(start) = target_start {
      if start < file_indices[0] {
        for (offset, &idx) in file_indices.iter().enumerate() {
          disk[start + offset] = file_id;
          disk[idx] = u64::MAX;
        }
      }
    }
  }

  disk.iter()
      .enumerate()
      .filter(|&(_, &val)| val != u64::MAX)
      .map(|(pos, &val)| pos as u64 * val)
      .sum()
}

fn main() {
  let lines = read_input();
  let disk = parse_input(&lines);

  let mut new_disk = build_disk(&disk);
  let result = format_disk(&mut new_disk);
  println!("{:?}", result);
  
}
