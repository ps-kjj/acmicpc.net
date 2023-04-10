// 2869 달팽이는 올라가고 싶다
use std::{
    error::Error,
    io::{self, Read}
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut it = input.split_whitespace().filter_map(|str| str.parse().ok());
    let a: usize = it.next().expect("error");
    let b: usize = it.next().expect("error");
    let v: usize = it.next().expect("error");

    let remain = v - a;
    let h = a - b;
    let days = remain / h + if remain % h == 0 { 0 } else { 1 } + 1;
    println!("{days}");

    Ok(())
}
