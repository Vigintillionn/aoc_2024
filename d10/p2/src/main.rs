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

fn score_trailheads(map: &Vec<Vec<char>>, start: Vec<(usize, usize)>) -> usize {
  let mut stack = vec![start];
  let mut visited_paths = HashSet::new();
  let mut trail_count = 0;
  let rows = map.len();
  let cols = map[0].len();

  while let Some(path) = stack.pop() {
    let (x, y) = *path.last().unwrap();
    
    if map[x][y] == '9' {
      if visited_paths.insert(path.clone()) {
        trail_count += 1;
      }
      continue;
    }

    for (nx, ny) in get_neighbors(x, y, rows, cols) {
      if map[nx][ny] != '.' && map[nx][ny].to_digit(10).unwrap() == map[x][y].to_digit(10).unwrap() + 1 {
        let mut new_path = path.clone();
        new_path.push((nx, ny));
        stack.push(new_path);
      }
    }
  }

  trail_count
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
    .map(|&start| score_trailheads(map, vec![start]))
    .sum()
}

fn main() {
  let map = read_input();
  let result = solution(&map);

  println!("{:?}", result);
}