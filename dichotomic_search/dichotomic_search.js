function dichotomic_search(array = [],v = 0) {

	let med = Math.round(array.length / 2);
	
	return 	array.length > 1 ? 
				array[med] > v ? 
					dichotomic_search(array.splice(0,med),v) : 
					dichotomic_search(array.splice(med,array.length),v) : 

				array[0] == v ? 
					v : 
					null;
}

let result = dichotomic_search();