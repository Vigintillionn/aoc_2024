use std::io::{self, Read};

pub fn read_input() -> String {
  let mut input = String::new();
  let stdin = io::stdin();

  stdin.lock().read_to_string(&mut input).unwrap();

  input
}