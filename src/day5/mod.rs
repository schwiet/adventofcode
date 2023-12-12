use super::util::open_file_as_bufreader;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    range: u64,
}

const VAL_PTRN: &str = r"^\d+[\s\d]*$";

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let mut reader = open_file_as_bufreader("src/day5/input.txt")?;

    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil: Vec<Mapping> = Vec::new();
    let mut soil_to_fert: Vec<Mapping> = Vec::new();
    let mut fert_to_water: Vec<Mapping> = Vec::new();
    let mut water_to_light: Vec<Mapping> = Vec::new();
    let mut light_to_temp: Vec<Mapping> = Vec::new();
    let mut temp_to_hum: Vec<Mapping> = Vec::new();
    let mut hum_to_loc: Vec<Mapping> = Vec::new();

    let val_row_re = Regex::new(VAL_PTRN).unwrap();

    let mut current_map: Option<&mut Vec<Mapping>> = None;
    for line in reader.lines() {
        let line = line?;

        let mut tokens = line.split_whitespace();

        if val_row_re.is_match(&line) {
            let (t1, t2, t3) = (
                tokens.next().and_then(|s| s.parse::<u64>().ok()),
                tokens.next().and_then(|s| s.parse::<u64>().ok()),
                tokens.next().and_then(|s| s.parse::<u64>().ok()),
            );
            match (t1, t2, t3) {
                (Some(dest_start), Some(src_start), Some(range)) => match &mut current_map {
                    Some(m) => m.push(Mapping {
                        dest_start,
                        src_start,
                        range,
                    }),
                    None => {}
                },
                _ => {}
            }
        } else {
            match tokens.next() {
                Some("seeds:") => {
                    seeds = tokens.map(|s| s.parse::<u64>().unwrap()).collect();
                }
                Some("seed-to-soil") => {
                    current_map = Some(&mut seed_to_soil);
                }
                Some("soil-to-fertilizer") => {
                    current_map = Some(&mut soil_to_fert);
                }
                Some("fertilizer-to-water") => {
                    current_map = Some(&mut fert_to_water);
                }
                Some("water-to-light") => {
                    current_map = Some(&mut water_to_light);
                }
                Some("light-to-temperature") => {
                    current_map = Some(&mut light_to_temp);
                }
                Some("temperature-to-humidity") => {
                    current_map = Some(&mut temp_to_hum);
                }
                Some("humidity-to-location") => {
                    current_map = Some(&mut hum_to_loc);
                }
                _ => {
                    // skip blank line
                }
            }
        }
    }

    for seed in seeds {
        // let soil = seed_to_soil.binary_search_by(|m| probe.cmp(&seek))
    }

    println!("seeds: {:?}", seeds);
    println!("seed_to_soil: {:?}", seed_to_soil);
    Ok(())
}
