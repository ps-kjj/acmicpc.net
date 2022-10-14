use std::collections::LinkedList;
use std::io;
use std::fmt::Write;
use std::str::SplitWhitespace;

fn op(cmd: &mut SplitWhitespace, queue: &mut LinkedList<usize>) -> String {
    let mut output = String::new();
    match cmd.next().unwrap() {
        "push" => {
            queue.push_back(cmd.next().unwrap().parse::<usize>().unwrap())
        },
        "pop" => {
            match queue.pop_front() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        "size" => {
            writeln!(output, "{}", queue.len()).unwrap();
        },
        "empty" => {
            writeln!(output, "{}", if queue.is_empty() { 1 } else { 0 }).unwrap();
        },
        "front" => {
            match queue.front() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        "back" => {
            match queue.back() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        _ => { }
    }

    output
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut queue = LinkedList::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut cmd = input.split_whitespace();
        write!(output, "{}", op(&mut cmd, &mut queue)).unwrap();
    }

    println!("{output}");
}
