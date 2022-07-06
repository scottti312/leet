fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let (mut prev, mut prev1, mut current) = (0, 1, 0);
    for _i in 2..n+2 {
        current = prev + prev1;
        prev1 = prev;
        prev = current;
    }
    current
}

fn main() {
    println!("{}", fib(10));
}
