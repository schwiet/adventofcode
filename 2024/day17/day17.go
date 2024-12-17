package day17

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func readProgram(filename string) (map[string]int, []int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()

	registers := make(map[string]int)
	var program []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "Register") {
			parts := strings.Split(line, ":")
			registerName := strings.TrimSpace(strings.TrimPrefix(parts[0], "Register "))
			value, err := strconv.Atoi(strings.TrimSpace(parts[1]))
			if err != nil {
				return nil, nil, err
			}
			registers[registerName] = value
		} else if strings.HasPrefix(line, "Program:") {
			parts := strings.Split(line, ":")
			numStrs := strings.Split(strings.TrimSpace(parts[1]), ",")
			for _, numStr := range numStrs {
				num, err := strconv.Atoi(strings.TrimSpace(numStr))
				if err != nil {
					return nil, nil, err
				}
				program = append(program, num)
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}

	return registers, program, nil
}

func Solve() error {
	registers, program, err := readProgram("day17/input.txt")
	if err != nil {
		return err
	}

	expected := strings.Trim(strings.Replace(fmt.Sprint(program), " ", ",", -1), "[]")

	fmt.Println("Looking for output:", expected)
	var result string
	var i int
	for i = 0; result != expected; i += 1 {
		registers["A"] = i
		registers["B"] = 0
		registers["C"] = 0
		if i%100000000 == 0 {
			fmt.Printf("Iteration %d: Expected %s, got %s\n", i, expected, result)
		}
		result = getOutput(registers, program, expected)
	}

	fmt.Printf("Found Output: %s after %d iterations\n", result, i)
	return nil
}

func getOutput(registers map[string]int, prog []int, expected string) string {
	pointer := 0
	var result strings.Builder
	hasOutput := false
	for pointer < len(prog) {
		instruction := prog[pointer]
		pointerUpdate, output := instructions[instruction](registers, prog[pointer+1])
		if pointerUpdate != nil {
			pointer = *pointerUpdate
		} else {
			pointer += 2
		}
		if output != "" {
			if hasOutput {
				result.WriteString(",")
			}
			result.WriteString(output)
			if expected != "" && !strings.HasPrefix(expected, result.String()) {
				break
			}
			hasOutput = true
		}
	}
	return result.String()
}

func getOperand(registers map[string]int, operand int) int {
	if operand < 4 {
		return operand
	}
	if operand == 4 {
		return registers["A"]
	}
	if operand == 5 {
		return registers["B"]
	}
	if operand == 6 {
		return registers["C"]
	}

	panic(fmt.Sprintf("invalid operand: %d", operand))
}

type instruction func(
	registers map[string]int,
	operand int,
) (
	pointerUpdate *int,
	output string,
)

func xdv(registers map[string]int, operand int, resultRegister string) (*int, string) {
	numerator := registers["A"]
	useOperand := getOperand(registers, operand)
	// raise 2 to the power of operand
	denominator := 1 << useOperand
	result := numerator / denominator
	registers[resultRegister] = result
	return nil, ""
}

func adv(registers map[string]int, operand int) (*int, string) {
	return xdv(registers, operand, "A")
}

func bdv(registers map[string]int, operand int) (*int, string) {
	return xdv(registers, operand, "B")
}

func cdv(registers map[string]int, operand int) (*int, string) {
	return xdv(registers, operand, "C")
}

func bxl(registers map[string]int, operand int) (*int, string) {
	registers["B"] = registers["B"] ^ operand
	return nil, ""
}

func bst(registers map[string]int, operand int) (*int, string) {
	useOperand := getOperand(registers, operand)
	registers["B"] = useOperand % 8
	return nil, ""
}

func jnz(registers map[string]int, operand int) (*int, string) {
	if registers["A"] != 0 {
		return &operand, ""
	}
	return nil, ""
}

func bxc(registers map[string]int, operand int) (*int, string) {
	registers["B"] = registers["B"] ^ registers["C"]
	return nil, ""
}

func out(registers map[string]int, operand int) (*int, string) {
	useOperand := getOperand(registers, operand)
	result := useOperand % 8
	return nil, fmt.Sprintf("%d", result)
}

var instructions = map[int]instruction{
	0: adv,
	1: bxl,
	2: bst,
	3: jnz,
	4: bxc,
	5: out,
	6: bdv,
	7: cdv,
}
