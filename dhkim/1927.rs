// 1927 최소 힙

use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::Write,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");

    let mut pq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for input in input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().expect("can't parse"))
    {
        match input {
            0 => writeln!(output, "{}", pq.pop().unwrap_or(Reverse(0)).0).expect("write error"),
            _ => pq.push(Reverse(input)),
        }
    }

    println!("{output}");
}
