# coding: utf8
# Python implementation of the write_txt code for Rosetta Stone
# Author: Caio Cesar Viana da Silva

import random

def read_txt(name="data.txt"):
	# Automatically closes the file once outside of it
	with open(name, 'r') as f:
		read_lines = f.readlines()
		read_lines = [x.strip() for x in read_lines] #remove whitespace characters like `\n` at the end of each line

	return read_lines

def main():
	number_of_lines=10
	lines = read_txt()
	list_of_numbers = random.sample(xrange(len(lines)), number_of_lines) #10 random numbers/lines of the text
	for i in range (0,number_of_lines):
		print(lines[list_of_numbers[i]]) #output the random lines
		

if __name__ == "__main__":
    main()