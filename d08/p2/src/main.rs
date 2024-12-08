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
      let (l, r) = (couple[0], couple[1]);

      // The following code is a direct translation of the mathematical
      // solution to the problem.
      // y = mx + b
      // m = (y2 - y1) / (x2 - x1)
      // b = y1 - m * x1
      // x = (b2 - b1) / (m1 - m2)
      // y = m1 * x + b1

      let m = (r.0 as isize - l.0 as isize, r.1 as isize - l.1 as isize);
      let b = (l.0 as isize * m.1 - m.0 * l.1 as isize, m.1);
    
      // If the line is vertical, we need to handle it separately.
      // We iterate over the x-axis and calculate the corresponding y-axis
      // value. If the y-axis value is within the grid, we add it to the
      // antinodes set. 
      if m.1 == 0 {     
        for y in 0..bottom_right.0 {
          antinodes.insert((y, l.1));
        }
      } else {
        // Otherwise, we iterate over the y-axis and calculate the
        // corresponding x-axis value. If the x-axis value is within the
        // grid, we add it to the antinodes set.
        for x in 0..bottom_right.1 {
          let p = (m.0 * x as isize + b.0, m.1);
          if p.0 % p.1 == 0 {
            let y = (p.0 / p.1) as usize;
            if y < bottom_right.0 {
              antinodes.insert((y, x));
            }
          }
        }
      }
    }
  }

  antinodes.len()
}

fn main() {
  let grid = read_input();
  let width = grid[0].len();
  let height = grid.len();

  let antennas = parse_input(grid);
  let antinodes = find_antinodes(width, height, antennas);

  println!("{}", antinodes);
}
