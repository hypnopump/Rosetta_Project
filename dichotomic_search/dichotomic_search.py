# coding: utf8
# Python implementation of Dichotomic Search algo
# Author: Eric Alcaide

def dicho(arr = [], v=0):
	med = round(len(arr)/2)
	if len(arr) <= 1:
		return True if arr[0] == v else False
	else: 
		return dicho(arr[:med], v) if arr[med] > v else dicho(arr[med:], v)


result = dicho(['a', 'b', 'c'], 'c')
print(result)
