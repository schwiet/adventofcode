use super::util::open_file_as_bufreader;
use std::io::{self, BufRead, ErrorKind};
use std::ops::Range;

use nalgebra::DMatrix;

fn binary_to_u64(binary_data: &[u8]) -> u64 {
    let mut result = 0u64;

    for &byte in binary_data {
        // Shift the existing bits in result left by 1 and add the current bit (byte)
        result = (result << 1) | u64::from(byte);
    }
    result
}

fn are_one_bit_off(x: u64, y: u64) -> Option<u32> {
    // xor the two numbers and determine if there is only one bit difference
    let n = x ^ y;

    if n != 0 && (n & (n - 1)) == 0 {
        return Some(n.trailing_zeros());
    }

    return None;
}

fn find_reflection_if_smudged(vec: &[u64], bit_len: usize) -> Option<usize> {
    // twiddle bits to remove smudges
    let mut i = 0;
    // idk, maybe a blank line slips in
    if vec.len() == 0 {
        return None;
    }

    while i < vec.len() - 1 {
        let mut j = i + 1;
        while j < vec.len() {
            // println!("checking {i} {j}, len: {}", vec.len());
            // println!("\t{:0width$b}", vec[i], width = bit_len);
            // println!("\t{:0width$b}", vec[j], width = bit_len);
            if let Some(k) = are_one_bit_off(vec[i], vec[j]) {
                // TODO: check if this smudge is along a reflection
                if (j - i) % 2 == 1 {
                    let mut v_copy: Vec<u64> = vec.iter().cloned().collect();
                    v_copy[i] = vec[j];
                    let mid_line = i + ((j - i) / 2);
                    // println!("Found Smudge at between: ({i}, {j}) at: {k} with mid line:{mid_line}");
                    // println!("{:0width$b}", vec[i], width = bit_len);
                    // println!("{:0width$b}", vec[j], width = bit_len);
                    if let Some(ref_line) = find_reflection(&v_copy, mid_line) {
                        // println!("REF AFTER SMUDGE {ref_line}");
                        if ref_line - 1 == mid_line {
                            // println!("Found Reflection at: {}", ref_line - 1);
                            return Some(ref_line);
                        }
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    None
}

fn find_reflection(vec: &[u64], start: usize) -> Option<usize> {
    for i in (start..vec.len() - 1) {
        if vec[i] == vec[i + 1] {
            let mut found_diff = false;
            for (l, u) in (0..i + 1).rev().zip((i + 1..vec.len())) {
                // println!("comparing \n\t{}\n\t{}", vec[l], vec[u]);
                if vec[l] != vec[u] {
                    found_diff = true;
                    break;
                }
            }

            if !found_diff {
                // println!("Found Reflection at: {i}");
                return Some(i + 1);
            }
        }
    }
    None
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day13/input.txt")?;

    let mut ref_rows: u64 = 0;
    let mut ref_cols: u64 = 0;

    let mut rows: Vec<u64> = Vec::new();
    let mut cols: Vec<u64> = Vec::new();
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    let mut col_count = 0usize;

    let mut sum = 0u64;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            // Construct matrix
            let row_count = matrix.len();
            let mat =
                DMatrix::from_iterator(col_count, row_count, matrix.clone().into_iter().flatten());

            // println!("Transpose {mat}");

            cols = mat
                .row_iter()
                .map(|row| binary_to_u64(&row.iter().cloned().collect::<Vec<u8>>()))
                .collect();

            // find reflection
            let mut row_index: Option<usize> = None;
            let mut col_index: Option<usize> = None;

            // // row_index = find_reflection(&rows);
            row_index = find_reflection_if_smudged(&rows, col_count);
            if let Some(ref_row) = row_index {
                ref_rows += (ref_row as u64);
                // println!("found row reflection {ref_row}");
                for i in (0..rows.len()) {
                    // println!("Row: {} - {i}", rows[i]);
                }
            }

            // col_index = find_reflection(&cols);
            col_index = find_reflection_if_smudged(&cols, row_count);
            if let Some(ref_col) = col_index {
                ref_cols += (ref_col as u64);
                // println!("found column reflection {ref_col}");
                // for i in (0..cols.len()) {
                // println!("Col: {} - {i}", cols[i]);
                // }
            }

            match (row_index, col_index) {
                (None, None) => {
                    println!("found no reflection");
                    println!("{}", mat.transpose());
                }
                (_, _) => {}
            }

            // reset rows and cols
            rows.clear();
            cols.clear();
            matrix.clear();
            col_count = 0;
        } else {
            // Replace '#' with 1 and '.' with 0
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

            // keep the string
            // rows.push(line.trim().to_string());

            // keep the binary row so we can calculate the column representation, later
            matrix.push(row_bin);
        }
    }

    println!(
        "Result: {:?}, Rows {ref_rows} Cols {ref_cols}",
        ref_cols + (ref_rows * 100)
    );

    Ok(())
}
