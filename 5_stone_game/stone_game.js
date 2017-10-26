// Code written for the Stone Game problem from Hackerrank
// JavaScript version

let cases = [1,2,3,4,5,6,7,10]
let moves = [2,3,5]

function p2(c, moves){
	let min_result = 1;
	for(let i = 0; i<moves.length; i++){
		if(c-moves[i] >= 0){
			let result = p1(c-moves[i], moves)
			if(result < min_result){
				min_result = result;
			}	
		}
	} 
	return min_result;
}
	
function p1(c, moves){
	let max_result = -1;
	for(let i = 0; i<moves.length; i++){
		if(c-moves[i] >= 0){
			let result = p2(c-moves[i], moves);
			if(result > max_result){
				max_result = result
			}
		}
	}
	return max_result;
}

function game(cases, moves){
	for(let i = 0; i<cases.length; i++){
		let outcome = p1(cases[i], moves);
		console.log(outcome);
	}
}
	
game(cases, moves)