// 1182 부분수열의 합
use std::io;
// use std::fmt::Write;

fn op(series: &[isize], sum: isize, s: isize) -> usize {
    if series.is_empty() {
        return if sum == s { 1 } else { 0 };
    }

    op(&series[1..], sum, s) + op(&series[1..], sum + series[0], s)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s: isize = input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .next()
        .unwrap();

    let mut series = String::new();
    io::stdin().read_line(&mut series).unwrap();
    let series: Vec<isize> = series
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let ans = op(&series, 0, s) - if s == 0 { 1 } else { 0 };
    println!("{ans}");
}
