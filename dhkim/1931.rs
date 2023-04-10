// 1931 회의실 배정

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");

    let mut times: Vec<(usize, usize)> = input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse::<usize>().expect("can't parse"))
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|chunk| (chunk[1], chunk[0]))
        .collect();
    times.sort();

    let mut current_end_time = 0;
    let mut count = 0;

    for (end_time, start_time) in times {
        if current_end_time <= start_time {
            current_end_time = end_time;
            count += 1;
        }
    }

    println!("{count}");
}
