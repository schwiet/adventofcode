mod day1;
mod day2;
//
mod day7;
mod day8;
mod day9;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a day to run (e.g., `cargo run 1`)");
        return;
    }

    let result = match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "7" => day7::run(),
        "8" => day8::run(),
        "9" => day9::run(),
        // ... other days
        _ => {
            println!("Day not recognized");
            return;
        }
    };

    if let Err(e) = result {
        // Handle the error
        println!("An error occurred: {}", e);
    }
}
