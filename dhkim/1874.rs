// 스택 수열
use std::{io, fmt::Write};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut stack = Vec::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let mut max = 1;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let v = input.trim().parse().unwrap();

        while max <= v {
            stack.push(max);
            writeln!(output, "+").unwrap();
            max += 1;
        }

        if let Some(&top) = stack.last() {
            if top == v {
                stack.pop();
                writeln!(output, "-").unwrap();
            } else {
                println!("NO");
                return;
            } 
        } else {
            println!("NO");
            return;
        }
    }

    println!("{output}");
}
