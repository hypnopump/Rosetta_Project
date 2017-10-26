# coding: utf8
# Python implementation of the write_txt code for Rosetta Stone
# Author: Eric Alcaide

def read_txt(name="data.txt"):
	# Automatically closes the file once outside of it
	with open(name, 'r') as f:
		r = f.readlines()
		first,last = r[0], r[-1]

	return first,last

def write_txt(first, second="Bye bye .txt file"):
    with open('write_py.txt', 'w') as f:
    	f.write(first)
    	f.write('\n'+second)

def main():
	first, second = read_txt()
	write_txt(first,second)

if __name__ == "__main__":
    main()