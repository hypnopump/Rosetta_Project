package main 

import "fmt"

func fibonacci(){
	prev, actual := 0,1
	aux := 0

	for q:=0; q < 10; q++{
		aux = actual
		actual = prev+aux
		prev = aux
		fmt.Println(actual)
	}
}

func main(){
	fibonacci()
}