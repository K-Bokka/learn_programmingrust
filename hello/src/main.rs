extern crate core;

fn main() {
    println!("Hello, world!");

    let i = gcd(24, 32);
    println!("gcd(24, 32) -> {}", i)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
