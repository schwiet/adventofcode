use super::util::open_file_as_bufreader;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

const EXPANDER: u64 = 999_999;

fn is_printable(c: char) -> bool {
    !c.is_control() && !c.is_whitespace()
}

fn next_char(start_char: char) -> char {
    let mut current_char = start_char;

    while !is_printable(current_char) || current_char == start_char {
        // Increment to the next character
        current_char = char::from_u32(current_char as u32 + 1).unwrap_or('a');
    }

    current_char
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day11/input.txt")?;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    const GAL_CHAR: char = '#';
    let mut counter_char = 'a'; // Starting counter character
    let mut col_gal_ct: Vec<u32> = Vec::new();

    let mut row_adders: Vec<u64> = Vec::new();
    let mut col_adders: Vec<u64> = Vec::new();

    let mut grown_amt: u64 = 0;
    for line in reader.lines() {
        let line = line?;

        if line.contains('#') {
            let mut line_chars: Vec<char> = line.chars().collect();
            for i in 0..line_chars.len() {
                if line_chars[i] == GAL_CHAR {
                    // make sur the column count vector is the right size
                    while col_gal_ct.len() < line.len() {
                        col_gal_ct.push(0);
                    }
                    col_gal_ct[i] += 1;

                    // Replace the matching character with the counter character
                    line_chars[i] = counter_char;
                    counter_char = next_char(counter_char)
                }
            }

            matrix.push(line_chars);
        } else {
            matrix.push(line.chars().collect());
            grown_amt += 1;
        }
        row_adders.push(grown_amt as u64);
    }

    let mut grown_amt: usize = 0;
    for i in 0..col_gal_ct.len() {
        if col_gal_ct[i] == 0 {
            grown_amt += 1;
        }
        col_adders.push(grown_amt as u64);
        print!("{} -", i + grown_amt * EXPANDER as usize);
    }
    println!("");

    let mut positions: HashMap<char, (u64, u64)> = HashMap::new();
    for y in 0..matrix.len() {
        let row = &matrix[y];
        let expanded_row = y as u64 + (row_adders[y] * EXPANDER);
        print!("row | ");
        for x in 0..row.len() {
            let cell = row[x];
            let expanded_col = x as u64 + (col_adders[x] * EXPANDER);
            print!("{cell}");

            if cell != '.' {
                positions.insert(cell, (expanded_col, expanded_row));
            }
        }
        print!(" ({expanded_row})");
        print!("\n");
    }

    for (p, (i, j)) in &positions {
        println!("{} - ({},{})", p, i, j);
    }

    let mut node_done: HashSet<char> = HashSet::new();
    let mut path_sum: u64 = 0;
    for (galaxy, pos) in &positions {
        for (other_galaxy, other_pos) in &positions {
            if other_galaxy != galaxy && !node_done.contains(other_galaxy) {
                let x_dist = if pos.0 > other_pos.0 {
                    pos.0 - other_pos.0
                } else {
                    other_pos.0 - pos.0
                };
                let y_dist = if pos.1 > other_pos.1 {
                    pos.1 - other_pos.1
                } else {
                    other_pos.1 - pos.1
                };

                println!("({}, {}) - ({}h,{}v)", galaxy, other_galaxy, x_dist, y_dist);
                path_sum += x_dist + y_dist;
            }
        }

        node_done.insert(*galaxy);
    }

    println!("Sum of paths: {path_sum}");
    Ok(())
}
