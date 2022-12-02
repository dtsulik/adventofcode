package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// A Rock
// B Paper
// C Scissors

// X Rock
// Y Paper
// Z Scissors

// X Loose
// Y Draw
// Z Win

var shape_score = map[string]int{"X": 1, "Y": 2, "Z": 3, "A": 1, "B": 2, "C": 3}
var shape_name = map[string]string{"X": "Rock", "Y": "Paper", "Z": "Scissors", "A": "Rock", "B": "Paper", "C": "Scissors"}
var win_loose_translation = map[string]string{"X": "Loose", "Y": "Draw", "Z": "Win"}
var outcome_combinations = map[string]int{"AX": 3, "AY": 6, "AZ": 0, "BX": 0, "BY": 3, "BZ": 6, "CX": 6, "CY": 0, "CZ": 3}
var win_loose_combinations = map[string]string{"AX": "Z", "AY": "X", "AZ": "Y", "BX": "X", "BY": "Y", "BZ": "Z", "CX": "Y", "CY": "Z", "CZ": "X"}

func RPS(line string) (int, int) {
	shapes := strings.Split(line, " ")
	opponent := shapes[0]
	me := shapes[1]

	part1 := outcome_combinations[opponent+me] + shape_score[me]

	my_shape_win_loose := win_loose_combinations[opponent+me]
	fmt.Printf("Opponent: %s, Me: %s ", opponent, me)
	fmt.Printf("Opponent: %s, Me: %s, Instruction Win/Loose/Draw: %s\n", shape_name[opponent], shape_name[my_shape_win_loose], win_loose_translation[me])
	part2 := outcome_combinations[opponent+my_shape_win_loose] + shape_score[my_shape_win_loose]
	return part1, part2
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	total := 0
	total_win_loose := 0
	for scanner.Scan() {
		line := string(scanner.Text())
		part1, part2 := RPS(line)
		total += part1
		total_win_loose += part2
	}
	fmt.Println(total)
	fmt.Println(total_win_loose)
}
