use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead};

type KnownPaths = HashMap<String, u64>;
type VisitedGroup = HashMap<String, KnownPaths>;

fn perm_count(visited: &mut VisitedGroup, seq: &[u32], group: &[char]) -> u64 {
    if seq.len() == 0 {
        return 1;
    }

    if group.len() == 0 {
        return 0;
    }

    // if the group doesn't have the minimal number of spaces left to satisfy
    // the sequence
    let min_space = (seq.iter().fold(0, |acc, s| acc + s + 1) - 1) as usize;

    let seq_key: String = seq.iter().map(|&n| n.to_string()).collect();
    let key: String = group.iter().collect();
    if let Some(knowns) = visited.get(&key) {
        if let Some(k) = knowns.get(&seq_key) {
            // println!("already known:\n\t{key}\n\t{seq_key}\n\tcount: {k}");
            return *k;
        }
    }

    let mut ct: u64 = 0;
    let mut group = &group[..];
    let s = seq[0] as usize;
    while group.len() >= min_space {
        if group.len() == s || group[s] == '?' || group[s] == '.' {
            // can't satisfy seq if any dots
            if !group[..s].contains(&'.') {
                // print!(" - NO DOTS {:?}", group[..s].to_vec());
                // if there is more of the sequence to try to fit in the group
                if seq.len() > 1 {
                    // print!("\n");
                    // if there are more characters
                    if group.len() > s {
                        ct += perm_count(visited, &seq[1..], &group[s + 1..]);
                    } else {
                        // print!(" x (not enough chars left)");
                    }
                } else if group[s..].contains(&'#') {
                    // reached the end of the sequence, but there are hashes left
                    // print!(" x (has remaining #)");
                } else {
                    // fit the whole sequence! +1
                    // print!(" √");
                    ct += 1;
                }
            } else {
                // print!(" x (has . in space)");
            }
        } else {
            // print!(" x (has # joining preventing)");
        }
        // print!("\n");

        // This has to be part of the group, so there are no more permuations
        // of the current sequence beyond this point
        if group[0] != '#' {
            group = &group[1..];
        } else {
            break;
        }
    }

    // println!("Inserting:\n\t{key}\n\t{seq_key}\n\tCount: {ct}");
    visited
        .entry(key)
        .or_insert_with(|| HashMap::new())
        .entry(seq_key)
        .or_insert(ct);
    return ct;
}

fn get_groups(row: &str, num_copies: usize) -> Option<Vec<char>> {
    let orig = row.chars().collect::<Vec<char>>();

    let copies: Vec<Vec<char>> = (0..num_copies).map(|_| orig.to_vec()).collect();

    // println!("COPIES {:?}", copies);
    // println!("JOINED {:?}", copies.join(&'?'));

    Some(copies.join(&'?'))
}

fn get_sequence(seq: &str, num_copies: usize) -> Option<Vec<u32>> {
    let seq: Vec<u32> = seq.split(",").map(|s| s.parse::<u32>().unwrap()).collect();

    let result: Vec<Vec<u32>> = (0..num_copies).map(|_| seq.to_vec()).collect();

    Some(result.concat())
}

pub fn run() -> io::Result<()> {
    let copies: usize = 1;
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day12/input.txt")?;

    let mut sum: u64 = 0;
    for line in reader.lines() {
        let line = line?;

        let mut iter = line.split_whitespace();
        let (r, s) = (
            iter.next().and_then(|row| get_groups(row, copies)),
            iter.next().and_then(|seq| get_sequence(seq, copies)),
        );

        let mut visited: &mut VisitedGroup = &mut HashMap::new();

        match (r, s) {
            (Some(row), Some(seq)) => {
                println!("           {:?} {:?}", row, seq);
                let count = perm_count(&mut visited, &seq, &row);
                sum += count;
                println!("Count: {count}\n---------------------------------\n");
            }
            (_, _) => {}
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
