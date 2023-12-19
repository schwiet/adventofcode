use super::util::open_file_as_bufreader;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day17/example.txt")?;

    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
      let line = line?;

      let row: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as u8)
        .collect();

      matrix.push(row);
    }

    Ok(())
}
