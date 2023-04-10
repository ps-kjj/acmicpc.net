// 11286 절댓값 힙

use std::{
    collections::BinaryHeap,
    fmt::Write,
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq)]
struct Abs(isize);

impl Ord for Abs {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.abs().cmp(&self.0.abs()).then(other.0.cmp(&self.0))
    }
}

impl PartialOrd for Abs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");

    let mut pq: BinaryHeap<Abs> = BinaryHeap::new();
    for input in input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().expect("can't parse"))
    {
        match input {
            0 => writeln!(output, "{}", pq.pop().unwrap_or(Abs(0)).0).expect("write error"),
            _ => pq.push(Abs(input)),
        }
    }

    println!("{output}");
}
