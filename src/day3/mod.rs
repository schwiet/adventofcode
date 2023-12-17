use super::util::open_file_as_bufreader;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use regex::Regex;

struct Number {
    as_str: String,
    row: usize,
    col_start: usize,
    col_end: usize,
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day3/example.txt")?;

    let re = Regex::new(r"(\d+)").unwrap();
    let char_re = Regex::new(r"[^.^\d]").unwrap();

    let mut candidates: Vec<Number> = Vec::new();
    let mut chars: HashSet<(usize, usize)> = HashSet::new();
    let mut gears: Vec<(usize, usize)> = Vec::new();
    let mut numbers: HashMap<(usize, usize), usize> = HashMap::new();

    let mut row = 0usize;
    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error

        for mat in re.find_iter(&line) {
            candidates.push(Number {
                as_str: mat.as_str().to_string(),
                row,
                col_start: mat.start(),
                col_end: mat.end(),
            });
            let n = mat.as_str().parse::<usize>().unwrap();
            for i in (mat.start()..mat.end() + 1) {
                numbers.insert((row, i), n);
            }
        }

        for mat in char_re.find_iter(&line) {
            println!("char \"{}\" at {}, {}", mat.as_str(), row, mat.start());
            chars.insert((row, mat.start()));
            if mat.as_str() == "*" {
                gears.push((row, mat.start()));
            }
        }

        row += 1;
    }

    let mut sum = 0usize;
    for num in candidates {
        // println!("{} - ({}, {})", num.as_str, num.row, num.col_start);
        let start_row = if num.row > 0 { num.row - 1 } else { 0 };
        let start_col = if num.col_start > 0 {
            num.col_start - 1
        } else {
            0
        };
        'row_loop: for r in (start_row..start_row + 3) {
            for c in (start_col..start_col + num.as_str.len() + 2) {
                if chars.contains(&(r as usize, c)) {
                    println!("\tfound symbol for {} at: ({r} {c})", num.as_str);
                    sum += num.as_str.parse::<usize>().unwrap();
                    break 'row_loop;
                }
            }
        }
    }

    println!("Sum {sum}");

    let mut sum = 0usize;
    for gear in gears {
        let mut adj_parts = 0usize;
        let mut product = 1usize;
        println!("gear {} {}", gear.0, gear.1);
        for r in (0..3) {
            for c in (0..3) {
                if let Some(n) = numbers.get(&(r + gear.0, c + gear.1)) {
                    adj_parts += 1;
                    product *= n;
                }
            }
        }
        if adj_parts == 2 {
            sum += product;
        }
    }

    println!("Gears {sum}");
    Ok(())
}
