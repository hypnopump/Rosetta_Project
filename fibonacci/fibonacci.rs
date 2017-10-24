/* Rust implementation. Prints 10 Fibonacci Series numbers */

fn fibonacci(n: i32) -> i32 {
    if n <= 1 { return n;}
    fibonacci(n-1) + fibonacci(n-2)
}

fn main() {
    // it start to 2 and ends to 11, because of the shift of lefting behind the first 1 in the series...
    for i in 2..12 {
        print!("{} ", fibonacci(i));
    }
}
