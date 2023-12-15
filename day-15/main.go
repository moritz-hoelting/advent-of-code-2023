package main

import (
	"fmt"
	"os"
)

func main() {
	data, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}
	input := string(data)

	p1 := part1(input)
	fmt.Printf("part1: %d\n", p1)
	p2 := part2(input)
	fmt.Printf("part2: %d\n", p2)
}
