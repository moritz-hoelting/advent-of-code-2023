package main

import (
	"slices"
	"strconv"
	"strings"
)

type lens struct {
	focal_length int
	label        string
}

func part2(input string) int {
	instructions := strings.Split(input, ",")
	boxes := make([][]lens, 256)
	for _, instruction := range instructions {
		if strings.Contains(instruction, "=") {
			split := strings.Split(instruction, "=")
			label := split[0]
			box_index := hash(label)
			focal_length, _ := strconv.Atoi(split[1])
			found_box := boxes[box_index]
			lens_index := slices.IndexFunc(found_box, func(l lens) bool {
				return l.label == label
			})
			if lens_index >= 0 {
				found_box[lens_index] = lens{focal_length, label}
				boxes[box_index] = found_box
			} else {
				boxes[box_index] = append(found_box, lens{focal_length, label})
			}
		} else if strings.Contains(instruction, "-") {
			split := strings.Split(instruction, "-")
			label := split[0]
			box_index := hash(label)
			found_box := boxes[box_index]
			boxes[box_index] = slices.DeleteFunc(found_box, func(l lens) bool {
				return l.label == label
			})
		}
	}

	sum := 0

	for box_number, box := range boxes {
		for slot_number, lens := range box {
			sum += ((box_number + 1) * (slot_number + 1) * lens.focal_length)
		}
	}

	return sum
}

func hash(input string) int {
	sum := 0
	for _, c := range input {
		sum += int(c)
		sum *= 17
		sum %= 256
	}
	return sum
}
