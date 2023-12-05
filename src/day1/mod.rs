use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const DIGIT_DEFINITIONS: &[(&str, u32)] = &[
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_digit(chars: &[char], reverse: bool, count_spelled: bool) -> Option<u32> {
    let mut current_word = String::new();

    // Reverse the chars if needed and then create an iterator
    let chars_iter = if reverse {
        chars.iter().rev().cloned().collect::<Vec<_>>()
    } else {
        chars.to_vec()
    };

    for ch in chars_iter {
        if count_spelled && ch.is_alphabetic() {
            if reverse {
                current_word.insert(0, ch);
            } else {
                current_word.push(ch);
            }

            // search the word for one of the spelled-out digits
            for &(word, number) in DIGIT_DEFINITIONS {
                if current_word.contains(word) {
                    return Some(number);
                }
            }
        } else if ch.is_digit(10) {
            return ch.to_digit(10);
        }
    }

    None
}

pub fn run() -> io::Result<()> {
    // Specify the file path
    let path = Path::new("src/day1/input.txt");

    // Open the file in read-only mode (ignoring errors)
    let file = File::open(&path)?;

    // Create a new BufReader for the file
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error
        let chars: Vec<char> = line.chars().collect();

        let first_digit = find_digit(&chars, false, true);
        let last_digit = find_digit(&chars, true, true);

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            println!(":{first},{last}");
            sum += first * 10 + last
        }
    }

    println!("Sum of all two-digit numbers: {}", sum);
    Ok(())
}
