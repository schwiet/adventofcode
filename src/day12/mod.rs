use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

fn perm_count(seq: &[u32], group: &[char]) -> u64 {
    if seq.len() == 0 {
        return 1;
    }

    if group.len() == 0 {
        return 0;
    }

    // if the group doesn't have the minimal number of spaces left to satisfy
    // the sequence
    let min_space = (seq.iter().fold(0, |acc, s| acc + s + 1) - 1) as usize;

    let mut ct: u64 = 0;
    let mut group = &group[..];
    let s = seq[0] as usize;
    while group.len() >= min_space {
        print!("- Checking {:?} {:?}", group, seq);
        if group.len() == s || group[s] == '?' || group[s] == '.' {
            // can't satisfy seq if any dots
            if !group[..s].contains(&'.') {
                // print!(" - NO DOTS {:?}", group[..s].to_vec());
                // if there is more of the sequence to try to fit in the group
                if seq.len() > 1 {
                    print!("\n");
                    // if there are more characters
                    if group.len() > s {
                        println!("  - took {:?}", group[..s + 1].to_vec());
                        ct += perm_count(&seq[1..], &group[s + 1..])
                    } else {
                        print!(" x (not enough chars left)");
                    }
                } else if group[s..].contains(&'#') {
                    // reached the end of the sequence, but there are hashes left
                    print!(" x (has remaining #)");
                } else {
                    // fit the whole sequence! +1
                    print!(" √");
                    ct += 1;
                }
            } else {
                print!(" x (has . in space)");
            }
        } else {
            print!(" x (has # joining preventing)");
        }
        print!("\n");

        // This has to be part of the group, so there are no more permuations
        // of the current sequence beyond this point
        if group[0] != '#' {
            group = &group[1..];
        } else {
            break;
        }
    }
    return ct;
}

fn get_groups(row: &str) -> Option<Vec<char>> {
    Some(
        row.split(".")
            .filter(|s| s != &"")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>()
            .join(&'.'),
    )
}

fn get_sequence(seq: &str) -> Option<Vec<u32>> {
    Some(seq.split(",").map(|s| s.parse::<u32>().unwrap()).collect())
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day12/input.txt")?;

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
                println!("           {:?} {:?}", row, seq);
                let count = perm_count(&seq, &row);
                sum += count;
                println!("Count: {count}\n---------------------------------\n");
            }
            (_, _) => {}
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
