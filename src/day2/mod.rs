use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead, ErrorKind};

type LineChecker = Box<dyn Fn(&str, u32) -> Result<(), std::io::Error>>;

fn process_line(line: &str, checker: &LineChecker) -> Result<u32, std::io::Error> {
    let mut map = HashMap::new();
    let parts = line.split([':', ',', ';'].as_ref()).map(str::trim);

    for (index, part) in parts.enumerate() {
        let mut iter = part.split_whitespace();
        if let (Some(t1), Some(t2)) = (iter.next(), iter.next()) {
            // the first key is _before_ the value, while the others have a key
            // proceeding the value
            let (key, value_str) = if index > 0 { (t2, t1) } else { (t1, t2) };

            if let Ok(value) = value_str.parse::<u32>() {
                let entry = map.entry(key.to_string()).or_insert(0);
                if value > *entry {
                    // if this value does not satisfy the checker, propagate the
                    // error
                    checker(key, value)?;
                    *entry = value;
                }
            }
        }
    }

    return match map.get("Game") {
        Some(&val) => Ok(val),
        None => Err(io::Error::new(ErrorKind::Other, "Game not found")),
    };
}

fn create_limit_checker(limits: HashMap<String, u32>) -> LineChecker {
    Box::new(move |key, value| {
        if let Some(&lim) = limits.get(key) {
            if value > lim {
                return Err(io::Error::new(ErrorKind::Other, "Exceeded Limit"));
            }
        }
        Ok(())
    })
}

pub fn run() -> io::Result<()> {
    let reader = open_file_as_bufreader("src/day2/input.txt")?;

    // construct the limits
    let mut limits = HashMap::new();
    limits.insert("red".to_string(), 12);
    limits.insert("green".to_string(), 13);
    limits.insert("blue".to_string(), 14);
    let lim_checker = create_limit_checker(limits);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?; // propagate error if encountered reading a line

        if let Ok(gameNum) = process_line(&line, &lim_checker) {
            sum += gameNum;
            println!("Game {gameNum} is possible");
        }
    }

    println!("Sum: {sum}");
    Ok(())
}
