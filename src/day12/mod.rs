use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

fn get_permutations(seq: &Vec<u32>, row: &[Vec<char>]) -> u64 {
    let mut sum: u64 = 0;

    let req: u32 = seq.iter().fold(0, |acc, &x| acc + x);
    let avail: u32 = row.iter().fold(0, |acc, group| group.len() as u32 + acc);

    if req > avail {
        return 0;
    }

    println!("possible - {:?} {:?}", row, seq);

    if row.len() > 1 {
        sum += get_permutations(seq, &row[1..])
    }
    return sum;
}

fn get_groups(row: &str) -> Option<Vec<Vec<char>>> {
    Some(
        row.split(".")
            .filter(|s| s != &"")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
}

fn get_sequence(seq: &str) -> Option<Vec<u32>> {
    Some(seq.split(",").map(|s| s.parse::<u32>().unwrap()).collect())
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day12/example.txt")?;

    let mut sum: u64 = 0;
    for line in reader.lines() {
        let line = line?;

        let mut iter = line.split_whitespace();
        let (r, s) = (
            iter.next().and_then(get_groups),
            iter.next().and_then(get_sequence),
        );

        match (r, s) {
            (Some(row), Some(seq)) => {
                sum += get_permutations(&seq, &row);
            }
            (_, _) => {}
        }
    }

    Ok(())
}
