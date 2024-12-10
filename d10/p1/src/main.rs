use std::collections::HashSet;
use util::read_input;

fn get_neighbors(x: usize, y: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
  let mut neighbors = Vec::new();

  if x > 0 {
    neighbors.push((x - 1, y));
  }
  if y > 0 {
    neighbors.push((x, y - 1));
  }
  if x < rows - 1 {
    neighbors.push((x + 1, y));
  }
  if y < cols - 1 {
    neighbors.push((x, y + 1));
  }

  neighbors
}

fn score_trailheads(map: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
  let mut stack = Vec::new();
  let mut visited = HashSet::new();
  let mut reachable_nines = HashSet::new();
  let rows = map.len();
  let cols = map[0].len();

  stack.push(start);

  while let Some((x, y)) = stack.pop() {
    if visited.contains(&(x, y)) {
      continue;
    }

    visited.insert((x, y));

    for (nx, ny) in get_neighbors(x, y, rows, cols) {
      if visited.contains(&(nx, ny)) {
        continue;
      }

      match map[nx][ny] {
        '.' => continue,
        h if h.to_digit(10).unwrap() == map[x][y].to_digit(10).unwrap() + 1 => {
          stack.push((nx, ny));
          if h == '9' {
            reachable_nines.insert((nx, ny));
          }
        },
        _ => continue,
      }
    }
  }

  reachable_nines.len()
}

fn solution(map: &Vec<Vec<char>>) -> usize {
  let trailheads: Vec<(usize, usize)> = map
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
      row.iter()
        .enumerate()
        .filter(|&(_, &height)| height == '0')
        .map(move |(j, _)| (i, j))
    })
    .collect();

  trailheads
    .iter()
    .map(|&start| score_trailheads(map, start))
    .sum()
}

fn main() {
  let map = read_input();
  let result = solution(&map);

  println!("{:?}", result);
}