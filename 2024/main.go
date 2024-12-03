package main

import (
	"aoc-2024/day1"
	"aoc-2024/day2"
	"aoc-2024/day3"
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
	case "2":
		err = day2.Solve(os.Args[2])
	case "3":
		err = day3.Solve()
	// Add cases for additional days here
	default:
		err = fmt.Errorf(`Day %s is not implemented yet.`, day)
	}

	if err != nil {
		fmt.Printf("Fail: %v\n", err)
	}
}