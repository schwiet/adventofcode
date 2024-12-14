package day14

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x, y int
}

type robot struct {
	position coord
	velocity coord
}

func Solve() error {

	file, err := os.Open("day14/input.txt")
	if err != nil {
		return err
	}
	defer file.Close()

	w, h := 101, 103
	// used in part 1, skipped for part 2
	// ticks := 100
	var robots []robot
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var r robot
		fmt.Sscanf(line, "p=%d,%d v=%d,%d",
			&r.position.x, &r.position.y,
			&r.velocity.x, &r.velocity.y)

		// used in part 1, skipped for part 2
		// newX := (r.position.x + r.velocity.x*ticks) % w
		// newY := (r.position.y + r.velocity.y*ticks) % h
		// if newX < 0 {
		// 	newX = w + newX
		// }
		// if newY < 0 {
		// 	newY = h + newY
		// }
		// r.position = coord{newX, newY}
		robots = append(robots, r)
	}

	if err := scanner.Err(); err != nil {
		return err
	}
	// count robots in each quadrant, excluding middle lines
	midX := w / 2
	midY := h / 2
	quadrants := make([]int, 4) // TL, TR, BL, BR

	for _, r := range robots {
		// skip robots on middle lines
		if r.position.x == midX || r.position.y == midY {
			continue
		}

		// determine quadrant
		if r.position.y < midY { // top half
			if r.position.x < midX {
				quadrants[0]++ // top left
			} else {
				quadrants[1]++ // top right
			}
		} else { // bottom half
			if r.position.x < midX {
				quadrants[2]++ // bottom left
			} else {
				quadrants[3]++ // bottom right
			}
		}
	}

	fmt.Printf("Quadrant counts (TL,TR,BL,BR): %v\n", quadrants)
	product := 1
	for _, count := range quadrants {
		product *= count
	}
	// need to uncomment lines above to get right answer for part 1
	fmt.Printf("Product of quadrant counts: %d\n", product)

	maxMiddle := 0
	for i := 0; i < 10000; i++ {
		maxMiddle = printGrid(robots, w, h, i, maxMiddle)
	}
	return nil
}

func printGrid(robots []robot, w, h, ticks, maxMiddle int) int {
	// create grid
	grid := make([][]int, h)
	for i := range grid {
		grid[i] = make([]int, w)
	}

	// specifically, look for concentration in the middle third of the grid
	x0 := w / 3
	x1 := 2 * (w / 3)
	y0 := h / 3
	y1 := 2 * (h / 3)
	middleCount := 0

	// mark robot positions
	maxCell := 0
	for _, r := range robots {
		newX := (r.position.x + r.velocity.x*ticks) % w
		newY := (r.position.y + r.velocity.y*ticks) % h
		if newX < 0 {
			newX = w + newX
		}
		if newY < 0 {
			newY = h + newY
		}
		grid[newY][newX]++
		if grid[newY][newX] > maxCell {
			maxCell = grid[newY][newX]
		}
		if newX > x0 && newX < x1 && newY > y0 && newY < y1 {
			middleCount++
		}
	}

	if middleCount < maxMiddle {
		return maxMiddle
	}
	// print grid
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if grid[y][x] == 0 {
				fmt.Print(".")
			} else {
				fmt.Print(grid[y][x])
			}
		}
		fmt.Println()
	}
	fmt.Printf("Tick %d\n----------------\n\n\n", ticks)

	return middleCount
}
