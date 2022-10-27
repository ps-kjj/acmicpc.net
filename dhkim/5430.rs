// AC
use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{self, Read};
use std::str::SplitWhitespace;

fn op(input: &mut SplitWhitespace, output: &mut String) {
    let p = input.next().unwrap();
    let mut input = input.skip(1);

    let x_str = input
        .next()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']'); // remove whitespaces

    let mut x: VecDeque<usize>;
    if x_str.is_empty() && p.find('D').is_some() {
        writeln!(output, "error").unwrap();
        return;
    } else if x_str.is_empty() {
        writeln!(output, "[]").unwrap();
        return;
    }

    x = x_str
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    let mut rev = false;
    for op in p.chars() {
        match op {
            'R' => {
                rev = !rev;
            }
            'D' => match rev {
                true => {
                    if let None = x.pop_back() {
                        writeln!(output, "error").unwrap();
                        return;
                    }
                }
                false => {
                    if let None = x.pop_front() {
                        writeln!(output, "error").unwrap();
                        return;
                    }
                }
            },
            _ => {}
        }
    }

    let mut ans: Vec<String> = x.iter().map(|i| i.to_string()).collect();
    if rev {
        ans.reverse();
    }
    let ans = ans.join(",");
    writeln!(output, "[{ans}]").unwrap();
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut input = input.split_whitespace();
    let tc: usize = input.next().unwrap().parse().unwrap();
    for _ in 0..tc {
        op(&mut input, &mut output);
    }

    println!("{output}");
}
