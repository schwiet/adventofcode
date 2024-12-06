package day6

import (
	"fmt"
	"os"
	"strings"
)

func Solve() error {
	// data, err := os.ReadFile("day6/example.txt")
	data, err := os.ReadFile("day6/input.txt")

	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	var grid [][]bool

	var pos Position
	for i, line := range lines {
		if len(line) == 0 {
			continue
		}
		grid = append(grid, make([]bool, len(line)))
		for j, char := range line {
			grid[i][j] = char == '#'
			if char == '^' {
				pos = Position{x: j, y: i}
			}
		}
	}
	start := Position{x: pos.x, y: pos.y}

	outOfBounds := false
	direction := Position{x: 0, y: -1}
	visited := make(map[Position]map[Position]bool)
	visited[pos] = map[Position]bool{direction: true}
	loopChan := make(chan *Position)
	numRoutines := 0
	for !outOfBounds {
		// take a step
		pos = moveForward(pos, direction)

		// if it's out of bounds, break
		if pos.x < 0 || pos.x >= len(grid[0]) || pos.y < 0 || pos.y >= len(grid) {
			fmt.Printf("out of bounds: %d, %d\n", pos.x, pos.y)
			outOfBounds = true
			break
		}

		obstacle := grid[pos.y][pos.x]
		// if it is an obstacle, take a step back and turn right
		if obstacle {
			pos = moveBackward(pos, direction)
			direction = turnRight(direction)
			// if it's not an obstacle, mark that we've been here, going this direction
			// also check if turning right from here will loop
			_, beenHere := visited[pos]
			if !beenHere {
				visited[pos] = map[Position]bool{direction: true}
			} else {
				// track that we've been here facing this direction
				visited[pos][direction] = true
			}
			continue
		}

		// if it's not an obstacle, mark that we've been here, going this direction
		// also check if turning right from here will loop
		_, beenHere := visited[pos]
		if !beenHere {
			visited[pos] = map[Position]bool{direction: true}
		} else {
			// track that we've been here facing this direction
			visited[pos][direction] = true
		}

		look_ahead := Position{x: pos.x + direction.x, y: pos.y + direction.y}
		oob_ahead := look_ahead.x < 0 || look_ahead.x >= len(grid[0]) || look_ahead.y < 0 || look_ahead.y >= len(grid)
		obstacle_ahead := oob_ahead || grid[look_ahead.y][look_ahead.x]
		start_pos_ahead := look_ahead.x == start.x && look_ahead.y == start.y
		// if there is not an obstacle ahead, check if turning right will loop
		if !obstacle_ahead && !start_pos_ahead {
			// Create a deep copy of visited for this goroutine
			visitedCopy := make(map[Position]map[Position]bool)
			for k, v := range visited {
				visitedCopy[k] = make(map[Position]bool)
				for dir, isVisited := range v {
					visitedCopy[k][dir] = isVisited
				}
			}
			// start a goroutine to check if turning right here will loop
			go willLoopFromHere(
				visitedCopy,
				pos,
				turnRight(direction),
				grid,
				loopChan,
			)
			numRoutines += 1
		}
	}

	loopPositions := make(map[Position]bool)
	for i := 0; i < numRoutines; i++ {
		if loopPos := <-loopChan; loopPos != nil {
			loopPositions[*loopPos] = true
		}
	}

	// Print the grid with visited markers
	print_it := false
	for y := 0; y < len(grid) && print_it; y++ {
		for x := 0; x < len(grid[0]); x++ {
			pos := Position{x: x, y: y}
			if grid[y][x] {
				fmt.Print("#")
			} else if visits, ok := visited[pos]; ok {
				// Check which directions this position was visited from
				hasHorizontal := visits[Position{x: 1, y: 0}] || visits[Position{x: -1, y: 0}]
				hasVertical := visits[Position{x: 0, y: 1}] || visits[Position{x: 0, y: -1}]

				if loopPositions[pos] && grid[y][x] {
					fmt.Printf("out of bounds: %d, %d\n", x, y)
					os.Exit(1)
				}
				if loopPositions[pos] {
					fmt.Print("R")
				} else if pos.x == start.x && pos.y == start.y {
					fmt.Print("X")
				} else if hasHorizontal && hasVertical {
					fmt.Print("+")
				} else if hasHorizontal {
					fmt.Print("-")
				} else if hasVertical {
					fmt.Print("|")
				}
			} else {
				fmt.Print("•")
			}
		}
		fmt.Println()
	}

	fmt.Printf("visited: %v\n", len(visited))
	fmt.Printf("loop count: %d\n", len(loopPositions))
	return nil
}

func willLoopFromHere(
	visited map[Position]map[Position]bool,
	pos Position,
	direction Position,
	grid [][]bool,
	loopChan chan<- *Position,
) {
	loopPos := moveForward(pos, turnLeft(direction))

	for {
		// determine if we've already searched this route
		inner, searchedHere := visited[pos]
		if searchedHere && inner[direction] {
			loopChan <- &loopPos
			return
		} else if searchedHere {
			inner[direction] = true
		} else {
			inner = make(map[Position]bool)
			inner[direction] = true
			visited[pos] = inner
		}

		// carry on
		pos = moveForward(pos, direction)
		if pos.x < 0 || pos.x >= len(grid[0]) || pos.y < 0 || pos.y >= len(grid) {
			loopChan <- nil
			return
		}

		// if it's an obstacle, back up, turn right and keep going
		obstacle := grid[pos.y][pos.x]
		if obstacle {
			// back up
			pos = moveBackward(pos, direction)
			// turn right
			direction = turnRight(direction)
		}
	}
}

func turnRight(direction Position) Position {
	return Position{x: -direction.y, y: direction.x}
}

func turnLeft(direction Position) Position {
	return Position{x: direction.y, y: -direction.x}
}

func moveForward(pos Position, direction Position) Position {
	return Position{x: pos.x + direction.x, y: pos.y + direction.y}
}

func moveBackward(pos Position, direction Position) Position {
	return Position{x: pos.x - direction.x, y: pos.y - direction.y}
}

type Position struct {
	x int
	y int
}
