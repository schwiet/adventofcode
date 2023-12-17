use super::util::open_file_as_bufreader;
use std::io::{self, BufRead};

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day10/input.txt")?;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut path: Vec<(char, i32, i32)> = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Using ? here to propagate error

        matrix.push(line.chars().collect());
    }

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if is_in_loop((-1, -1), (-1, -1), (x as i32, y as i32), &matrix, &mut path) {
                println!("found {:?}", path.len() / 2);
                for d in path {
                    if d.0 == '|' {
                        matrix[d.2 as usize][d.1 as usize] = '┇'
                    } else if d.0 == 'L' {
                        matrix[d.2 as usize][d.1 as usize] = '┗'
                    } else if d.0 == 'F' {
                        matrix[d.2 as usize][d.1 as usize] = '┏'
                    } else if d.0 == 'J' || d.0 == 'S' {
                        // note: I happen to know S is a J in my input
                        matrix[d.2 as usize][d.1 as usize] = '┛'
                    } else if d.0 == '7' {
                        matrix[d.2 as usize][d.1 as usize] = '┓'
                    } else {
                        matrix[d.2 as usize][d.1 as usize] = '┅'
                    };
                }

                let mut sum: u64 = 0;
                print!("\n");
                for row in matrix {
                    let mut corner: char = ' ';
                    let mut vert_ct: u64 = 0;

                    for cell in row {
                        match cell {
                            '┇' => {
                                vert_ct += 1;
                                print!("{cell}");
                            }
                            '┗' => {
                                corner = cell;
                                print!("{cell}");
                            }
                            '┏' => {
                                corner = cell;
                                print!("{cell}");
                            }
                            '┛' => {
                                print!("{cell}");
                                if corner == '┏' {
                                    vert_ct += 1;
                                }
                                corner = ' ';
                            }
                            '┓' => {
                                print!("{cell}");
                                if corner == '┗' {
                                    vert_ct += 1;
                                }
                                corner = ' ';
                            }
                            '┅' => {
                                print!("{cell}");
                            }
                            _ => {
                                if vert_ct % 2 != 0 {
                                    print!("I");
                                    sum += 1;
                                } else {
                                    print!("O");
                                }
                            }
                        }
                    }
                    print!("\n");
                }

                println!("Tiles Inside: {sum}");

                return Ok(());
            }
        }
    }

    Ok(())
}

fn is_in_loop(
    start: (i32, i32),
    prev: (i32, i32),
    curr: (i32, i32),
    mat: &Vec<Vec<char>>,
    path: &mut Vec<(char, i32, i32)>,
) -> bool {
    let (start_x, start_y) = start;
    let (prev_x, prev_y) = prev;
    let (curr_x, curr_y) = curr;

    let mut start = start;
    if curr_x == start_x && curr_y == start_y {
        return true;
    } else if start == (-1, -1) {
        start = curr;
    }

    if curr_x >= mat[0].len() as i32 || curr_y >= mat.len() as i32 || curr_x < 0 || curr_y < 0 {
        return false;
    }

    let sym = mat[curr_y as usize][curr_x as usize];

    if sym == 'S' {
        let branches = if prev_y < curr_y {
            [
                (curr_x - 1, curr_y),
                (curr_x + 1, curr_y),
                (curr_x, curr_y + 1),
            ]
        } else if prev_y > curr_y {
            [
                (curr_x - 1, curr_y),
                (curr_x + 1, curr_y),
                (curr_x, curr_y - 1),
            ]
        } else if prev_x < curr_x {
            [
                (curr_x + 1, curr_y),
                (curr_x, curr_y + 1),
                (curr_x, curr_y - 1),
            ]
        } else {
            [
                (curr_x - 1, curr_y),
                (curr_x, curr_y + 1),
                (curr_x, curr_y - 1),
            ]
        };

        let p1 = is_in_loop(start, curr, branches[0], mat, path);
        let p2 = is_in_loop(start, curr, branches[1], mat, path);
        let p3 = is_in_loop(start, curr, branches[2], mat, path);
        if p1 || p2 || p3 {
            path.push((sym, curr_x, curr_y));
            return true;
        } else {
            return false;
        }
    };

    let next = match sym {
        '|' => {
            if prev_x != curr_x {
                (-1, -1)
            } else if prev_y < curr_y {
                (curr_x, curr_y + 1)
            } else {
                (curr_x, curr_y - 1)
            }
        }
        '-' => {
            if prev_y != curr_y {
                (-1, -1)
            } else if prev_x < curr_x {
                (curr_x + 1, curr_y)
            } else {
                (curr_x - 1, curr_y)
            }
        }
        'L' => {
            if prev_x == -1 || (prev_y < curr_y && prev_x == curr_x) {
                (curr_x + 1, curr_y)
            } else if prev_x > curr_x && prev_y == curr_y {
                (curr_x, curr_y - 1)
            } else {
                (-1, -1)
            }
        }
        'J' => {
            if prev_x == -1 || (prev_y < curr_y && prev_x == curr_x) {
                (curr_x - 1, curr_y)
            } else if prev_x < curr_x && prev_y == curr_y {
                (curr_x, curr_y - 1)
            } else {
                (-1, -1)
            }
        }
        '7' => {
            if prev_x == -1 || (prev_y > curr_y && prev_x == curr_x) {
                (curr_x - 1, curr_y)
            } else if prev_x < curr_x && prev_y == curr_y {
                (curr_x, curr_y + 1)
            } else {
                (-1, -1)
            }
        }
        'F' => {
            if prev_x == -1 || (prev_y > curr_y && prev_x == curr_x) {
                (curr_x + 1, curr_y)
            } else if prev_x > curr_x && prev_y == curr_y {
                (curr_x, curr_y + 1)
            } else {
                (-1, -1)
            }
        }
        _ => (-1, -1),
    };

    if is_in_loop(start, curr, next, mat, path) {
        path.push((sym, curr_x, curr_y));
        return true;
    } else {
        return false;
    }
}
