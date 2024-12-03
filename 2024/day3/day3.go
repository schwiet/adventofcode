package day3

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func part2(content []byte) error {
	// Split content by "do()"
	parts := strings.Split(string(content), "do()")

	// Print each part for debugging
	sum := 0
	for _, part := range parts {
		if idx := strings.Index(part, "don't()"); idx >= 0 {
			part = part[:idx]
		}
		// fmt.Printf("Part %d: %s\n", i, part)
		count, err := part1([]byte(part))
		if err != nil {
			return err
		}
		sum += count
	}

	fmt.Printf("Sum: %d\n", sum)
	return nil
}

func part1(content []byte) (int, error) {
	// Match mul(X,Y) only if it's not preceded by a don't() without an intervening do()
	re := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)

	matches := re.FindAllStringSubmatch(string(content), -1)

	sum := 0
	for _, match := range matches {
		// match[1] is now the first number (X)
		// match[2] is now the second number (Y)
		x, _ := strconv.Atoi(match[1])
		y, _ := strconv.Atoi(match[2])
		// fmt.Printf("Found multiplication: %d * %d\n", x, y)
		sum += x * y
	}

	return sum, nil
}

func Solve() error {
	content, err := os.ReadFile("day3/input.txt")
	if err != nil {
		return err
	}

	sum, _ := part1(content)
	fmt.Printf("Part 1: %d\n", sum)
	return part2(content)
}
