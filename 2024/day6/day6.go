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

	outOfBounds := false
	direction := Position{x: 0, y: -1}
	visited := make(map[Position]map[Position]bool)
	visited[pos] = map[Position]bool{direction: true}
	loopCount := 0
	for !outOfBounds {
		_, beenHere := visited[pos]
		if !beenHere {
			visited[pos] = map[Position]bool{direction: true}
		} else {
			// track that we've been here facing this direction
			visited[pos][direction] = true
		}

		nextX := pos.x + direction.x
		nextY := pos.y + direction.y
		// if it's out of bounds, break
		if nextX < 0 || nextX >= len(grid[0]) || nextY < 0 || nextY >= len(grid) {
			fmt.Printf("out of bounds: %d, %d\n", nextX, nextY)
			outOfBounds = true
		}

		// if it's not an obstacle, move to it
		obstacle := outOfBounds || grid[nextY][nextX]
		right_turn := Position{x: -direction.y, y: direction.x}
		if !obstacle {
			pullBack := Position{x: pos.x - right_turn.x, y: pos.y - right_turn.y}
			// if we've been here facing the other direction, we've looped
			if willLoopFromHere(visited, nil, pullBack, right_turn, grid) {
				loopCount += 1
			}
			// carry on
			pos = Position{x: nextX, y: nextY}
		} else {
			// turn right
			direction = right_turn
		}
	}
	fmt.Printf("visited: %v\n", len(visited))
	fmt.Printf("loop count: %d\n", loopCount)
	return nil
}

func willLoopFromHere(
	visited map[Position]map[Position]bool,
	searchPositions map[Position]map[Position]bool,
	pos Position,
	direction Position,
	grid [][]bool,
) bool {
	for {
		dirs, beenHere := visited[pos]
		if beenHere {
			if dirs[direction] {
				return true
			}
		}
		nextX := pos.x + direction.x
		nextY := pos.y + direction.y
		// copy the search positions
		innerLoop := make(map[Position]map[Position]bool)
		for k, v := range searchPositions {
			innerLoop[k] = v
		}

		if nextX < 0 || nextX >= len(grid[0]) || nextY < 0 || nextY >= len(grid) {
			return false
		}

		// determine if we've already searched this route
		inner, searchedHere := innerLoop[pos]
		if searchedHere && inner[direction] {
			return false
		} else if searchedHere {
			inner[direction] = true
		} else {
			inner = make(map[Position]bool)
			inner[direction] = true
			innerLoop[pos] = inner
		}

		// if it's an obstacle, search right from here
		obstacle := grid[nextY][nextX]
		if obstacle {
			right_turn := Position{x: -direction.y, y: direction.x}
			pullBack := Position{x: pos.x - right_turn.x, y: pos.y - right_turn.y}
			return willLoopFromHere(visited, innerLoop, pullBack, right_turn, grid)
		}

		// carry on
		pos = Position{x: nextX, y: nextY}
	}
}

type Position struct {
	x int
	y int
}
