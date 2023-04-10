// 11053 가장 긴 증가하는 부분 수열

use std::io::{self, Read};

fn longest(seq: &[usize], dp: &mut [usize], curr: usize) -> usize {
    if seq.len() == curr {
        return 1;
    }

    if dp[curr] != 0 {
        return dp[curr];
    }

    dp[curr] = 1;

    (curr + 1..seq.len())
        .filter(|&next| seq[curr] < seq[next])
        .for_each(|next| dp[curr] = dp[curr].max(longest(seq, dp, next) + 1));

    dp[curr]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");
    let seq: Vec<usize> = input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().expect("parse error"))
        .collect();
    let mut dp = vec![0; seq.len()];

    let ans = (0..seq.len())
        .map(|i| longest(&seq, &mut dp, i))
        .max()
        .expect("seq is empty");
    println!("{ans}");
}
