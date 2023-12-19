use super::util::open_file_as_bufreader;
use core::cmp::Ordering;
use std::collections::hash_map::Entry::Occupied;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

type State = (i32, i32, i32, i32, u8);

#[derive(Debug, Clone, Copy)]
struct Node {
    position: (i32, i32),
    distance: usize,
    estimate: usize,
    direction: (i32, i32),
    direction_count: u8,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_total = other.distance + other.estimate;
        let node_total = self.distance + self.estimate;
        other_total.cmp(&node_total) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction_count == other.direction_count
    }
}

impl Eq for Node {}

pub fn run() -> io::Result<()> {
    // Create a new BufReader for the file
    let reader = open_file_as_bufreader("src/day17/input.txt")?;

    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut map: HashMap<State, Node> = HashMap::new();
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

    // push the starting point to the heap
    let start = Node {
        position: (0, 0),
        distance: 0,
        estimate: 0,
        direction: (0, 1),
        direction_count: 0,
    };
    map.insert((0, 0, 0, 1, 0), start);
    heap.push(start.clone());

    // search the priority heap
    while let Some(next) = heap.pop() {
        let state = (
            next.position.0,
            next.position.1,
            next.direction.0,
            next.direction.1,
            next.direction_count,
        );
        let node = map.get(&state).unwrap();

        // skip stale entries
        if node.distance != next.distance {
            continue;
        }

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

        let position = node.position;
        let distance = node.distance;
        let direction = node.direction;
        let direction_count = node.direction_count;

        update_or_add_node(position, distance, goal, &matrix, &mut heap, &mut map, l, 1);
        update_or_add_node(position, distance, goal, &matrix, &mut heap, &mut map, r, 1);
        if direction_count < 3 {
            update_or_add_node(
                position,
                distance,
                goal,
                &matrix,
                &mut heap,
                &mut map,
                (direction.0, direction.1),
                direction_count + 1,
            );
        }
        // println!(") Steps from, {:?}", position);
        // println!("nodes");
        for node in heap.iter() {
            // println!("\t{:?}", node);
        }
    }

    // println!("results");
    // for (i, row) in matrix.iter().enumerate() {
    //   for j in 0..row.len() {
    //     if let Some(node) = map.get(&(i as i32, j as i32)) {
    //       print!("{}\t", node.distance);
    //     } else {
    //       print!("x\t");
    //     }
    //   }
    //   println!()
    // }

    Ok(())
}

fn update_or_add_node(
    start_pos: (i32, i32),
    dist_offset: usize,
    goal: (i32, i32),
    matrix: &Vec<Vec<u8>>,
    heap: &mut BinaryHeap<Node>,
    map: &mut HashMap<State, Node>,
    direction: (i32, i32),
    direction_count: u8,
) -> Option<usize> {
    let pos = (start_pos.0 + direction.0, start_pos.1 + direction.1);
    let state = (pos.0, pos.1, direction.0, direction.1, direction_count);

    if pos.0 >= 0 && pos.0 <= goal.0 && pos.1 >= 0 && pos.1 <= goal.1 {
        let dist_to_next = matrix[pos.0 as usize][pos.1 as usize] as usize;
        let distance = dist_offset + dist_to_next;
        let estimate = (goal.0 - pos.0 + goal.1 - pos.1) as usize;

        if let Occupied(mut next_node) = map.entry(state) {
            if distance < next_node.get().distance {
                next_node.get_mut().distance = distance;
                next_node.get_mut().estimate = estimate;
                next_node.get_mut().direction = direction;
                next_node.get_mut().direction_count = direction_count;
            }
        } else {
            let new_node = Node {
                position: pos,
                distance,
                estimate,
                direction,
                direction_count,
            };
            heap.push(new_node);
            map.insert(state, new_node.clone());
        }

        return Some(distance);
    }

    return None;
}
