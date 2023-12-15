use super::util::open_file_as_bufreader;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn print_boxes(boxes: &[Vec<(String, u32)>]) {
    let mut sum = 0usize;
    for (i, b) in boxes.iter().enumerate() {
        if b.len() > 0 {
            // println!("{:?} \t\tBox: {i}", b);
            for (s, lens) in b.iter().enumerate() {
                sum += (i + 1) * (s + 1) * (lens.1 as usize)
            }
        }
    }
    println!("FL Sum: {sum}");
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day15/input.txt")?;

    let mut sum = 0u64;

    let operations = ['-', '='];

    // Create a boxes with 256 empty LinkedLists
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for line in reader.lines() {
        let line = line?;

        let steps = line.split(',');

        for step in steps {
            // part 1, to calculate sum of hashes
            let hash = step
                .chars()
                .fold(0u64, |acc, c| ((acc + (c as u8 as u64)) * 17) % 256);
            sum += hash;

            let params: Vec<&str> = step.split(|c| operations.contains(&c)).collect();

            if params.len() == 2 {
                let label = params[0].to_string();
                let hash = label
                    .chars()
                    .fold(0u64, |acc, c| ((acc + (c as u8 as u64)) * 17) % 256);
                if params[1] == "" {
                    // subtraction step
                    if let Some(curr_box) = boxes.get_mut(hash as usize) {
                        curr_box.retain(|l| l.0 != label);
                    }
                } else {
                    match params[1].parse::<u32>() {
                        Ok(focal_length) => {
                            if let Some(lenses) = boxes.get_mut(hash as usize) {
                                if let Some((index, _)) =
                                    lenses.iter_mut().enumerate().find(|l| l.1 .0 == label)
                                {
                                    lenses[index] = (label, focal_length);
                                } else {
                                    lenses.push((label, focal_length));
                                }
                            }
                        }
                        Err(_) => {
                            println!("Not a valid number: {}", params[1])
                        }
                    }
                }
            } else {
                println!("Invalid step {step}");
            }
        }
    }

    println!("Sum: {sum}");
    print_boxes(&boxes);
    Ok(())
}
