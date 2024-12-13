package day13

import (
	"bufio"
	"fmt"
	"os"

	"aoc-2024/utils"
)

func Solve() error {
	file, err := os.Open("day13/input.txt")
	if err != nil {
		return fmt.Errorf("failed to open file: %v", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	cost := 0
	// p1
	offset := 0
	// p2
	// offset := 10000000000000
	gameSet := -1
	for scanner.Scan() {
		gameSet += 1
		// read button A line
		buttonALine := scanner.Text()
		var aX, aY int
		fmt.Sscanf(buttonALine, "Button A: X+%d, Y+%d", &aX, &aY)

		// read button B line
		scanner.Scan()
		buttonBLine := scanner.Text()
		var bX, bY int
		fmt.Sscanf(buttonBLine, "Button B: X+%d, Y+%d", &bX, &bY)

		// read prize line
		scanner.Scan()
		prizeLine := scanner.Text()
		var pX, pY int
		fmt.Sscanf(prizeLine, "Prize: X=%d, Y=%d", &pX, &pY)

		// skip the blank line between game sets
		scanner.Scan()

		// solve for i,j,k,l where:
		// i * aX + j * bX = pX
		// k * aY + l * bY = pY
		// favor solutions with highest j and l values
		exists, x, y, hasMax := utils.SolveTwoEquations(aX, bX, pX+offset, aY, bY, pY+offset)
		if !exists {
			fmt.Printf("no solution found for game set: %+v\n", gameSet)
			continue
		}
		if hasMax {
			cost += x*3 + y*1
		} else {
			fmt.Printf("no max solution found for game set: %+v\n", gameSet)
		}
	}

	if err := scanner.Err(); err != nil {
		return fmt.Errorf("error reading file: %v", err)
	}

	fmt.Println(cost)
	return nil
}
