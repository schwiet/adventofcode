use super::util::open_file_as_bufreader;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Directions {
    left: String,
    right: String,
}

fn find_loop(positions: &HashMap<String, Directions>, directions: String, start: String) -> u64 {
    let ends_z = Regex::new(r".*Z$").unwrap();

    let mut steps = 0u64;
    let mut visited: HashMap<String, u64> = HashMap::new();
    let mut next = start;

    let mut z_found: Option<String> = None;
    loop {
        for direction in directions.chars() {
            
            // From the current position get the next position, based on
            // direction
            if let Some(next_step) = positions.get(&next) {
                next = match direction {
                    'L' => next_step.left.clone(),
                    'R' => next_step.right.clone(),
                    _ => {
                        return 0;
                    }
                }
            }

            // keep track of how many steps have been taken
            steps += 1;

            // keep track of when position ending with Z is found
            if let Some(captures) = ends_z.captures(&next) {
                let z_str = captures.get(0).unwrap().as_str().to_string();
                z_found = Some(z_str);
            }

            // if we detected a loop and there is a Z-ending node in the loop
            if visited.get(&next) != None && z_found != None {
                println!("Loop including Z Detected at {next} after {steps} steps");
                return steps - 1;
            }

            visited.insert(next.clone(), steps);
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_set(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().reduce(|a, b| lcm(a, b)).unwrap_or(1)
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let mut reader = open_file_as_bufreader("src/day8/input.txt")?;
    let mut directions = String::new();

    reader.read_line(&mut directions)?;
    // Trim whitespace, including newlines, from the `directions` string
    directions = directions.trim().to_string();

    // Create an empty Vec<String>
    let mut positions: Vec<String> = Vec::new();

    let mut positions_map: HashMap<String, Directions> = HashMap::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let ends_a = Regex::new(r".*A$").unwrap();

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

            if let Some(start_pos) = ends_a.captures(&position) {
                positions.push(start_pos.get(0).unwrap().as_str().to_string());
            }
        } else {
            println!("{line} didn't match regex")
        }

        // println!("line - {line}");
    }

    let mut loop_lengths: Vec<u64> = Vec::new();
    for pos in positions {
        let loop_len = find_loop(&positions_map, directions.clone(), pos);
        loop_lengths.push(loop_len);
    }

    let lcm = lcm_of_set(&loop_lengths);
    println!("Loops sync after: {lcm}");

    Ok(())
}
