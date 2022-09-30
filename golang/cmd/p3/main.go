package main

import (
	"fmt"
	"strings"
)

type State = int

const (
	Tree State = iota
	Snow
)

func getInput() string {
	return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
}

func main() {
	treeCount := 0
	for row, line := range strings.Split(getInput(), "\n") {
		if line[row*3%len(line)] == '#' {
			treeCount++
		}

	}

	fmt.Printf("treecount %v\n", treeCount)
}
