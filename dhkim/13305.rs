// 13305 주유소

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("can't read input");
    let mut input = input
        .split_whitespace()
        .map(|str| str.parse().expect("can't parse str"));

    let n = input.next().expect("input error") - 1;

    let v: Vec<usize> = input.collect();

    let distances = &v[..n];
    let costs = &v[n..];

    let mut ans = 0;
    let mut curr = 0;

    while let Some(pos) = costs[curr + 1..].iter().position(|&cost| cost <= costs[curr]) {
        ans += costs[curr] * distances[curr..curr + pos + 1].iter().sum::<usize>();
        curr += pos + 1;
    }
    ans += costs[curr] * distances[curr..].iter().sum::<usize>();

    println!("{ans}");
}
