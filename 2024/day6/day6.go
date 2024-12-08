package day6

import (
	"fmt"
	"os"
	"strings"
)

type obstacles struct {
	grid [][]bool
}

func (o *obstacles) handleObstacle(pos Coordinate, direction Direction) (Coordinate, Direction) {
	if o.isObstacle(pos) {
		pos = moveBackward(pos, direction)
		direction = turnRight(direction)
	}
	return pos, direction
}

func (o *obstacles) isOutOfBounds(pos Coordinate) bool {
	return pos.x < 0 || pos.x >= len(o.grid[0]) || pos.y < 0 || pos.y >= len(o.grid)
}

func (o *obstacles) isObstacle(pos Coordinate) bool {
	return o.grid[pos.y][pos.x]
}

func turnRight(direction Direction) Direction {
	return Direction{x: -direction.y, y: direction.x}
}

func moveForward(pos Coordinate, direction Direction) Coordinate {
	return Coordinate{x: pos.x + direction.x, y: pos.y + direction.y}
}

func moveBackward(pos Coordinate, direction Direction) Coordinate {
	return Coordinate{x: pos.x - direction.x, y: pos.y - direction.y}
}

type Coordinate struct {
	x int
	y int
}
type Direction Coordinate

func (c Coordinate) equals(other Coordinate) bool {
	return c.x == other.x && c.y == other.y
}

func Solve() error {
	// data, err := os.ReadFile("day6/example.txt")
	data, err := os.ReadFile("day6/input.txt")

	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	var inputMap *obstacles = &obstacles{}

	var start, pos Coordinate
	for i, line := range lines {
		if len(line) == 0 {
			continue
		}
		inputMap.grid = append(inputMap.grid, make([]bool, len(line)))
		for j, char := range line {
			inputMap.grid[i][j] = char == '#'
			if char == '^' {
				start = Coordinate{x: j, y: i}
			}
		}
	}
	pos = start

	outOfBounds := false
	direction := Direction{x: 0, y: -1}
	visited := make(map[Coordinate]map[Direction]bool)
	loopCount := 0
	for !outOfBounds {
		// if it's out of bounds, break
		if inputMap.isOutOfBounds(pos) {
			fmt.Printf("out of bounds: %d, %d\n", pos.x, pos.y)
			outOfBounds = true
			break
		}

		// if there is an obstacle, take a step back and turn right
		pos, direction = inputMap.handleObstacle(pos, direction)

		// see if we've ever been here before
		_, beenHere := visited[pos]
		if !beenHere {
			visited[pos] = make(map[Direction]bool)
		}
		// track that we've been here facing this direction
		visited[pos][direction] = true

		// determine if next step is eligible for placing an obstacle
		look_ahead := moveForward(pos, direction)
		_, beenThere := visited[look_ahead]
		// if we haven't been there, it isn't out of bounds, isn't an obstacle
		// and it isn't the start position, it is eligible
		eligible := (!beenThere &&
			!inputMap.isOutOfBounds(look_ahead) &&
			!inputMap.isObstacle(look_ahead) &&
			!start.equals(look_ahead))
		if eligible {
			// Create a deep copy of visited for this goroutine
			visitedCopy := make(map[Coordinate]map[Direction]bool, len(visited))
			for k, v := range visited {
				visitedCopy[k] = make(map[Direction]bool, len(v))
				for dir := range v {
					visitedCopy[k][dir] = true
				}
			}
			// check if turning right here will loop
			loop := willLoopFromHere(visitedCopy, pos, turnRight(direction), inputMap, look_ahead)
			if loop != nil {
				loopCount += 1
			}
		}

		// take a step
		pos = look_ahead
	}

	fmt.Printf("visited: %v\n", len(visited))
	fmt.Printf("loop count: %d\n", loopCount)
	return nil
}

func willLoopFromHere(
	visited map[Coordinate]map[Direction]bool,
	pos Coordinate,
	direction Direction,
	inputMap *obstacles,
	newObstacle Coordinate,
) *map[Coordinate]map[Direction]bool {

	for {
		if inputMap.isOutOfBounds(pos) {
			return nil
		}

		// if it's an obstacle, back up, turn right and keep going
		pos, direction = inputMap.handleObstacle(pos, direction)

		// it it's a new obstacle, back up, turn right and keep going
		if pos.equals(newObstacle) {
			pos = moveBackward(pos, direction)
			direction = turnRight(direction)
		}

		// determine if we've already searched this route
		inner, searchedHere := visited[pos]
		if searchedHere {
			if inner[direction] {
				return &visited
			}
		} else {
			inner = make(map[Direction]bool)
			visited[pos] = inner
		}
		// mark that we've been here facing this direction
		visited[pos][direction] = true

		// carry on
		pos = moveForward(pos, direction)
	}
}
