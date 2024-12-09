package day8

import (
	"fmt"
	"os"
	"strings"
)

type Coordinate struct {
	x int
	y int
}

func Solve() error {
	// data, err := os.ReadFile("day8/example.txt")
	data, err := os.ReadFile("day8/input.txt")
	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	symbols := make(map[string][]Coordinate)

	for y, line := range lines {
		for x, char := range line {
			if char != '.' {
				coord := Coordinate{x: x, y: y}
				key := string(char)
				symbols[key] = append(symbols[key], coord)
			}
		}
	}

	// antinodesCoordinates := calculateAntinodes(symbols, len(lines), len(lines[0]))
	antinodesCoordinates := calculateHarmonics(symbols, len(lines), len(lines[0]))
	fmt.Printf("Antinodes: %d\n", len(antinodesCoordinates))

	// create a copy of the lines to modify
	if false {
		modifiedLines := make([]string, len(lines))
		copy(modifiedLines, lines)

		// for each antinode coordinate
		for antinode := range antinodesCoordinates {
			// convert line to rune slice so we can modify individual characters
			lineRunes := []rune(modifiedLines[antinode.y])

			// if current character is a '.', replace with '#'
			if lineRunes[antinode.x] == '.' {
				lineRunes[antinode.x] = '#'
			}

			// convert back to string
			modifiedLines[antinode.y] = string(lineRunes)
		}

		// Print modified lines
		fmt.Println("\nGrid with antinodes marked:")
		for _, line := range modifiedLines {
			fmt.Println(line)
		}
	}
	return nil
}

func calculateAntinodes(symbols map[string][]Coordinate, rows, cols int) map[Coordinate]bool {
	antinodesCoordinates := make(map[Coordinate]bool)
	// for each symbol type
	for _, coords := range symbols {
		// for each coordinate of this symbol
		for _, coord1 := range coords {
			// compare with all other coordinates of same symbol
			for _, coord2 := range coords {
				// skip if coordinates are the same
				if coord1.x == coord2.x && coord1.y == coord2.y {
					continue
				}
				// calculate x and y distances
				xDist := coord2.x - coord1.x
				yDist := coord2.y - coord1.y
				// check if the third point would be in bounds
				thirdPoint := Coordinate{
					x: coord2.x + xDist,
					y: coord2.y + yDist,
				}
				if thirdPoint.y >= 0 && thirdPoint.y < rows &&
					thirdPoint.x >= 0 && thirdPoint.x < cols {
					antinodesCoordinates[thirdPoint] = true
				}
			}
		}
	}
	return antinodesCoordinates
}

func calculateHarmonics(symbols map[string][]Coordinate, rows, cols int) map[Coordinate]bool {
	antinodesCoordinates := make(map[Coordinate]bool)
	// for each symbol type
	for _, coords := range symbols {
		// for each coordinate of this symbol
		for _, coord1 := range coords {
			// compare with all other coordinates of same symbol
			for _, coord2 := range coords {
				// skip if coordinates are the same
				if coord1.x == coord2.x && coord1.y == coord2.y {
					continue
				}
				// calculate x and y distances
				xDist := coord2.x - coord1.x
				yDist := coord2.y - coord1.y
				// handle straight lines
				if xDist == 0 {
					if yDist > 0 {
						yDist = 1
					} else if yDist < 0 {
						yDist = -1
					}
				}
				if yDist == 0 {
					if xDist > 0 {
						xDist = 1
					} else if xDist < 0 {
						xDist = -1
					}
				}

				// find harmonic of delta, if any are even
				for xDist%2 == 0 && yDist%2 == 0 {
					xDist /= 2
					yDist /= 2
				}
				thirdPoint := Coordinate{
					x: coord1.x,
					y: coord1.y,
				}
				// from the current antenna, to the end of the grid, count antinode
				// NOTE: this could be optimized by checking if the harmonic is already
				// in the map
				for thirdPoint.y >= 0 && thirdPoint.y < rows &&
					thirdPoint.x >= 0 && thirdPoint.x < cols {
					antinodesCoordinates[thirdPoint] = true
					thirdPoint.x += xDist
					thirdPoint.y += yDist
				}
			}
		}
	}
	return antinodesCoordinates
}
