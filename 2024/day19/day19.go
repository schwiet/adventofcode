package day19

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readInput(filename string) ([]string, []string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// Read first line into available
	scanner.Scan()
	available := strings.Split(strings.TrimSpace(scanner.Text()), ", ")

	// Skip empty line
	scanner.Scan()

	// Read remaining lines into targets
	var targets []string
	for scanner.Scan() {
		targets = append(targets, strings.TrimSpace(scanner.Text()))
	}

	return available, targets, nil
}

func Solve() error {
	available, targets, err := readInput("day19/input.txt")
	if err != nil {
		return err
	}

	count := 0
	memo := make(memo)
	for _, target := range targets {
		if canAssemble(target, available, memo) {
			count++
		}
	}

	fmt.Println("Possible targets:", count)
	return nil
}

// memo stores whether a target string can be assembled from available pieces
type memo map[string]bool

func canAssemble(target string, available []string, memo memo) bool {
	// Check memo first
	if result, exists := memo[target]; exists {
		return result
	}

	// Base case - empty target can always be assembled
	if len(target) == 0 {
		memo[target] = true
		return true
	}

	// Try each available string as a prefix
	for _, prefix := range available {
		if len(prefix) <= len(target) && strings.HasPrefix(target, prefix) {
			// Recursively check if remaining target can be assembled
			if canAssemble(target[len(prefix):], available, memo) {
				memo[target] = true
				return true
			}
		}
	}

	memo[target] = false
	return false
}
