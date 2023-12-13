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
    let ends_A = Regex::new(r".*A$").unwrap();
    let ends_Z = Regex::new(r".*Z$").unwrap();

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

            if let Some(start_pos) = ends_A.captures(&position) {
                positions.push(start_pos.get(0).unwrap().as_str().to_string());
            }
        } else {
            println!("{line} didn't match regex")
        }

        // println!("line - {line}");
    }

    let mut z_count: usize = 0;
    let mut found: bool = false;
    let mut steps: u64 = 0;

    let mut uk_paths: Vec<Vec<String>> = vec![Vec::new(); positions.len()];
    let mut kn_dists: HashMap<String, usize> = HashMap::new();
    let mut remaining: Vec<u64> = vec![0; positions.len()];
    println!("num {}", positions.len());
    while !found {
        for dir in directions.chars() {

            if remaining[0] != 0 && remaining.iter().all(|&x| x == remaining[0]) {
                found = true;
                println!("steps = {}", steps + remaining[0]);
                return Ok(());
            } else {
                // print!("visited {pos}");
                steps += 1;
            }

            if z_count > 0 {
                println!("{}/{} Zs - {:?}", z_count, positions.len(), positions);
            }

            // println!("{:?}", kn_dists);
            // reset the count of end postitions that match our goal
            z_count = 0;
            let mut new_pos: &str;
            for i in 0..positions.len() {
                let pos: &str = &positions[i];
                // Accessing values by position
                if let Some(dirs) = positions_map.get(pos) {
                    // println!(", next {dir}");
                    match dir {
                        'L' => {
                            new_pos = &dirs.left;
                        }
                        'R' => {
                            new_pos = &dirs.right;
                        }
                        _ => {
                            println!("Direction not found - {pos}");
                            return Ok(());
                        }
                    }
                    positions[i] = new_pos.to_string();
                    let mut cur_path_len: usize = 0;

                    // Clone the value obtained from get to avoid borrowing conflicts
                    let dist = kn_dists.get(new_pos).cloned().unwrap_or(0);

                    if dist > 0 || ends_Z.is_match(new_pos) {
                        for j in 0..uk_paths[i].len() {
                            cur_path_len += 1;
                            kn_dists.insert(uk_paths[i][j].clone(), cur_path_len + dist);
                        }
                        uk_paths[i].clear();
                        remaining[i] = steps + dist as u64
                    } else {
                        uk_paths[i].push(new_pos.to_string());
                        remaining[i] = 0;
                    }
                } else {
                    println!("Position not found - {pos}");
                    return Ok(());
                }
            }
        }

        // print!("\r");
        // print!("{steps} steps so far...");
    }

    Ok(())
}
