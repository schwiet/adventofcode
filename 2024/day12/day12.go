package day12

import (
	"fmt"
	"os"
	"strings"
)

type Direction int

const (
	North Direction = iota
	East
	South
	West
)

type coordinate struct {
	x, y int
}

type fencePanel struct {
	coordinate
	orientation Direction
}

func (f fencePanel) isHorizontal() bool {
	return f.orientation == North || f.orientation == South
}

func (f fencePanel) leftOf() fencePanel {
	return fencePanel{coordinate{f.x - 1, f.y}, f.orientation}
}

func (f fencePanel) rightOf() fencePanel {
	return fencePanel{coordinate{f.x + 1, f.y}, f.orientation}
}

func (f fencePanel) above() fencePanel {
	return fencePanel{coordinate{f.x, f.y - 1}, f.orientation}
}

func (f fencePanel) below() fencePanel {
	return fencePanel{coordinate{f.x, f.y + 1}, f.orientation}
}

type gardenRegion struct {
	letter    rune
	fence     map[fencePanel]bool
	area      map[coordinate]bool
	perimeter map[coordinate]bool
	sides     int
}

func Solve() error {
	data, err := os.ReadFile("day12/input.txt")
	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	grid := make([][]rune, 0, len(lines))

	for _, line := range lines {
		if line == "" {
			continue
		}
		grid = append(grid, []rune(line))
	}

	grouped := make(map[coordinate]bool, len(grid)*len(grid[0]))
	var regions []gardenRegion
	for y, row := range grid {
		for x, cell := range row {
			if grouped[coordinate{x, y}] {
				continue
			}

			region := make(map[coordinate]bool)
			identifyRegion(grid, cell, coordinate{x, y}, region, grouped)
			regions = append(regions, gardenRegion{
				letter:    cell,
				fence:     make(map[fencePanel]bool),
				area:      region,
				sides:     0,
				perimeter: nil,
			})
		}
	}

	cost := 0
	costBulk := 0
	for _, region := range regions {
		for char := range region.area {
			above := coordinate{char.x, char.y - 1}
			below := coordinate{char.x, char.y + 1}
			left := coordinate{char.x - 1, char.y}
			right := coordinate{char.x + 1, char.y}
			if !region.area[above] {
				region.fence[fencePanel{char, North}] = true
			}
			if !region.area[below] {
				region.fence[fencePanel{char, South}] = true
			}
			if !region.area[left] {
				region.fence[fencePanel{char, West}] = true
			}
			if !region.area[right] {
				region.fence[fencePanel{char, East}] = true
			}
		}
		// fmt.Printf("%c: %d, %d\n", region.letter, len(region.area), len(region.fence))

		grouped := make(map[fencePanel]bool)
		for panel := range region.fence {
			if identifySides(region.fence, panel, grouped) {
				region.sides++
			}
		}

		cost += len(region.area) * len(region.fence)
		costBulk += len(region.area) * region.sides
	}
	fmt.Printf("Original cost: %d\n", cost)
	fmt.Printf("Bulk cost: %d\n", costBulk)
	return nil
}

func identifySides(fence map[fencePanel]bool, fencePanel fencePanel, grouped map[fencePanel]bool) bool {
	// avoid double counting
	if grouped[fencePanel] {
		return false
	}

	// mark the current fence panel as counted
	grouped[fencePanel] = true

	// identify and mark all adjacent fence panels
	if fencePanel.isHorizontal() {
		// go left until no more fence
		left := fencePanel.leftOf()
		for found := fence[left]; found; {
			grouped[left] = true
			left = left.leftOf()
			found = fence[left]
		}
		// go right until no more fence
		right := fencePanel.rightOf()
		for found := fence[right]; found; {
			grouped[right] = true
			right = right.rightOf()
			found = fence[right]
		}
	} else {
		// go up until no more fence
		above := fencePanel.above()
		for found := fence[above]; found; {
			grouped[above] = true
			above = above.above()
			found = fence[above]
		}
		// go down until no more fence
		below := fencePanel.below()
		for found := fence[below]; found; {
			grouped[below] = true
			below = below.below()
			found = fence[below]
		}
	}

	return true
}

func identifyRegion(
	garden [][]rune,
	letter rune,
	c coordinate,
	region map[coordinate]bool,
	grouped map[coordinate]bool,
) {
	if grouped[c] {
		return
	}

	oob := c.x < 0 || c.x >= len(garden[0]) || c.y < 0 || c.y >= len(garden)
	if oob {
		return
	}

	if garden[c.y][c.x] != letter {
		return
	}

	region[c] = true
	grouped[c] = true

	identifyRegion(garden, letter, coordinate{c.x + 1, c.y}, region, grouped)
	identifyRegion(garden, letter, coordinate{c.x - 1, c.y}, region, grouped)
	identifyRegion(garden, letter, coordinate{c.x, c.y + 1}, region, grouped)
	identifyRegion(garden, letter, coordinate{c.x, c.y - 1}, region, grouped)
}
