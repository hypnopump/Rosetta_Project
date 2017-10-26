// JavaScript code for a hashing practice (Rosetta Stone)
// Author: Eric Alcaide

function req_string(){
	let word = ;
	return word
}

function main(){
	console.log("Introduce password: ")
	word = req_string()

	while true{
		console.log("Repeat password: ")
		new_w = req_string()

		if new_w == word{
			console.log("Good job!")
			break;
		}else{
			console.log("Passwords don't match. Try again.")
		}
	}
}

main();