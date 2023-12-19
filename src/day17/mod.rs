use super::util::open_file_as_bufreader;
use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    position: (i32, i32),
    distance: usize,
    direction: (i32, i32),
    direction_count: u8,
}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day17/example.txt")?;

    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let mut queue: VecDeque<Node> = VecDeque::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        let row: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as u8)
            .collect();

        matrix.push(row);
    }

    // define the goal as the node in the lower right corner of the grid
    let goal = (matrix.len() as i32 - 1, matrix[0].len() as i32 - 1);

    // push the starting point to the queue
    queue.push_back(Node {
        position: (0, 0),
        distance: 0,
        direction: (0, 1),
        direction_count: 0,
    });

    // search the priority queue
    while let Some(node) = queue.pop_front() {
        if node.position == goal {
            println!("END! {:?}", node);
            break;
        }

        let (l, r) = match node.direction {
            // up
            (-1, 0) => ((0i32, -1i32), (0i32, 1i32)),
            // right
            (0, 1) => ((-1, 0), (1, 0)),
            // down
            (1, 0) => ((0, 1), (0, -1)),
            // left
            (0, -1) => ((1, 0), (-1, 0)),
            // invalid
            (_, _) => {
                break;
            }
        };

        update_or_add_node(
            node.position,
            node.distance,
            goal,
            &matrix,
            &mut queue,
            l,
            1,
            1,
        );
        update_or_add_node(
            node.position,
            node.distance,
            goal,
            &matrix,
            &mut queue,
            r,
            1,
            1,
        );
        print!("trying ( ");
        let mut straight_dist = node.distance;
        // TODO: I'm not totally sure this is the way to handle the consecutive step constraint
        for i in (node.direction_count + 1)..4 {
            print!("{i} ");
            if let Some(dist) = update_or_add_node(
                node.position,
                straight_dist,
                goal,
                &matrix,
                &mut queue,
                (node.direction.0, node.direction.1),
                node.direction_count + i,
                i as i32,
            ) {
                straight_dist += dist;
            }
        }
        println!(") Steps from, {:?}", node.position);

        println!("QUEUE");
        for node in queue.iter() {
            println!("\t{:?}", node);
        }

        // TODO: sort
    }
    Ok(())
}

fn update_or_add_node(
    start_pos: (i32, i32),
    dist_offset: usize,
    goal: (i32, i32),
    matrix: &Vec<Vec<u8>>,
    queue: &mut VecDeque<Node>,
    direction: (i32, i32),
    direction_count: u8,
    look_ahead: i32, // TODO: unsure about this
) -> Option<usize> {
    let pos = (
        start_pos.0 + (direction.0 * look_ahead),
        start_pos.1 + (direction.1 * look_ahead),
    );

    if pos.0 >= 0 && pos.0 <= goal.0 && pos.1 >= 0 && pos.1 <= goal.1 {
        let dist_to_next = matrix[pos.0 as usize][pos.1 as usize] as usize;
        let distance = dist_offset + dist_to_next;

        if let Some(next_node) = queue.iter_mut().find(|x| x.position == pos) {
            if distance < next_node.distance {
                next_node.distance = distance;
                next_node.direction = direction;
                next_node.direction_count = direction_count;
            }
        } else {
            let new_node = Node {
                position: pos,
                distance: distance,
                direction: direction,
                direction_count: direction_count,
            };
            queue.push_front(new_node);
        }

        return Some(distance);
    }

    return None;
}
