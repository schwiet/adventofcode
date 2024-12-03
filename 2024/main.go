package main

import (
	"aoc-2024/day1"
	"fmt"
	"os"
)

func main() {
	// Check if the correct number of arguments is provided
	if len(os.Args) < 2 {
		fmt.Println("Usage: adventofcode <day> [additional arguments]")
		return
	}

	// Extract the day argument
	day := os.Args[1]

	// Optional arguments can be captured as follows
	// args := os.Args[2:]

	var err error
	switch day {
	case "1":
		err = day1.Solve()
	// Add cases for additional days here
	default:
		err = fmt.Errorf(`Day %s is not implemented yet.`, day)
	}

	if err != nil {
		fmt.Printf("Fail: %v\n", err)
	}
}
