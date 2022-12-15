// 숫자카드2
use std::{io, collections::HashMap, fmt::Write};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    
    let mut map: HashMap<i32, u32> = HashMap::new();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    input.as_str().split_whitespace().map(|i| {
        i.parse().unwrap()
    })
    .for_each(|k| {
        map.entry(k)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    input.as_str().split_whitespace()
    .map(|v| {
        v.parse().unwrap()
    })
    .for_each(|k| {
        match map.get(&k) {
            Some(v) => {
                write!(output, "{v} ").unwrap()
            },
            None => {
                write!(output, "0 ").unwrap()
            }
        }
    });
    
    println!("{output}");
}
