use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead};

const CARDS: [(&str, i32); 13] = [
    ("A", 14),
    ("K", 13),
    ("Q", 12),
    ("J", 11),
    ("T", 10),
    ("9", 9),
    ("8", 8),
    ("7", 7),
    ("6", 6),
    ("5", 5),
    ("4", 4),
    ("3", 3),
    ("2", 2),
];

#[derive(Debug, Eq, PartialEq)]
enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

pub fn run() -> io::Result<()> {
    // Convert the array of tuples into a HashMap
    let map: HashMap<_, _> = CARDS.iter().cloned().collect();

    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day7/input.txt")?;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error

        // Splitting the line on whitespace
        let tokens: Vec<&str> = line.split_whitespace().collect();

        println!("{}, {}", tokens[0], tokens[1]);
    }

    Ok(())
}
