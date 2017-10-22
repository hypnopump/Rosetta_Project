def fibonacci():
    prev, actual = 0, 1
    aux = 0
    for q in range(10):
        aux = actual + prev
        prev = actual
        actual = aux
        print(aux)

def main():
    fibonacci()

if __name__ == "__main__":
    main()