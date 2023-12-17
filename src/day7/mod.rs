use super::util::open_file_as_bufreader;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

const CARDS: [(char, u8); 13] = [
    ('A', 14),
    ('K', 13),
    ('Q', 12),
    // ('J', 11), // Jack
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
    ('J', 1), // wild Joker
];

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    card_vals: [u8; 5],
    hand_str: String,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare based on hand_type first
        match (&self.hand_type, &other.hand_type) {
            (self_type, other_type) if self_type != other_type => self_type.cmp(&other_type),
            _ => {
                // If hand types are the same, compare card_vals index by index
                for i in 0..self.card_vals.len() {
                    match self.card_vals[i].cmp(&other.card_vals[i]) {
                        Ordering::Equal => continue,
                        non_equal_ordering => return non_equal_ordering,
                    }
                }
                Ordering::Equal // All elements are equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_vals == other.card_vals
    }
}

impl Eq for Hand {}

fn classify_hand(hand: &str, card_values: &HashMap<char, u8>) -> Hand {
    let mut card_counts: HashMap<char, u8> = HashMap::new();
    let mut i = 0;
    let mut card_vals: [u8; 5] = [0; 5];
    for char in hand.chars() {
        // get the value of the card
        if let Some(val) = card_values.get(&char) {
            card_vals[i] = *val;
        }
        // keep track of how many times this card is in the hand
        let count = card_counts.entry(char).or_insert(0);
        *count += 1;

        // increment the card index
        i += 1;
    }

    // Joker's wild rule
    // Find the entry with the highest value
    let mut highest_key = None;
    let mut highest_value = 0;

    for (key, &value) in &card_counts {
        if value > highest_value && key != &'J' {
            highest_key = Some(key);
            highest_value = value;
        }
    }

    // Check if 'J' exists and add its value to the highest key
    if let Some(highest_key) = highest_key {
        if let Some(value_to_add) = card_counts.get(&'J') {
            *card_counts.entry(*highest_key).or_insert(0) += *value_to_add;
        }
    }

    if card_counts.len() > 1 {
        card_counts.remove(&'J');
    }
    // end Joker's wild rule

    // Iterate over the card counts and create an array that tallies the
    // frequencies of:
    // [singles, pairs, threes, fours, fives]
    let mut count_freq = [0; 5];
    for count in card_counts.values() {
        count_freq[(count - 1) as usize] += 1
    }

    // based on the frequencies, determine the hand type
    let hand_type = match count_freq {
        [5, 0, 0, 0, 0] => PokerHand::HighCard,
        [3, 1, 0, 0, 0] => PokerHand::OnePair,
        [1, 2, 0, 0, 0] => PokerHand::TwoPair,
        [2, 0, 1, 0, 0] => PokerHand::ThreeOfAKind,
        [0, 1, 1, 0, 0] => PokerHand::FullHouse,
        [1, 0, 0, 1, 0] => PokerHand::FourOfAKind,
        [0, 0, 0, 0, 1] => PokerHand::FiveOfAKind,
        _ => PokerHand::Undefined,
    };

    Hand {
        hand_type,
        card_vals,
        hand_str: hand.to_string(),
        bid: 0,
    }
}

pub fn run() -> io::Result<()> {
    // Convert the array of tuples into a HashMap
    let map: HashMap<_, _> = CARDS.iter().cloned().collect();

    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day7/input.txt")?;

    let mut sorted_hands: Vec<Hand> = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error

        // Splitting the line on whitespace
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let mut poker_hand = classify_hand(tokens[0], &map);
        if let Ok(bid) = tokens[1].parse::<u32>() {
            poker_hand.bid = bid;
        }

        insert_sorted(&mut sorted_hands, poker_hand);
    }

    let mut winnings: u64 = 0;
    let mut rank: u32 = 0;
    for hand in sorted_hands {
        println!("{}\t{}\t{:?}", hand.hand_str, hand.bid, hand.hand_type);
        rank += 1;
        winnings += (rank * hand.bid) as u64
    }
    println!("Winnings: {winnings}");
    Ok(())
}

fn insert_sorted(sorted_hands: &mut Vec<Hand>, hand: Hand) {
    let index = match sorted_hands.binary_search(&hand) {
        Ok(index) => index,
        Err(index) => index,
    };

    sorted_hands.insert(index, hand);
}
