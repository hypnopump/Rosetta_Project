// Go code for a hashing practice (Rosetta Stone)
// Author: Eric Alcaide
package main 

import (
	"fmt"
	"crypto/sha512"
	"bufio"
	"os"
)

func req_string() string{
	reader := bufio.NewReader(os.Stdin)
	word, _ := reader.ReadString('\n')
	
	hasher := sha512.New()
	hasher.Write([]byte(word))
	b_hasher := string(hasher.Sum(nil))

	return b_hasher
}

func main() {
	fmt.Print("Introduce password: ")
	word := req_string()

	for {
		fmt.Print("Repeat password: ")
		new_w := req_string()

		if new_w == word{
			fmt.Println("Good job!", word)
			break
		}else{
			fmt.Println("Passwords don't match. Try again.")
		}
	}
}