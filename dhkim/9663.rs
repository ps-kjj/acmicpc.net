// 9663 N-Queen
use std::io;

fn can_place(placed: &Vec<usize>, row: usize, col: usize) -> bool {
    for r in 0..row {
        let c = placed[r];
        if c == col || r.abs_diff(row) == c.abs_diff(col) {
            return false;
        }
    }

    true
}

fn go(placed: &mut Vec<usize>, row: usize, n: usize) -> usize {
    if row == n {
        return 1;
    }

    let mut cnt = 0;
    for i in 0..n {
        if can_place(&placed, row, i) {
            placed[row] = i;
            cnt += go(placed, row + 1, n);
            placed[row] = 15;
        }
    }

    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut placed: Vec<usize> = vec![15; n];
    let ans = go(&mut placed, 0, n);
    println!("{ans}");
}
