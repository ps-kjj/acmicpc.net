// 프린터 큐
use std::collections::VecDeque;
use std::io;

fn main() {
    type Item = (usize, usize);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let tc = input.trim().parse().unwrap();
    for _ in 0..tc {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let v: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let _n = v[0];
        let m = v[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut q: VecDeque<Item> = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .enumerate()
            .map(|(i, p)| (i, p))
            .collect();

        let mut i = 1usize;
        while let Some((idx, priority)) = q.pop_front() {
            if q.iter().find(|(_, p)| priority < *p).is_some() {
                q.push_back((idx, priority));
                continue;
            }

            if idx == m {
                println!("{i}");
                break;
            }

            i += 1;
        }
    }
}
