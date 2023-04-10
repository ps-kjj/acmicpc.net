// 11279 최대 힙

use std::{
    collections::BinaryHeap,
    fmt::Write,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");

    let mut pq: BinaryHeap<usize> = BinaryHeap::new();
    for input in input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().expect("can't parse"))
    {
        match input {
            0 => writeln!(output, "{}", pq.pop().unwrap_or(0)).expect("write error"),
            _ => pq.push(input),
        }
    }

    println!("{output}");
}
