// 1037 약수
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut max: usize = usize::MIN;
    let mut min: usize = usize::MAX;
    input.trim().split_whitespace().skip(1).for_each(|u| {
        let parsed = u.parse().unwrap();
        max = if max < parsed { parsed } else { max };
        min = if parsed < min { parsed } else { min };
    });
    let ans = min * max;

    println!("{ans}");
}
