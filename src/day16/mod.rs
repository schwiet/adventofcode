use super::util::open_file_as_bufreader;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Path = HashMap<(i32, i32), HashSet<(i32, i32)>>;

// I think this could be optimized further by mapping the start position passed
// in to the count of energized cells when returning from the function
fn get_visited(mat: &[Vec<char>], start: (i32, i32), vel: (i32, i32), visited: &Path) -> Path {
    // println!("\nStarting ({}, {})", start.0, start.1);
    let mut visited = visited.clone();

    let mut next_row = start.0;
    let mut next_col = start.1;
    let mut new_vel = (vel.0, vel.1);

    'follow: while next_row >= 0
        && next_row < mat.len() as i32
        && next_col >= 0
        && next_col < mat[0].len() as i32
    {
        let crossed_paths = visited
            .entry((next_row, next_col))
            .or_insert_with(HashSet::new);

        if crossed_paths.get(&(new_vel.0, new_vel.1)) != None {
            break 'follow;
        }

        // println!("Visiting ({}, {})", next_row, next_col);
        crossed_paths.insert((new_vel.0, new_vel.1));

        let r = next_row as usize;
        let c = next_col as usize;
        match mat[r][c] {
            '/' => {
                new_vel = (new_vel.1 * -1, new_vel.0 * -1);
                next_col += new_vel.0;
                next_row += new_vel.1;
            }
            '\\' => {
                new_vel = (new_vel.1, new_vel.0);
                next_col += new_vel.0;
                next_row += new_vel.1;
            }
            '-' => {
                if new_vel.0 == 0 {
                    // TODO: recursive call
                    let br1 = get_visited(mat, (next_row, next_col + 1), (1, 0), &visited);
                    visited.extend(br1);
                    let br2 = get_visited(mat, (next_row, next_col - 1), (-1, 0), &visited);
                    visited.extend(br2);
                    break 'follow;
                } else {
                    // doesn't change velocity, but does change next col
                    next_col += new_vel.0;
                }
            }
            '|' => {
                if new_vel.1 == 0 {
                    // TODO: recursive call
                    let br1 = get_visited(mat, (next_row + 1, next_col), (0, 1), &visited);
                    visited.extend(br1);
                    let br2 = get_visited(mat, (next_row - 1, next_col), (0, -1), &visited);
                    visited.extend(br2);
                    break 'follow;
                } else {
                    // doesn't change velocity, but does change next row
                    next_row += new_vel.1;
                }
            }
            _ => {
                next_col += new_vel.0;
                next_row += new_vel.1;
            }
        }
    }
    return visited;
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day16/input.txt")?;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        matrix.push(line.chars().collect())
    }

    let mut max_visited = 0usize;
    let mut visited: Path = HashMap::new();

    for (i, row) in matrix.iter().enumerate() {
        // for the first and last row, start from each column
        if i == 0 || i == matrix.len() - 1 {
            let start_dir = if i == 0 { 1i32 } else { -1i32 };
            for j in (0..row.len()) {
                visited = get_visited(
                    &matrix,
                    (i as i32, j as i32),
                    (0, start_dir),
                    &HashMap::new(),
                );
                if visited.len() > max_visited {
                    max_visited = visited.len();
                }
            }
        }

        visited = get_visited(&matrix, (i as i32, 0), (1, 0), &HashMap::new());
        if visited.len() > max_visited {
            max_visited = visited.len();
        }

        visited = get_visited(
            &matrix,
            (i as i32, row.len() as i32),
            (-1, 0),
            &HashMap::new(),
        );
        if visited.len() > max_visited {
            max_visited = visited.len();
        }
    }
    println!("Energized: {}", max_visited);

    Ok(())
}
