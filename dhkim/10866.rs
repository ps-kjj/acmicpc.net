use std::collections::LinkedList;
use std::io;
use std::fmt::Write;
use std::str::SplitWhitespace;

fn op(cmd: &mut SplitWhitespace, deque: &mut LinkedList<usize>) -> String {
    let mut output = String::new();
    match cmd.next().unwrap() {
        "push_front" => {
            deque.push_front(cmd.next().unwrap().parse::<usize>().unwrap())
        },
        "push_back" => {
            deque.push_back(cmd.next().unwrap().parse::<usize>().unwrap())
        },
        "pop_front" => {
            match deque.pop_front() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        "pop_back" => {
            match deque.pop_back() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        "size" => {
            writeln!(output, "{}", deque.len()).unwrap();
        },
        "empty" => {
            writeln!(output, "{}", if deque.is_empty() { 1 } else { 0 }).unwrap();
        },
        "front" => {
            match deque.front() {
                Some(i) => {
                    writeln!(output, "{i}").unwrap();
                },
                None => {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
        },
        "back" => {
            match deque.back() {
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

    let mut deque = LinkedList::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut cmd = input.split_whitespace();
        write!(output, "{}", op(&mut cmd, &mut deque)).unwrap();
    }

    println!("{output}");
}
