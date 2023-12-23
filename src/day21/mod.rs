use super::util::open_file_as_bufreader;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn reachable_gardens(
    max_step: u64,
    step: u64,
    area: &HashSet<(u64, u64)>,
    garden: &(u64, u64),
) -> u64 {
    if area.get(garden) == None || step > max_step {
        return 0;
    }

    let mut sum = 0u64;

    // LEFT
    if garden.1 > 0 {
        let l = (garden.0, garden.1 - 1);
        if let Some(g) = area.get(&l) {
            println!("Garden at {:?}", l);
            if step >= max_step {
                sum += 1;
            } else {
                sum += reachable_gardens(max_step, step + 1, area, &l);
            }
        }
    }

    // UP
    if garden.0 > 0 {
        let u = (garden.0 - 1, garden.1);
        if let Some(g) = area.get(&u) {
            println!("Garden at {:?}", u);
            if step >= max_step {
                sum += 1;
            } else {
                sum += reachable_gardens(max_step, step + 1, area, &u);
            }
        }
    }

    // RIGHT
    let r = (garden.0, garden.1 + 1);
    if let Some(g) = area.get(&r) {
        println!("Garden at {:?}", r);
        if step >= max_step {
            sum += 1;
        } else {
            sum += reachable_gardens(max_step, step + 1, area, &r);
        }
    }

    // DOWN
    let d = (garden.0 + 1, garden.1);
    if let Some(g) = area.get(&d) {
        println!("Garden at {:?}", d);
        if step >= max_step {
            sum += 1;
        } else {
            sum += reachable_gardens(max_step, step + 1, area, &d);
        }
    }

    return sum;
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day21/example.txt")?;

    let mut i = 0u64;
    let mut s = (0u64, 0u64);

    let mut gardens: HashSet<(u64, u64)> = HashSet::new();

    for line in reader.lines() {
        let line = line?;

        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    gardens.insert((i, j as u64));
                }
                'S' => {
                    gardens.insert((i, j as u64));
                    s = (i, j as u64);
                }
                _ => { /* ignore others */ }
            }
        }

        i += 1;
    }

    let sum = reachable_gardens(1, 0, &gardens, &s);

    println!("Reachable Gardens: {sum}, {:?}", s);

    Ok(())
}
