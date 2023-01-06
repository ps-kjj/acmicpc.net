// 2485 가로수
use std::collections::HashSet;
use std::io::{self, Read};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let location: Vec<usize> = input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().unwrap())
        .collect();

    let diff_set: HashSet<usize> = location.windows(2).map(|w| w[1] - w[0]).collect();

    let mut total_gcd: usize = *diff_set.iter().next().unwrap();
    diff_set.iter().skip(1).for_each(|&d| {
        total_gcd = gcd(total_gcd, d);
    });

    let n = location
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold(0, |acc, diff| acc + (diff / total_gcd) - 1);

    println!("{n}");
}
