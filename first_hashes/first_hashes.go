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
	fmt.Print("Introduce password: ")
	word, _ := reader.ReadString('\n')
	fmt.Println(word)
	
	hasher := sha512.New()
	hasher.Write([]byte(word))
	b_hasher := hasher.Sum(nil)

	fmt.Println(word)
	fmt.Println(b_hasher)

	return b_hasher
}

func main() {
	req_string()
}