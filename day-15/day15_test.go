package main

import "testing"

var input string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"

func TestPart1(t *testing.T) {
	expected := 1320
	actual := part1(input)

	if actual != expected {
		t.Errorf("part1 = %d, expected %d", actual, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 145
	actual := part2(input)

	if actual != expected {
		t.Errorf("part2 = %d, expected %d", actual, expected)
	}
}
