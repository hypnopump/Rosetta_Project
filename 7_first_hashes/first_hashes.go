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
	
	sha_512 := sha512.New()
	sha_512.Write([]byte(word))
	sha_512_out := fmt.Sprintf("%x", sha_512.Sum(nil))

	return sha_512_out
}

func main() {
	fmt.Print("Introduce password: ")
	word := req_string()

	for {
		fmt.Print("Repeat password: ")
		new_w := req_string()

		if new_w == word{
			fmt.Println("Good job!")
			break
		}else{
			fmt.Println("Passwords don't match. Try again.")
		}
	}
}