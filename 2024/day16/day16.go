package day16

import (
	"fmt"
	"os"
	"strings"
)

type mem map[coord]map[coord]int

type path map[coord]bool

func (p path) add(c coord) {
	p[c] = true
}

func (p path) remove(c coord) {
	delete(p, c)
}

type coord struct {
	x, y int
}

func (c coord) turnLeft() coord {
	return coord{c.y, c.x * -1}
}

func (c coord) turnRight() coord {
	return coord{x: c.y * -1, y: c.x}
}

func (c coord) turnAround() coord {
	return coord{c.x * -1, c.y * -1}
}

func (c coord) move(dir coord) coord {
	return coord{c.x + dir.x, c.y + dir.y}
}

func readMaze(filename string) (map[coord]bool, coord, coord, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, coord{}, coord{}, err
	}

	maze := make(map[coord]bool)
	var start, end coord

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	for y, line := range lines {
		for x, char := range line {
			switch char {
			case '.':
				maze[coord{x, y}] = true
			case 'S':
				start = coord{x, y}
				maze[coord{x, y}] = true
			case 'E':
				end = coord{x, y}
				maze[coord{x, y}] = true
			}
		}
	}

	return maze, start, end, nil
}

func searchBranch(maze map[coord]bool, pos, end, dir coord, explored mem, path path) (int, bool) {
	// if we're not in the maze, fail
	if !maze[pos] {
		return 0, false
	}
	// if we've reached the end, return the score
	if pos.x == end.x && pos.y == end.y {
		return 0, true
	}

	// loop detection: if this is in the current path, bail
	if path[pos] {
		return 0, false
	}
	// add this position to the path
	path.add(pos)
	defer path.remove(pos)

	// if we've already explored this position
	minScore, beenHere := explored[pos]
	if beenHere {
		score, goneThisWay := minScore[dir]
		if goneThisWay {
			if score > 0 {
				// we've already found the best path to the end going this way
				return score, true
			}
			// found a place we've been before going the same way, but was a dead end
			return 0, false
		}
	} else {
		explored[pos] = make(map[coord]int)
	}

	// unexplored location and direction with no path found from here, yet
	found := false

	// try going straight
	sscore, sfound := searchBranch(maze, pos.move(dir), end, dir, explored, path)
	if sfound {
		explored[pos][dir] = sscore + 1
		found = true
	}

	// try going left
	left := dir.turnLeft()
	lscore, lfound := searchBranch(maze, pos.move(left), end, left, explored, path)
	if lfound {
		if !found || lscore+1001 < explored[pos][dir] {
			explored[pos][dir] = lscore + 1001
		}
		found = true
	}

	// try going right
	right := dir.turnRight()
	rscore, rfound := searchBranch(maze, pos.move(right), end, right, explored, path)
	if rfound {
		if !found || rscore+1001 < explored[pos][dir] {
			explored[pos][dir] = rscore + 1001
		}
		found = true
	}

	if !found {
		// mark this position as leading to a dead end; neither straight, left, nor
		// right found a path to the end
		explored[pos][dir] = -1
		return 0, false
	}

	return explored[pos][dir], true
}

func Solve() error {
	maze, start, end, err := readMaze("day16/input.txt")
	if err != nil {
		return err
	}

	dir := coord{1, 0}
	explored := make(mem)
	path := make(path)

	score, found := searchBranch(maze, start, end, dir, explored, path)
	if !found {
		return fmt.Errorf("no path found")
	}

	fmt.Println(score)

	return nil
}
