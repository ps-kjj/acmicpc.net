// 2580 스도쿠
// use std::collections::HashSet;
use std::io;

fn check_row(sudoku: &[[usize; 9]], r: usize, n: usize) -> bool {
    // let mut set = HashSet::new();
    for c in 0..9 {
        if sudoku[r][c] == n { // sudoku[r][c] != 0 && !set.insert(sudoku[r][c]) {
            return false;
        }
    }

    true
}

fn check_col(sudoku: &[[usize; 9]], c: usize, n: usize) -> bool {
    // let mut set = HashSet::new();
    for r in 0..9 {
        if sudoku[r][c] == n { // sudoku[r][c] != 0 && !set.insert(sudoku[r][c]) {
            return false;
        }
    }

    true
}

fn check_33(sudoku: &[[usize; 9]], r: usize, c: usize, n: usize) -> bool {
    // let mut set = HashSet::new();
    let dr = r / 3;
    let dc = c / 3;

    for i in 0..3 {
        let r = dr * 3 + i;
        for k in 0..3 {
            let c = dc * 3 + k;
            if sudoku[r][c] == n { // sudoku[r][c] != 0 && !set.insert(sudoku[r][c]) {
                return false;
            }
        }
    }

    true
}

fn check(sudoku: &[[usize; 9]], r: usize, c: usize, n: usize) -> bool {
    return check_row(sudoku, r, n) && check_col(sudoku, c, n) && check_33(sudoku, r, c, n);
}

fn go(sudoku: &mut [[usize; 9]], blanks: &[(usize, usize)]) -> bool {
    if blanks.is_empty() {
        return true;
    }

    let r = blanks[0].0;
    let c = blanks[0].1;

    for v in 1..=9 {
        if check(sudoku, r, c, v) {
            sudoku[r][c] = v;
            if go(sudoku, &blanks[1..]) {
                return true;
            }
            sudoku[r][c] = 0;
        }
    }

    false
}

fn main() {
    let mut sudoku = [[0usize; 9]; 9];
    let mut blanks = Vec::new();

    for r in 0..9 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.split_whitespace().map(|str| str.parse().unwrap());
        for (c, val) in input.enumerate() {
            if val == 0 {
                blanks.push((r, c));
            }
            sudoku[r][c] = val;
        }
    }

    go(&mut sudoku, &blanks);

    for row in sudoku {
        for col in row {
            print!("{col} ");
        }
        println!();
    }
}
