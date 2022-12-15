// 2609 최대공약수와 최소공배수
use std::io;
// use std::fmt::Write;

fn gcd(mut n: usize, mut m: usize) -> usize {
    while m != 0 {
        let remainder = n % m;
        n = m;
        m = remainder;
    }

    n
}

fn main() {
    let mut input = String::new();
    // let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.split_whitespace().map(|str| str.parse().unwrap());
    let n: usize = input.next().unwrap();
    let m: usize = input.next().unwrap();

    let gcd = gcd(n, m);
    let lcm = n * m / gcd;
    println!("{gcd}");
    println!("{lcm}");
}
