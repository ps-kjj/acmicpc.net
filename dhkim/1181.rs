// 단어 정렬
use std::{io, collections::HashSet, fmt::Write, cmp::Ordering};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut words = HashSet::new();
    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        words.insert(word);
    }

    let mut v = Vec::from_iter(words.iter());
    v.sort_by(|a, b| {
        match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(&b),
            ordering => ordering,
        }
    });

    v.iter().for_each(|w| write!(output, "{w}").unwrap());
    println!("{output}");
}
