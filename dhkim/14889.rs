// 14889 스타트와 링크
use std::io;

fn op(power: &Vec<Vec<usize>>, selected: &mut Vec<bool>, i: usize, n: usize) -> usize {
    if selected.iter().filter(|&&selected| selected).count() == n / 2 {
        let mut start: usize = 0;
        let mut link: usize = 0;
        for i in 0..n  {
            for k in 0..n {
                if selected[i] && selected[k] {
                    start += power[i][k];
                } else if !selected[i] && !selected[k] {
                    link += power[i][k];
                }
            }
        }

        return start.abs_diff(link);
    }

    let mut ans = usize::MAX;
    for sel in i..n {
        selected[sel] = true;
        ans = std::cmp::min(ans, op(&power, selected, sel + 1, n));
        selected[sel] = false;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut power: Vec<Vec<usize>> = vec![];
    let mut seleted: Vec<bool> = vec![false; n];
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        power.push(line.split_whitespace().map(|str| str.parse().unwrap()).collect());
    }

    let ans = op(&power, &mut seleted, 0, n);
    println!("{ans}");
}
