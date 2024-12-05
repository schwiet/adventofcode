package day5

import (
	"aoc-2024/utils"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func Solve() error {
	lines, err := utils.ReadLines("day5/input.txt")
	if err != nil {
		return err
	}

	pattern := regexp.MustCompile(`^\d+\|\d+$`)
	orderRules := make(map[string][]string)
	updates := [][]string{}
	for _, line := range lines {
		// check if line matches pattern number|number
		if pattern.MatchString(line) {
			parts := strings.Split(line, "|")
			key := parts[0]
			value := parts[1]
			orderRules[key] = append(orderRules[key], value)
		} else if strings.Contains(line, ",") {
			// split comma-separated numbers into slice
			numbers := strings.Split(line, ",")
			updates = append(updates, numbers)
		}
	}

	middle_element_sum := 0
	corrected_middle_element_sum := 0
	for _, update := range updates {
		var corrected_update []string
		any_error := false
		fmt.Printf("Checking update: %v\n", update)
		for i, element := range update {
			swap := -1
			rule_visited := make(map[string]bool)
			swap = breaksRule(corrected_update, orderRules, element, element, rule_visited)

			if swap >= 0 {
				fmt.Printf("Found error at: %s\n", element)
				result := make([]string, i+1)
				copy(result, corrected_update[:swap])
				result[swap] = element
				copy(result[swap+1:], corrected_update[swap:])
				// fmt.Printf("Broke Rule: %s <-> %s => %v\n", rule, element, result)
				corrected_update = result
				any_error = true
			} else {
				corrected_update = append(corrected_update, element)
			}
		}

		if !any_error {
			mid_number, err := strconv.Atoi(update[len(update)/2])
			if err != nil {
				return err
			}
			middle_element_sum += mid_number
		} else {
			fmt.Printf("Corrected update: %v\n", corrected_update)
			mid_number, err := strconv.Atoi(corrected_update[len(corrected_update)/2])
			if err != nil {
				return err
			}
			corrected_middle_element_sum += mid_number
		}
	}
	fmt.Printf("Middle element sum: %d\n", middle_element_sum)
	fmt.Printf("Corrected middle element sum: %d\n", corrected_middle_element_sum)
	return nil
}

func breaksRule(
	updated []string,
	orderRules map[string][]string,
	ruleKey string,
	element string,
	visited map[string]bool,
) int {
	if visited[element] {
		return -1
	}
	visited[element] = true

	swap := -1
	for _, rule := range orderRules[ruleKey] {
		if i := isIn(rule, updated); i >= 0 {
			if swap == -1 || (i >= 0 && i < swap) {
				swap = i
			}
		}

		lowestOther := breaksRule(updated, orderRules, rule, element, visited)
		if swap == -1 || (lowestOther >= 0 && lowestOther < swap) {
			swap = lowestOther
		}
	}

	return swap
}

func isIn(element string, updated []string) int {
	for i, updated_element := range updated {
		if updated_element == element {
			return i
		}
	}
	return -1
}
