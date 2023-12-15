package main

import (
	"strings"
)

func part1(input string) int {
	parts := strings.Split(input, ",")
	sum := 0
	for _, p := range parts {
		lsum := 0
		for _, c := range p {
			lsum += int(c)
			lsum *= 17
			lsum %= 256
		}
		sum += lsum
	}
	return sum
}
