package day1

import (
	"aoc-2024/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func Solve() error {
	lines, err := utils.ReadLines("day1/input.txt")
	if err != nil {
		return err
	}

	fmt.Printf("• Read %d lines\n", len(lines))

	note := [][]int{{}, {}}
	var leftList []string
	rightCounts := map[string]int{}
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

				if i == 0 {
					leftList = append(leftList, word)
				} else if i == 1 {
					rightCounts[word] = rightCounts[word] + 1
				}
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

	sim := 0
	for _, row := range leftList {
		rc := rightCounts[row]
		ln, _ := strconv.Atoi(row)
		sim += rc * ln
	}
	fmt.Printf("similarity: %d\n", sim)
	return nil
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
