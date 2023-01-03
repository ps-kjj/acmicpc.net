// 2661 좋은수열
use std::io;

fn is_bad(sequence: &str) -> bool {
    for start in 0..sequence.len() {
        let slice = &sequence[start..];

        for i in 1..=(slice.len() / 2) {
            if slice[..i] == slice[i..(i * 2)] {
                return true;
            }
        }
    }

    false
}

fn op(sequence: &str, n: usize) -> bool {
    if is_bad(sequence) {
        return false;
    }

    if sequence.len() == n {
        println!("{sequence}");
        return true;
    }

    for num in ["1", "2", "3"] {
        let next = sequence.to_owned() + num;
        if op(&next, n) {
            return true;
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: usize = input.trim().parse().unwrap();
    op("", n);
}
