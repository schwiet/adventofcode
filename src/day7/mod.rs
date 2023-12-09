use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead};

const CARDS: [(char, u8); 13] = [
    ('A', 12),
    ('K', 11),
    ('Q', 10),
    ('J', 9),
    ('T', 8),
    ('9', 7),
    ('8', 6),
    ('7', 5),
    ('6', 4),
    ('5', 3),
    ('4', 2),
    ('3', 1),
    ('2', 0),
];

#[derive(Debug, Eq, PartialEq)]
enum PokerHand {
    Undefined,
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

#[derive(Debug)]
struct Hand {
    hand_type: PokerHand,
    cards: [u8; 5],
    str: String,
}

fn classify_hand(hand: &str, card_values: &HashMap<char,u8>) -> PokerHand {
    let mut card_counts: HashMap<char, u8> = HashMap::new();
    let mut i = 0;
    let mut cards: [u8; 5] = [0; 5];
    for char in hand.chars() {
        if let Some(val) = card_values.get(char) {
            cards[i] = *val;
        }
        let count = card_counts.entry(char).or_insert(0);

        *count += 1;
        i += 1;
    }
    let mut count_freq = [0;5];

    // Iterate over the values
    for count in card_counts.values() {
        count_freq[(count - 1) as usize] += 1
    }

    match count_freq {
        [5, 0, 0, 0, 0] => PokerHand::HighCard,
        [1, 2, 0, 0, 0] => PokerHand::TwoPair,
        [2, 0, 1, 0, 0] => PokerHand::ThreeOfAKind,
        [0, 1, 1, 0, 0] => PokerHand::FullHouse,
        [1, 0, 0, 1, 0] => PokerHand::FourOfAKind,
        [0, 0, 0, 0, 1] => PokerHand::FiveOfAKind,
        _ => PokerHand::Undefined,
    }
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

        let poker_hand = classify_hand(tokens[0], &map);
        println!("{:?}, {}, {}", poker_hand, tokens[0], tokens[1]);
    }

    Ok(())
}
