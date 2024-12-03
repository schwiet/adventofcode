use super::util::open_file_as_bufreader;

use std::io::{self, BufRead};
use regex::Regex;

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day25/example.txt")?;

    let re = Regex::new(r"\b[a-zA-Z]{3}\b").unwrap(); // matches exactly 3 letters
    for line in reader.lines() {
        let line = line?;

        let _nodes: Vec<String> = re.find_iter(&line)
            .map(|mat| mat.as_str().to_string())
            .collect();

    }

    Ok(())
}