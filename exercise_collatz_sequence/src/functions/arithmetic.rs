pub fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        return gcd(b, a % b);
    } else {
        return a;
    }
}
