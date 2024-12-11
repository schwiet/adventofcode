package day11

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Solve() error {
	file, err := os.Open("day11/input.txt")
	if err != nil {
		return err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	var numbers [][]string
	for _, line := range lines {
		numbers = append(numbers, strings.Fields(line))
	}

	stones := numbers[0]

	stoneSum := 0
	for _, stone := range stones {
		section := []string{stone}

		known := make(map[string]int)
		for i := 0; i < 25; i += 1 {
			section = blink(section)
		}
		known[stone] = len(section)
		for _, stone = range section {
			section = []string{stone}
			for i := 0; i < 25; i += 1 {
				section = blink(section)
			}
			known[stone] = len(section)
			for _, stone = range section {
				if expanded, ok := known[stone]; ok {
					stoneSum += expanded
					// fmt.Printf("found known: %d\n", expanded)
					continue
				}
				section = []string{stone}
				for i := 0; i < 25; i += 1 {
					section = blink(section)
				}
				known[stone] = len(section)
				fmt.Printf("finished section: %d\n", len(section))
				stoneSum += len(section)
			}
		}
		fmt.Printf("Finished Stone %s:", stone)
	}

	fmt.Println(stoneSum)

	if err := scanner.Err(); err != nil {
		return err
	}

	return nil
}

func blink(stones []string) []string {
	var newStones []string

	for _, stone := range stones {
		if stone == "0" {
			newStones = append(newStones, "1")
		} else if len(stone)%2 == 0 {
			length := len(stone)
			first := stone[:length/2]
			second := stone[length/2:]

			// Trim leading zeros
			first = strings.TrimLeft(first, "0")
			second = strings.TrimLeft(second, "0")

			// Handle empty strings (all zeros)
			if first == "" {
				first = "0"
			}
			if second == "" {
				second = "0"
			}

			newStones = append(newStones, first, second)

		} else {
			num, _ := strconv.ParseInt(stone, 10, 64)
			newStones = append(newStones, strconv.FormatInt(num*2024, 10))
		}
	}

	return newStones
}
