fn main() {
    let n = std::env::args().skip(1).next()
        .expect("Should provide n for point in Fibonacci sequence.")
        .parse::<i32>()
        .expect("Could not parse n as an i32!");

    if n < 0 { panic!("n={} should not be less than zero!"); }

    println!("{}", fibonacci(n));
}

fn fibonacci(n: i32) -> i64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _i in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}
