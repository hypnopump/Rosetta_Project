function fibonacci(){
	let prev = 0; let actual = 1;
	let aux = 0;

	for (let q=0; q<10;q++){
		aux = actual
		actual = prev+aux
		prev = aux
		console.log(actual)
	}
}

fibonacci();