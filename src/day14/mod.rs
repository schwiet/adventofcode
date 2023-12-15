use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

use nalgebra::DMatrix;

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day14/input.txt")?;
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        rows.push(line.chars().collect());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();
    let mut matrix = DMatrix::from_iterator(num_cols, num_rows, rows.clone().into_iter().flatten());

    let mut load = matrix.row_iter().fold(0usize, |acc, row| {
        let mut pt = row.len();
        let row_load = row.iter().enumerate().fold(0, |acc, cell| {
            if cell.1 == &'O' {
                let update = acc + pt;
                println!("adding {update}");
                pt -= 1;
                return update;
            } else if cell.1 == &'#' {
                pt = row.len() - (cell.0 + 1);
                return acc;
            } else {
                return acc;
            }
        });
        acc + row_load
    });
    // println!("{matrix}");
    println!("Load: {load}");
    Ok(())
}
