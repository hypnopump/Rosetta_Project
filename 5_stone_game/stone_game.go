// Code written for the Stone Game problem from Hackerrank
// Go version
package main 

import "fmt"

func p2 (c int, moves [3]int) int{
	min_result := 1;
	for i:=0; i<len(moves); i++{
		if c - moves[i] >= 0{
			result := p1(c-moves[i], moves)
			if result < min_result{
				min_result = result;
			}
		}
	}
	return min_result
}

func p1(c int, moves [3]int) int{
	max_result := -1;
	for i:=0; i<len(moves); i++{
		if c - moves[i] >= 0{
			result := p2(c-moves[i], moves)
			if result > max_result{
				max_result = result;
			}
		}
	}
	return max_result
}

func main() {
	var cases = [8]int{1,2,3,4,5,6,7,10}
	var moves = [3]int{2,3,5};
	for i:=0; i<len(cases); i++{
		outcome := p1(cases[i], moves)
		fmt.Println(outcome)
	}
}
