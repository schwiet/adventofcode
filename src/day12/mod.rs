use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

// TODO: I'm not quite there with this one. Vaguely, I know you need to try
//       permutations of sequences on permuations of groups, but I haven't
//       fleshed out an algorithm
fn get_permutations(seq: &[u32], row: &[Vec<char>]) -> u64 {
    let mut sum: u64 = 0;

    let mut start_seq = &seq[0..];
    while start_seq.len() > 0 {
        let seq_total = 0;
        for r in row.iter() {
            // get the number of permuations of this sequence that can be made
            // in just this row
            let mut num = perm_count(start_seq, r);

            // if any, keep track and try the remainder of the sequence on the
            // next row
            if num != 0 {
                if seq_total == 0 {
                    seq_total = num
                } else {
                    seq_total *= num
                }
                // rem_seq = $
                num = perm_count()
            }
        }
        s = &start_seq[0..s.len() - 1];
    }

    return sum;
}

fn perm_count(seq: &[u32], row: &Vec<char>) -> u64 {
    // TODO
    0
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
