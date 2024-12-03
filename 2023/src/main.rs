mod day1;
mod day2;
mod day3;
//
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
// 
mod day21;
// 
mod day25;
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
        "3" => day3::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        "7" => day7::run(),
        "8" => day8::run(),
        "9" => day9::run(),
        "10" => day10::run(),
        "11" => day11::run(),
        "12" => day12::run(),
        "13" => day13::run(),
        "14" => day14::run(),
        "15" => day15::run(),
        "16" => day16::run(),
        "17" => day17::run(),
        "18" => day18::run(),
        "21" => day21::run(),
        "25" => day25::run(),
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
