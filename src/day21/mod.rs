use super::util::open_file_as_bufreader;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn run_game(gardens: &HashSet<(u64, u64)>, gen: &HashSet<(u64, u64)>) -> HashSet<(u64, u64)> {
    let mut next_gen: HashSet<(u64, u64)> = HashSet::new();

    for g in gen {
        // check possible gardens
        // up
        if g.0 > 0 && gardens.get(&(g.0 - 1, g.1)) != None {
            next_gen.insert((g.0 - 1, g.1));
        }

        // left
        if g.1 > 0 && gardens.get(&(g.0, g.1 - 1)) != None {
            next_gen.insert((g.0, g.1 - 1));
        }

        // right
        if gardens.get(&(g.0, g.1 + 1)) != None {
            next_gen.insert((g.0, g.1 + 1));
        }

        // down
        if gardens.get(&(g.0 + 1, g.1)) != None {
            next_gen.insert((g.0 + 1, g.1));
        }

        // check current state
        let mut has_neighbors: bool = false;
        // up
        if g.0 > 0 && gen.get(&(g.0 - 1, g.1)) != None {
            has_neighbors = true;
        }

        // left
        if g.1 > 0 && gen.get(&(g.0, g.1 - 1)) != None {
            has_neighbors = true;
        }

        // right
        if gen.get(&(g.0, g.1 + 1)) != None {
            has_neighbors = true;
        }

        // down
        if gen.get(&(g.0 + 1, g.1)) != None {
            has_neighbors = true;
        }

        if has_neighbors {
            next_gen.insert(*g);
        }
    }

    return next_gen;
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day21/input.txt")?;

    let mut i = 0u64;
    let mut s = (0u64, 0u64);

    let mut gardens: HashSet<(u64, u64)> = HashSet::new();
    let mut gen: HashSet<(u64, u64)> = HashSet::new();

    for line in reader.lines() {
        let line = line?;

        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    gardens.insert((i, j as u64));
                }
                'S' => {
                    gardens.insert((i, j as u64));
                    gen.insert((i, j as u64));
                    s = (i, j as u64);
                }
                _ => { /* ignore others */ }
            }
        }

        i += 1;
    }

    for i in 0..64 {
        gen = run_game(&gardens, &gen)
    }

    println!("Starting Garden: {:?}", s);
    println!("Visited Gardens: {}", gen.len());
    // 715 is too low
    // 1663 is also too low

    Ok(())
}
