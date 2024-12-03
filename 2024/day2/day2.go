package day2

import (
	"fmt"
	"strconv"
	"strings"

	"aoc-2024/utils"
)

func ParseInput() ([][]int, error) {
	lines, err := utils.ReadLines("day2/input.txt")
	if err != nil {
		return nil, err
	}
	var result [][]int

	for _, line := range lines {
		// Split the line into individual number strings
		numStrs := strings.Fields(line)

		// Convert each string to integer
		nums := make([]int, 0, len(numStrs))
		for _, numStr := range numStrs {
			num, err := strconv.Atoi(numStr)
			if err != nil {
				continue // Skip non-numeric values
			}
			nums = append(nums, num)
		}

		result = append(result, nums)
	}

	return result, nil
}

func Solve(tolerance string) error {
	input, err := ParseInput()
	if err != nil {
		return err
	}

	toleranceInt, err := strconv.Atoi(tolerance)
	if err != nil {
		return err
	}

	validCount := 0
	for _, sequence := range input {
		if len(sequence) < 2 {
			continue
		}

		// Determine if sequence is increasing or decreasing based on first two numbers
		isIncreasing := sequence[1] > sequence[0]
		invalidCount := 0

		for j := 1; j < len(sequence); j++ {
			diff := sequence[j] - sequence[j-1]

			if isIncreasing {
				if diff <= 0 || diff > 3 {
					invalidCount += 1
				}
			} else {
				if diff >= 0 || diff < -3 {
					invalidCount += 1
				}
			}

			if invalidCount > toleranceInt {
				break
			}
		}

		if invalidCount <= toleranceInt {
			validCount += 1
		}
	}

	fmt.Printf("Valid count: %d\n", validCount)
	return nil
}
