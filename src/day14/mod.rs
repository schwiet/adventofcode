use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

type Matrix = Vec<Vec<char>>;

fn transpose(matrix: Matrix) -> Matrix {
    let max_row_len = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut transposed = vec![vec![' '; max_row_len]; matrix.len()];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            transposed[j][i] = item;
        }
    }

    // Trimming empty rows if any exist
    transposed
        .into_iter()
        .filter(|row| !row.iter().all(|&c| c == ' '))
        .collect()
}

fn calc_load(matrix: &Matrix) -> usize {
    let mut load = matrix.iter().fold(0usize, |acc, row| {
        let mut pt = row.len();
        let row_load = row.iter().enumerate().fold(0, |acc, cell| {
            if cell.1 == &'O' {
                let update = acc + pt;
                // println!("adding {update}");
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
    return load;
}

fn tilt(mut matrix: &mut Matrix) {
    // for each row, split on the cube rocks "#", sort each resulting vector or
    // just count the "O"s and replace the section, then join them all back with
    // the "#"
    for i in 0..matrix.len() {}
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day14/input.txt")?;
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut cube: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        rows.push(line.chars().collect());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut matrix = transpose(rows);
    for i in 0..100 {
        tilt(&mut matrix);
        matrix = transpose(matrix);
        if i % 1000 == 0 {
            println!("{i}");
        }
    }

    let mut load = calc_load(&matrix);
    // println!("{matrix}");
    println!("Load: {load}");
    Ok(())
}
