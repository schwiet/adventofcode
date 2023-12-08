use super::util::open_file_as_bufreader;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Directions {
    left: String,
    right: String,
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let mut reader = open_file_as_bufreader("src/day8/input.txt")?;
    let mut directions = String::new();

    reader.read_line(&mut directions)?;
    // Trim whitespace, including newlines, from the `directions` string
    directions = directions.trim().to_string();

    // start positions
    let pattern = Regex::new(r"A$").unwrap();
    // Create an empty Vec<String>
    let mut positions: Vec<String> = Vec::new();

    let mut positions_map: HashMap<String, Directions> = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let lines = reader.lines();
    let pos_lines = lines.skip(1);
    // Iterate over each line in the file
    for line in pos_lines {
        let line = line?; // Using ? here to propagate error

        if let Some(captures) = re.captures(&line) {
            // Splitting the line on whitespace
            // let tokens: Vec<&str> = line.split_whitespace().collect();
            let position = captures.get(1).unwrap().as_str();
            let left = captures.get(2).unwrap().as_str();
            let right = captures.get(3).unwrap().as_str();

            let directions = Directions {
                left: left.to_string(),
                right: right.to_string(),
            };
            positions_map.insert(position.to_string(), directions);

            if let Some(start_pos) = re.captures(position) {
                positions.push(start_pos.get(1).unwrap().as_str().to_string());
            }
        } else {
            println!("{line} didn't match regex")
        }

        // println!("line - {line}");
    }

    let pos_count: u32 = positions.len();
    let mut z_count: u32 = 0;
    let mut found: bool = false;
    let mut steps: u64 = 0;
    while !found {
        for dir in directions.chars() {
            if pos == "ZZZ" {
                found = true;
                println!("steps = {steps}");
                return Ok(());
            } else {
                // print!("visited {pos}");
                steps += 1;
            }
            // Accessing values by position
            if let Some(dirs) = positions_map.get(pos) {
                // println!(", next {dir}");
                match dir {
                    'L' => {
                        pos = &dirs.left;
                    }
                    'R' => {
                        pos = &dirs.right;
                    }
                    _ => {
                        println!("Direction not found - {pos}");
                        return Ok(());
                    }
                }
            } else {
                println!("Position not found - {pos}");
                return Ok(());
            }
        }

        print!("\r");
        print!("{steps} steps so far...");
    }

    Ok(())
}
