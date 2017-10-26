# coding: utf8
# Python code written for the Stone Game problem from Hackerrank
# Author: Eric Alcaide

cases = [1,2,3,4,5,6,7,10]
moves = [2,3,5]
convert = {1: "First", -1: "Second"}

def p2(c, moves):
	min_result = 1
	for m in moves:
		if c-m >= 0:
			result = p1(c-m, moves)
			if result < min_result:
				min_result = result

	return min_result

def p1(c, moves):
	max_result = -1
	for m in moves:
		if c-m >= 0:
			result = p2(c-m, moves)
			if result > max_result:
				max_result = result

	return max_result

def game(cases, convert):
	for c in cases: 
		outcome = p1(c, moves)
		print(convert[outcome])

game(cases, convert)