package day9

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Solve() error {
	type block struct {
		data  []int64
		free  int64
		index int64
	}

	data, err := os.ReadFile("day9/input.txt")
	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	var blocks []block

	var index int64 = 0
	for _, line := range lines {
		// fmt.Println(line)
		// Split line into pairs of digits since values are not separated
		for i := 0; i < len(line); i += 2 {
			size, err := strconv.ParseInt(line[i:i+1], 10, 64)
			if err != nil {
				return err
			}
			var free int64
			if i+1 >= len(line) {
				free = 0
			} else {
				free, err = strconv.ParseInt(line[i+1:i+2], 10, 64)
				if err != nil {
					return err
				}
			}

			// Create block with data slice filled with index value
			b := block{
				data:  make([]int64, size),
				free:  free,
				index: index,
			}
			for i := range b.data {
				b.data[i] = index
			}

			blocks = append(blocks, b)
			index++
		}
		// fmt.Printf("Blocks: %v\n", blocks)
	}
	freeIndex := 0
	for i := len(blocks) - 1; i >= 0; i-- {
		if i <= freeIndex {
			break
		}
		blockToMove := blocks[i]
		for len(blockToMove.data) > 0 && i > freeIndex {
			if blocks[freeIndex].free > 0 {
				blocks[freeIndex].data = append(blocks[freeIndex].data, blockToMove.data[len(blockToMove.data)-1])
				blockToMove.data = blockToMove.data[:len(blockToMove.data)-1]
				blocks[freeIndex].free -= 1
				blockToMove.free += 1
			} else {
				freeIndex += 1
			}
		}
		blocks[i] = blockToMove
	}

	// fmt.Printf("Blocks After: %v\n", blocks)
	index = int64(0)
	checksum := int64(0)
	for _, block := range blocks {
		if len(block.data) > 0 {
			for _, value := range block.data {
				checksum += value * index
				index += 1
			}
		}
	}
	fmt.Println(checksum)
	return nil
}
