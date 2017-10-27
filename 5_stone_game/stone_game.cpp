// Code written for the Stone Game problem from Hackerrank
// Solution using dynamic programming
// Author: Ricardo Velázquez
#include <iostream>
using namespace std;

bool game[101];

int main() {
	int cases = 8;
	int n[8] = {1, 2, 3, 4, 5, 6, 7, 10};

	for (int i=0; i < cases; i++){
		game[2] = game[3] = game[4] = game[5] = true;

		for(int j=6; j <= n[i]; j++)
			game[j] = (game[j-2] == false || game[j-3] == false || game[j-5] == false);

		if(game[n[i]])
			cout << "First\n";
		else
			cout << "Second\n";
	} 
    return 0;
}
