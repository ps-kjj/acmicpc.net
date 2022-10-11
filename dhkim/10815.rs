use std::io;
use std::fmt::Write;

fn main() {
    let mut cards: Vec<isize> = Vec::new();
    let mut input = String::new();
    let mut output = String::new();

    io::stdin().read_line(&mut input).unwrap(); // N
    input.clear();

    io::stdin().read_line(&mut input).unwrap(); // cards
    cards = input
        .as_mut_str()
        .split_whitespace()
        .map(|item| item.parse().unwrap())
        .collect();

    cards.sort(); // 이분탐색을 위한 정렬
    
    io::stdin().read_line(&mut input).unwrap(); // M
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    input.as_mut_str().split_whitespace().map(|item| item.parse::<isize>().unwrap()).for_each(|item| {
        match cards.binary_search(&item) {
            Ok(_) => {
                writeln!(output, "1").unwrap();
            },
            Err(_) => {
                writeln!(output, "0").unwrap();
            }
        }
    });

    println!("{output}");
}
