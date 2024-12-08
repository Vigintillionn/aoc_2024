use itertools::Itertools;
use util::read_input;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

fn parse_input(grid: Vec<String>) -> HashMap<char, Vec<Point>> {
  let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

  for (i, line) in grid.iter().enumerate() {
    for (j, c) in line.chars().enumerate() {
      if c.is_ascii_alphanumeric() {
        let entry = antennas.entry(c).or_default();
        entry.push((i, j));
      }
    }
  }

  antennas
}

fn find_antinodes(width: usize, height: usize, antennas: HashMap<char, Vec<Point>>) -> usize {
  let bottom_right = (height, width);
  let mut antinodes = HashSet::new();

  for points in antennas.values() {
    for couple in points.iter().combinations(2) {
      let (l, r) = (couple[0].min(couple[1]), couple[0].max(couple[1]));
      let (dx, dy) = (l.0.abs_diff(r.0), l.1.abs_diff(r.1));

      if dx <= l.0 {
        if l.1 <= r.1 && dy <= l.1 {
          antinodes.insert((l.0 - dx, l.1 - dy));
        }
        if l.1 > r.1 && l.1 + dy < bottom_right.1 {
          antinodes.insert((l.0 - dx, l.1 + dy));
        }
      }

      if r.0 + dx < bottom_right.0 {
        if r.1 <= l.1 && dy <= r.1 {
          antinodes.insert((r.0 + dx, r.1 - dy));
        }
        if r.1 > l.1 && r.1 + dy < bottom_right.1 {
          antinodes.insert((r.0 + dx, r.1 + dy));
        }
      }
    }
  }

  antinodes.len()
}

fn main() {
  let lines = read_input();
  let width = lines[0].len();
  let height = lines.len();

  let antennas = parse_input(lines);
  let antinodes = find_antinodes(width, height, antennas);

  println!("{}", antinodes);
}
