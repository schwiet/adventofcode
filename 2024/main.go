package main

import (
	"aoc-2024/day1"
	"aoc-2024/day10"
	"aoc-2024/day11"
	"aoc-2024/day12"
	"aoc-2024/day13"
	"aoc-2024/day2"
	"aoc-2024/day3"
	"aoc-2024/day4"
	"aoc-2024/day5"
	"aoc-2024/day6"
	"aoc-2024/day7"
	"aoc-2024/day8"
	"aoc-2024/day9"
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
	case "4":
		err = day4.Solve()
	case "5":
		err = day5.Solve()
	case "6":
		err = day6.Solve()
	case "7":
		err = day7.Solve()
	case "8":
		err = day8.Solve()
	case "9":
		err = day9.Solve()
	case "10":
		err = day10.Solve()
	case "11":
		err = day11.Solve()
	case "12":
		err = day12.Solve()
	case "13":
		err = day13.Solve()
	// Add cases for additional days here
	default:
		err = fmt.Errorf(`Day %s is not implemented yet.`, day)
	}

	if err != nil {
		fmt.Printf("Fail: %v\n", err)
	}
}
