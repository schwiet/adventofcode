use super::util::open_file_as_bufreader;
use regex::Regex;
use std::io::{self, BufRead, ErrorKind};

// Helper function to parse a hex value and convert errors to io::Error
fn parse_hex(hex: &str) -> io::Result<u64> {
    u64::from_str_radix(hex, 16).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

fn parse_hex_instructions(hex: &str) -> io::Result<(char, u64)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid hex format",
        ));
    }

    let length = parse_hex(&hex[1..6])?;
    let dir = hex.chars().nth(6).ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "Invalid hex format",
    ))?;

    Ok((dir, length))
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day18/input.txt")?;

    // vector start
    let mut current = (0i64, 0i64);

    // sums used in Shoelace Formula
    let mut trap_sum = 0i64;
    let mut perim = 0usize;

    let re = Regex::new(r"^([A-Za-z]) (\d+) \((#[0-9a-fA-F]{6})\)$").unwrap();
    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(&line).ok_or(io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid line format",
        ))?;

        let color_hex = caps.get(3).unwrap().as_str();

        // P1 Block
        // let character = caps.get(1).unwrap().as_str().chars().next().unwrap();
        // let number = caps
        //     .get(2)
        //     .unwrap()
        //     .as_str()
        //     .parse::<u32>()
        //     .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Invalid number"))?;

        // P2 Block
        let (character, number) = parse_hex_instructions(color_hex)?;

        // same for P1 and P@
        let length = number as i64;

        // P1 Block
        // let end = match character {
        //     'R' => (current.0 + length, current.1),
        //     'L' => (current.0 - length, current.1),
        //     'U' => (current.0, current.1 + length),
        //     'D' => (current.0, current.1 - length),
        //     _ => return Err(io::Error::new(ErrorKind::Other, "Invalid Character")),
        // };

        // P2 Block
        let end = match character {
            '0' => (current.0 + length, current.1),
            '2' => (current.0 - length, current.1),
            '3' => (current.0, current.1 + length),
            '1' => (current.0, current.1 - length),
            _ => return Err(io::Error::new(ErrorKind::Other, "Invalid Character")),
        };

        // using the Shoelace Formula to sum the trapezoid formed by each vector
        // and its transpose
        trap_sum += current.0 * end.1 - end.0 * current.1;
        // I'm also summing the perimeter formed by the source directions, because
        // each step also has an area of 1
        perim += length as usize;

        // make the starting point of the next vector the same as the end of
        // this vector
        current = end;
    }

    // get the shoelace-based area and add each unit of perimeter
    let area = (trap_sum.abs() as usize + perim) / 2 + 1; // I don't know why I'm off by one 🤔
                                                          // println!("Perimeter: {perim}");
                                                          // println!("trap_sum: {trap_sum}");
    println!("Area: {area}");

    Ok(())
}
