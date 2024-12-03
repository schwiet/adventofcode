use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day9/input.txt")?;

    let mut sum: i64 = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error

        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|word| word.parse::<i64>())
            .filter_map(Result::ok)
            // .rev()
            .collect();

        sum += predict_next(&numbers, &Direction::Forward);
    }

    println!("sum: {sum}");
    Ok(())
}

enum Direction {
    Forward,
    Backward,
}

fn predict_next(seq: &Vec<i64>, dir: &Direction) -> i64 {
    let mut something_not_0: bool = false;
    let mut diffs: Vec<i64> = Vec::with_capacity(seq.len());

    for i in 0..(seq.len() - 1) {
        let diff = match dir {
            Direction::Forward => seq[i + 1] - seq[i],
            Direction::Backward => seq[i] - seq[i + 1],
        };
        diffs.push(diff);

        if diff != 0 {
            something_not_0 = true;
        }
    }

    if !something_not_0 {
        return seq[seq.len() - 1];
    }

    return match dir {
        Direction::Forward => seq[seq.len() - 1] + predict_next(&diffs, dir),
        Direction::Backward => seq[seq.len() - 1] - predict_next(&diffs, dir),
    };
}
