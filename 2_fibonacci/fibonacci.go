// Go implementation of the Fibonacci Series
// Author: Eric Alcaide

package main 

import "fmt"

func fibonacci(){
	prev, actual := 0,1

	for q:=0; q < 10; q++{
		actual, prev = prev+actual, actual
		fmt.Println(actual)
	}
}

func main(){
	fibonacci()
}