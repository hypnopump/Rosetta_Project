def factorialize(num):
    if not num:
        return 1
    else:
        return num * factorialize(num-1)

print factorialize(0)
