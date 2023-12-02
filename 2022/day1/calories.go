package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

type elf struct {
	id       int
	calories []int
}

func main() {
	f, err := os.Open("calories.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	elfs := get_elfs(scanner)
	sort.Slice(elfs, func(i, j int) bool {
		return total_calories_elf(elfs[i].calories) < total_calories_elf(elfs[j].calories)
	})

	for _, e := range elfs {
		fmt.Printf("Elf %d has %d\n", e.id, total_calories_elf(e.calories))
	}
	total_top_three := 0
	for _, e := range top_x_elfs(3, elfs) {
		total := total_calories_elf(e.calories)
		fmt.Printf("Top Elf %d has %d\n", e.id, total)
		total_top_three += total
	}
	fmt.Println(total_top_three)
}

func top_x_elfs(x int, elfs []elf) []elf {
	return elfs[len(elfs)-x:]
}

func total_calories_elf(calories []int) int {
	total := 0
	for _, calories := range calories {
		total += calories
	}
	return total
}

func get_elfs(calories *bufio.Scanner) []elf {
	elfs := []elf{}
	elfs = append(elfs, elf{0, []int{}})
	num_elfs := 0
	for calories.Scan() {

		line := string(calories.Text())

		calorie, err := strconv.Atoi(string(line))
		if err != nil {
			num_elfs++
			elfs = append(elfs, elf{})
			elfs[num_elfs].id = num_elfs
			continue
		}

		// fmt.Println(line, calorie)

		elfs[num_elfs].calories = append(elfs[num_elfs].calories, calorie)
	}

	return elfs
}
