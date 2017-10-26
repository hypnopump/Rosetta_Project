# coding: utf8
# Python code for a hashing practice (Rosetta Stone)
# Author: Eric Alcaide

import hashlib

def req_string():
	word = input().encode('utf-8')
	return hashlib.sha512(word).hexdigest()

def main():
	print("Introduce password:")
	word = req_string()

	while True:
		print("Repeat password:")
		new = req_string()

		if new == word:
			print("Good job!")
			break
		else:
			print("Passwords don't match. Try again.")

if __name__ == "__main__":
	main()