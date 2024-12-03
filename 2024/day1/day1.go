package day1

import (
	"aoc-2024/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func part1() error {
	lines, err := utils.ReadLines("day1/input.txt")
	if err != nil {
		return err
	}

	fmt.Printf("• Read %d lines\n", len(lines))

	note := [][]int{[]int{}, []int{}}
	for _, line := range lines {
		words := strings.Fields(line)

		for i, word := range words {

			if i < 2 { // just a guardrail
				num, err := strconv.Atoi(word)
				if err != nil {
					return fmt.Errorf("failed to convert '%s' to integer: %w", word, err)
				}
				// append number to column
				note[i] = append(note[i], num)
			}
		}
	}

	col1 := note[0]
	col2 := note[1]
	fmt.Printf("• Left: %d, Right: %d\n", len(col1), len(col2))

	sort.Ints(col1)
	sort.Ints(col2)

	diffSum := 0

	for i := 0; i < len(lines); i += 1 {
		diffSum += abs(col1[i] - col2[i])
	}

	fmt.Printf("diff sum: %d\n", diffSum)
	return nil
}

func Solve() error {
	return part1()
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
