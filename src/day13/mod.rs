use super::util::open_file_as_bufreader;
use std::io::{self, BufRead, ErrorKind};

use nalgebra::DMatrix;

fn binary_to_u64(binary_data: &[u8]) -> u64 {
    let mut result = 0u64;

    for &byte in binary_data {
        // Shift the existing bits in result left by 1 and add the current bit (byte)
        result = (result << 1) | u64::from(byte);
    }
    result
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day13/example.txt")?;

    let mut ref_rows: u64 = 0;
    let mut ref_cols: u64 = 0;

    let mut rows: Vec<u64> = Vec::new();
    let mut cols: Vec<u64> = Vec::new();
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    let mut col_count = 0usize;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            // Construct matrix
            let row_count = matrix.len();
            let tmat =
                DMatrix::from_iterator(row_count, col_count, matrix.clone().into_iter().flatten());
            let mat = tmat.transpose();

            println!("Matrix {mat}");
            println!("Transpose {tmat}");

            // find reflection

            // reset rows and cols
        } else {
            // Replace '#' with 1 and '.' with 0
            println!("Src: {line}");
            let row_bin = line
                .chars()
                .map(|c| match c {
                    '.' => 0u8,
                    '#' => 1u8,
                    _ => panic!("Invalid character"),
                })
                .collect::<Vec<u8>>();

            col_count = row_bin.len();

            // convert binary representation to a u64
            let row: u64 = binary_to_u64(&row_bin);

            // keep the u64 for the row
            rows.push(row);

            // keep the binary row so we can calculate the column representation, later
            matrix.push(row_bin);
        }
    }

    println!("Result: {:?}", ref_cols * 100 + ref_rows);

    Ok(())
}
