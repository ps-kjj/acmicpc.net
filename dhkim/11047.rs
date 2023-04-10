// 11047 동전 0

use std::{
    error::Error,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut input = input.split_whitespace().filter_map(|str| str.parse().ok());

    let _ = input.next().expect("input error");
    let k = input.next().expect("input error");

    let coins: Vec<usize> = input.rev().collect();
    let mut remainder = k;
    let mut count = 0;
    for coin in coins {
        count += remainder / coin;
        remainder = remainder % coin;
    }

    println!("{count}");

    Ok(())
}
