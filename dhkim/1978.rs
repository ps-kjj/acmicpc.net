// 1978 소수 찾기
use std::io::{self, Read};
// use std::fmt::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ans = input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().unwrap())
        .filter(|num: &usize| {
            if *num < 2 {
                return false;
            }
            let mut i = 2usize;
            while i * i <= *num {
                if num % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        })
        .count();

    println!("{ans}");
}
