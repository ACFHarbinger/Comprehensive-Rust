pub fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

pub fn fib_iter(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    return a;
}

pub fn collatz(n: i32, count: u32) -> u32 {
    if n == 1 {
        return count + 1;
    } else if n % 2 == 0 {
        return collatz(n / 2, count + 1);
    } else {
        return collatz(3 * n + 1, count + 1);
    }
}

pub fn collatz_iter(n: i32) -> u32 {
    let mut count = 0;
    let mut num = n;
    while num != 1 {
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        count += 1;
    }
    count + 1
}
