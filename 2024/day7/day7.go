package day7

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type calibration struct {
	output float64
	input  []float64
}

func merge(a, b float64) float64 {
	aStr := strconv.FormatFloat(a, 'f', 0, 64)
	bStr := strconv.FormatFloat(b, 'f', 0, 64)
	merged, _ := strconv.ParseFloat(aStr+bStr, 64)
	return merged
}

func isValid(output float64, current float64, input []float64) bool {
	if len(input) == 0 {
		return output == current
	}

	return (isValid(output, current+input[0], input[1:]) ||
		// this line can be commented out to answer part 1
		isValid(output, merge(current, input[0]), input[1:]) ||
		isValid(output, current*input[0], input[1:]))
}

func Solve() error {
	calibrations, err := readCalibrations("day7/input.txt")
	if err != nil {
		return err
	}

	var sum float64
	for _, calibration := range calibrations {
		if isValid(calibration.output, 0, calibration.input) {
			sum += calibration.output
		}
	}

	fmt.Printf("sum: %d\n", int(sum))

	return nil
}

func readCalibrations(filename string) ([]calibration, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var calibrations []calibration
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, ":")

		output, err := strconv.ParseFloat(strings.TrimSpace(parts[0]), 64)
		if err != nil {
			return nil, err
		}

		var inputs []float64
		for _, numStr := range strings.Fields(strings.TrimSpace(parts[1])) {
			num, err := strconv.ParseFloat(numStr, 64)
			if err != nil {
				return nil, err
			}
			inputs = append(inputs, num)
		}

		calibrations = append(calibrations, calibration{
			output: output,
			input:  inputs,
		})
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return calibrations, nil
}
