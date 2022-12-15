// 괄호
use std::io;
use std::fmt::Write;
use std::str::Chars;

fn op(mut vps: Chars) -> bool {
    let mut stack = Vec::new();
    for ch in &mut vps {
        match ch {
            '(' => {
                stack.push(ch)
            },
            ')' => {
                match stack.pop() {
                    Some(_) => { },
                    None => return false
                }
            }
            _ => { }
        }
    }

    if stack.is_empty() { true }
    else { false }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        let mut vps = String::new();
        io::stdin().read_line(&mut vps).unwrap();
        let vps = vps.trim().to_string();
        if op(vps.chars()) {
            writeln!(output, "YES").unwrap();
        } else {
            writeln!(output, "NO").unwrap();
        }
    }

    println!("{output}");
}
