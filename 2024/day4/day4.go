package day4

import (
	"fmt"
	"os"
	"strings"
)

func Solve() error {
	content, err := os.ReadFile("day4/input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return err
	}

	// split the content into lines and create a 2D array
	lines := strings.Split(string(content), "\n")
	grid := make([][]rune, len(lines))

	// convert each line into a slice of runes
	for i, line := range lines {
		grid[i] = []rune(line)
	}

	fmt.Printf("Part 1: %d\n", part1(grid))
	fmt.Printf("Part 2: %d\n", part2(grid))
	return nil
}

func extend(x, y, dx, dy, length int) [][2]int {
	coords := make([][2]int, length)
	for i := 1; i <= length; i++ {
		coords[i-1] = [2]int{y + dy*i, x + dx*i}
	}
	return coords
}

func part1(grid [][]rune) int {
	// loop through each row and column
	mas := []rune{'M', 'A', 'S'}
	sum := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			if grid[y][x] == 'X' {
				// check north
				if checkWord(mas, extend(x, y, 0, -1, 3), grid) {
					sum += 1
				}
				// check north-east
				if checkWord(mas, extend(x, y, 1, -1, 3), grid) {
					sum += 1
				}
				// check east
				if checkWord(mas, extend(x, y, 1, 0, 3), grid) {
					sum += 1
				}
				// check south-east
				if checkWord(mas, extend(x, y, 1, 1, 3), grid) {
					sum += 1
				}
				// check south
				if checkWord(mas, extend(x, y, 0, 1, 3), grid) {
					sum += 1
				}
				// check south-west
				if checkWord(mas, extend(x, y, -1, 1, 3), grid) {
					sum += 1
				}
				// check west
				if checkWord(mas, extend(x, y, -1, 0, 3), grid) {
					sum += 1
				}
				// check north-west
				if checkWord(mas, extend(x, y, -1, -1, 3), grid) {
					sum += 1
				}
			}
		}
	}

	return sum
}

func part2(grid [][]rune) int {
	sum := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			if grid[y][x] == 'A' {
				slashMS := checkWord([]rune{'M', 'S'}, [][2]int{{y - 1, x + 1}, {y + 1, x - 1}}, grid)
				slashSM := checkWord([]rune{'S', 'M'}, [][2]int{{y - 1, x + 1}, {y + 1, x - 1}}, grid)
				backslashMS := checkWord([]rune{'M', 'S'}, [][2]int{{y - 1, x - 1}, {y + 1, x + 1}}, grid)
				backslashSM := checkWord([]rune{'S', 'M'}, [][2]int{{y - 1, x - 1}, {y + 1, x + 1}}, grid)

				if (slashMS || slashSM) && (backslashMS || backslashSM) {
					sum += 1
				}
			}
		}
	}

	return sum
}

// checkWord checks if a sequence of runes exists at the given coordinates in the grid
func checkWord(runes []rune, coords [][2]int, grid [][]rune) bool {
	if len(runes) != len(coords) {
		return false
	}

	for i := 0; i < len(runes); i++ {
		y := coords[i][0]
		x := coords[i][1]

		// Check row bounds first
		if y < 0 || y >= len(grid) {
			return false
		}
		// Then check column bounds
		if x < 0 || x >= len(grid[y]) {
			return false
		}

		// Check if rune matches
		if grid[y][x] != runes[i] {
			return false
		}
	}

	return true
}
