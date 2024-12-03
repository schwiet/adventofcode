use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead, ErrorKind};

type LineChecker = Box<dyn Fn(&str, u32) -> Result<(), std::io::Error>>;
type KeepFn = fn(newVal: u32, oldVal: u32) -> bool;
type ResultFn = fn(map: &HashMap<String, u32>) -> Result<u32, std::io::Error>;

fn process_line(
    line: &str,
    checker: &LineChecker,
    keep_fn: KeepFn,
    result_fn: ResultFn,
) -> Result<u32, std::io::Error> {
    let mut map = HashMap::new();
    let parts = line.split([':', ',', ';'].as_ref()).map(str::trim);

    for (index, part) in parts.enumerate() {
        let mut iter = part.split_whitespace();
        if let (Some(t1), Some(t2)) = (iter.next(), iter.next()) {
            // the first key is _before_ the value, while the others have a key
            // proceeding the value
            let (key, value_str) = if index > 0 { (t2, t1) } else { (t1, t2) };

            if let Ok(value) = value_str.parse::<u32>() {
                // validate the value
                checker(key, value)?;
                // get the entry and replace it, if necessary
                let entry = map.entry(key.to_string()).or_insert(value);
                if keep_fn(value, *entry) {
                    *entry = value
                }
            }
        }
    }

    // println!("{:?}", map);
    return result_fn(&map);
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

fn always_true() -> LineChecker {
    Box::new(move |_key, _value| return Ok(()))
}

fn is_greater(val1: u32, val2: u32) -> bool {
    return val1 > val2;
}

fn is_less(val1: u32, val2: u32) -> bool {
    return val1 < val2;
}

fn game_num(map: &HashMap<String, u32>) -> Result<u32, std::io::Error> {
    return match map.get("Game") {
        Some(&val) => Ok(val),
        None => Err(io::Error::new(ErrorKind::Other, "Game not found")),
    };
}

fn game_power(map: &HashMap<String, u32>) -> Result<u32, std::io::Error> {
    if let (Some(&red), Some(&green), Some(&blue)) =
        (map.get("red"), map.get("green"), map.get("blue"))
    {
        return Ok(red * green * blue);
    }

    Err(io::Error::new(ErrorKind::Other, "Missing colors"))
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
    let mut sum_of_pwr = 0;

    for line in reader.lines() {
        let line = line?; // propagate error if encountered reading a line

        if let Ok(game_num) = process_line(&line, &lim_checker, is_greater, game_num) {
            sum += game_num;
            // println!("Game {game_num} is possible");
        }

        if let Ok(game_pow) = process_line(&line, &always_true(), is_greater, game_power) {
            sum_of_pwr += game_pow;
        }
    }

    println!("Sum: {sum}");
    println!("Sum of Power: {sum_of_pwr}");
    Ok(())
}
