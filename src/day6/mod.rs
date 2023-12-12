use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let mut reader = open_file_as_bufreader("src/day6/input-2.txt")?;

    let mut times: String = String::new();
    let mut distances: String = String::new();
    reader.read_line(&mut times)?;
    reader.read_line(&mut distances)?;

    let mut times = times.split_whitespace();
    // skip key
    let times: Vec<u64> = times
        .by_ref()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut distances = distances.split_whitespace();
    // skip key
    let distances: Vec<u64> = distances
        .by_ref()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut product: u64 = 1;
    for i in 0..times.len() {
        let mut sum: u64 = 0;
        let t1: u64 = times[i];
        let d1: u64 = distances[i];

        for t in (1..t1 + 1) {
            if t * (t1 - t) > d1 {
                sum += 1;
            }
        }
        product *= sum;
    }

    println!("Product: {product}");
    Ok(())
}
