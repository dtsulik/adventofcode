package main

import "testing"

var testCases = []struct {
	description string
	input       string
	expected1   int
	expected2   int
}{
	{
		description: "Rock Paper or Draw",
		input:       "A Y",
		expected1:   8,
		expected2:   4,
	},
	{
		description: "Paper or Rock Loose",
		input:       "B X",
		expected1:   1,
		expected2:   1,
	},
	{
		description: "Scissors or Scissors Win",
		input:       "C Z",
		expected1:   6,
		expected2:   7,
	},
}

func TestRPS(t *testing.T) {
	for _, tc := range testCases {
		t.Run(tc.description, func(t *testing.T) {
			if part1, part2 := RPS(tc.input); part1 != tc.expected1 && part2 != tc.expected2 {
				t.Fatalf("Valid(%q) = %d, %d, want: %d, %d", tc.input, part1, part2, tc.expected1, tc.expected2)
			}
		})
	}
}

// func FuzzRPS(f *testing.F) {
// 	tests := []string{
// 		"A Y",
// 		"B X",
// 		"B Z",
// 		"B X",
// 		"B Y",
// 		"B ",
// 		"B %%",
// 		"C &#H",
// 		"C Z",
// 		" ",
// 		"  ",
// 		"",
// 	}

// 	for _, test := range tests {
// 		f.Add(test)
// 	}

// 	f.Fuzz(func(t *testing.T, a string) {
// 		RPS(a)
// 	})
// }

func BenchmarkValid(b *testing.B) {
	if testing.Short() {
		b.Skip("skipping benchmark in short mode.")
	}
	for i := 0; i < b.N; i++ {
		for _, tc := range testCases {
			RPS(tc.input)
		}
	}
}
