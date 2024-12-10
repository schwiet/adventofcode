package day10

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type trailmap struct {
	grid [][]int
}

func Solve() error {
	grid, trailheads, err := readInput()
	if err != nil {
		return err
	}

	trails := &trailmap{grid: grid}

	score := make(map[Coordinate]int)
	for _, trailhead := range trailheads {
		visited := make(map[Coordinate]bool)
		// set final flag to true to only count distinct trails
		s := calculateScore(trails, trailhead, visited, true)
		score[trailhead] = s
	}

	sum := 0
	for c, s := range score {
		sum += s
		fmt.Printf("c: %v, s: %d\n", c, s)
	}

	fmt.Printf("sum: %d\n", sum)

	return nil
}

type Coordinate struct {
	x int
	y int
}

var directions = []Coordinate{
	{x: 0, y: 1},
	{x: 1, y: 0},
	{x: 0, y: -1},
	{x: -1, y: 0},
}

func calculateScore(
	trails *trailmap,
	trailhead Coordinate,
	visited map[Coordinate]bool,
	distinct bool,
) int {
	if visited[trailhead] && distinct {
		return 0
	}
	visited[trailhead] = true

	height := trails.grid[trailhead.y][trailhead.x]

	branchScore := 0
	for _, direction := range directions {
		next := Coordinate{x: trailhead.x + direction.x, y: trailhead.y + direction.y}
		if next.x < 0 || next.x >= len(trails.grid[0]) || next.y < 0 || next.y >= len(trails.grid) {
			continue
		}
		if visited[next] && distinct {
			continue
		}
		nextHeight := trails.grid[next.y][next.x]
		if nextHeight == height+1 {
			if nextHeight == 9 {
				visited[next] = true
				branchScore += 1
				// fmt.Printf("reached end of trail %v\n", next)
			} else {
				// fmt.Printf("%v:%d,", next, nextHeight)
				branchScore += calculateScore(trails, next, visited, distinct)

			}
		}
	}

	return branchScore
}

func readInput() ([][]int, []Coordinate, error) {
	file, err := os.Open("day10/input.txt")
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()

	var grid [][]int
	var zeros []Coordinate
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		row := make([]int, len(line))
		for i, char := range line {
			num, err := strconv.Atoi(string(char))
			if err != nil {
				return nil, nil, err
			}
			row[i] = num
			if num == 0 {
				zeros = append(zeros, Coordinate{x: i, y: len(grid)})
			}
		}
		grid = append(grid, row)
	}

	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}

	return grid, zeros, nil
}
