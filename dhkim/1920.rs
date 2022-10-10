use std::io;
use std::fmt::Write;
use std::collections::HashSet;

fn main() {
    let mut t = String::new();
    let mut input = String::new();
    let mut find = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut t).unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let set: HashSet<i64> = input
        .as_mut_str()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    io::stdin().read_line(&mut t).unwrap();
    io::stdin().read_line(&mut find).unwrap();

    let vec: Vec<i64> = find
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for v in vec.iter() {
        if set.contains(v) {
            writeln!(output, "1").unwrap();
        } else {
            writeln!(output, "0").unwrap();
        }
    }

    println!("{output}");
}
