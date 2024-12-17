package day15

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x, y int
}

type direction struct {
	x, y int
}

func readMap(filename string) ([][]string, *coord, []direction, error) {
	var position coord
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var grid [][]string
	var directions []direction
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}

		// Check if line contains grid characters
		if line[0] == '#' || line[0] == '.' || line[0] == 'O' {
			row := make([]string, len(line))
			for i, c := range line {
				if c == '@' {
					position = coord{x: i, y: len(grid)}
					row[i] = `.`
				} else {
					row[i] = string(c)
				}
			}
			grid = append(grid, row)
		} else {
			// Line contains directions
			for _, c := range line {
				switch c {
				case '<':
					directions = append(directions, direction{x: -1, y: 0})
				case '>':
					directions = append(directions, direction{x: 1, y: 0})
				case '^':
					directions = append(directions, direction{x: 0, y: -1})
				case 'v':
					directions = append(directions, direction{x: 0, y: 1})
				}
			}
		}
	}
	return grid, &position, directions, nil
}

func moveIfPossible(grid [][]string, position coord, direction direction) bool {
	newPosition := coord{x: position.x + direction.x, y: position.y + direction.y}
	if newPosition.x < 0 || newPosition.x >= len(grid[0]) || newPosition.y < 0 || newPosition.y >= len(grid) {
		return false
	}
	if grid[newPosition.y][newPosition.x] == "#" {
		return false
	}
	if grid[newPosition.y][newPosition.x] == "O" {
		if !moveIfPossible(grid, newPosition, direction) {
			return false
		}
	}
	grid[newPosition.y][newPosition.x] = "O"
	grid[position.y][position.x] = "."
	return true
}

func Solve() error {
	grid, position, directions, err := readMap("day15/input.txt")
	if err != nil {
		return err
	}

	for _, direction := range directions {
		if moveIfPossible(grid, *position, direction) {
			position.x += direction.x
			position.y += direction.y
		}
	}
	grid[position.y][position.x] = "@"

	sum := 0
	for y, row := range grid {
		for x, cell := range row {
			if cell == "O" {
				sum += y*100 + x
			}
			fmt.Print(cell)
		}
		fmt.Println()
	}
	fmt.Println(sum)
	return nil
}
