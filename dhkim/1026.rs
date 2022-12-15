// 보물
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a: Vec<usize> = a.as_str().split_whitespace().map(|i| i.parse().unwrap()).collect();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let mut b: Vec<usize> = b.as_str().split_whitespace().map(|i| i.parse().unwrap()).collect();

    a.sort_by(|a, b| b.cmp(&a));
    b.sort();

    let res: usize = a.iter().zip(b.iter()).map(|(i, k)| i * k).sum();
    println!("{res}");
}
