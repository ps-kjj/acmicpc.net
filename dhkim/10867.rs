// 중복 빼고 정렬하기
use std::{io, collections::BTreeSet, fmt::Write};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let set: BTreeSet<i32>  = input.as_str().split_whitespace().map(|i| i.parse().unwrap()).collect();

    set.iter().for_each(|v| write!(output, "{v} ").unwrap());

    println!("{output}");
}
