// 2217 로프

use std::{
    error::Error,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut input = input.split_whitespace().filter_map(|str| str.parse().ok());

    let _ = input.next().expect("input error");
    let mut ropes: Vec<usize> = input.collect();
    ropes.sort();
    ropes.reverse();

    let weight = ropes
        .iter()
        .enumerate()
        .map(|(i, rope)| rope * (i + 1))
        .max()
        .expect("can't find max");

    println!("{weight}");

    Ok(())
}
