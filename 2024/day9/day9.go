package day9

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type block struct {
	data  []int64
	added []int64
	free  int64
	index int64
}

func Solve() error {

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

	fragmentedBlocks := fragmentBlocks(blocks)
	mergedBlocks := moveBlocks(blocks)

	// fmt.Printf("Blocks After: %v\n", mergedBlocks)
	fmt.Printf("Fragmented Checksum: %d\n", checksum(fragmentedBlocks))
	fmt.Printf("Merged Checksum: %d\n", checksum(mergedBlocks))
	return nil
}

func fragmentBlocks(blocks []block) []block {
	newBlocks := make([]block, len(blocks))
	copy(newBlocks, blocks)

	freeIndex := 0
	for i := len(newBlocks) - 1; i >= 0; i-- {
		if i <= freeIndex {
			break
		}
		blockToMove := newBlocks[i]
		for len(blockToMove.data) > 0 && i > freeIndex {
			if newBlocks[freeIndex].free > 0 {
				newBlocks[freeIndex].data = append(newBlocks[freeIndex].data, blockToMove.data[len(blockToMove.data)-1])
				blockToMove.data = blockToMove.data[:len(blockToMove.data)-1]
				newBlocks[freeIndex].free -= 1
				blockToMove.free += 1
			} else {
				freeIndex += 1
			}
		}
		newBlocks[i] = blockToMove
	}
	return newBlocks
}

func moveBlocks(blocks []block) []block {
	newBlocks := make([]block, len(blocks))
	copy(newBlocks, blocks)

	for i := range newBlocks {
		newBlocks[i].added = make([]int64, newBlocks[i].free)
	}

	for i := len(newBlocks) - 1; i >= 0; i-- {
		freeIndex := 0
		blockToMove := newBlocks[i]
		for len(blockToMove.data) > 0 && i > freeIndex {
			if newBlocks[freeIndex].free >= int64(len(blockToMove.data)) {
				insertIndex := len(newBlocks[freeIndex].added) - int(newBlocks[freeIndex].free)
				for x, value := range blockToMove.data {
					newBlocks[freeIndex].added[insertIndex+x] = value
					blockToMove.data[x] = 0
				}
				newBlocks[freeIndex].free -= int64(len(blockToMove.data))
				blockToMove.free += int64(len(blockToMove.data))
				newBlocks[i] = blockToMove
				break
			} else {
				freeIndex += 1
			}
		}
	}
	return newBlocks
}

func checksum(blocks []block) int64 {
	index := int64(0)
	checksum := int64(0)
	for _, block := range blocks {
		for _, value := range block.data {
			checksum += value * index
			index += 1
		}
		for _, value := range block.added {
			checksum += value * index
			index += 1
		}
	}
	return checksum
}
