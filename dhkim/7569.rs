// 7569 토마토
use std::{collections::VecDeque, error::Error, io::Read};

const DIFF: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn bfs(r#box: &[&[&[i32]]], m: usize, n: usize, h: usize) -> Option<i32> {
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; m]; n]; h];

    r#box.iter().enumerate().for_each(|(h, hv)| {
        hv.iter().enumerate().for_each(|(n, nv)| {
            let mut res = nv
                .iter()
                .enumerate()
                .filter_map(|(m, &status)| match status {
                    -1 => {
                        visited[h][n][m] = true;
                        None
                    }
                    1 => {
                        visited[h][n][m] = true;
                        Some((h as i32, n as i32, m as i32))
                    }
                    _ => None,
                })
                .collect();

            queue.append(&mut res);
        })
    });

    let mut day = 0;

    while !queue.is_empty() {
        let q_size = queue.len();

        for _ in 0..q_size {
            let (ch, cn, cm) = queue.pop_front()?;

            for (dh, dn, dm) in DIFF {
                let (nh, nn, nm) = (ch + dh, cn + dn, cm + dm);
                if (0..h as i32).contains(&nh)
                    && (0..n as i32).contains(&nn)
                    && (0..m as i32).contains(&nm)
                    && !visited[nh as usize][nn as usize][nm as usize]
                    && r#box[nh as usize][nn as usize][nm as usize] == 0
                {
                    visited[nh as usize][nn as usize][nm as usize] = true;
                    queue.push_back((nh, nn, nm));
                }
            }
        }
        day += 1;
    }

    if visited.iter().flatten().flatten().any(|v| !v) {
        None
    } else {
        Some(day - 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut input = input.split_whitespace().map(|str| str.parse().unwrap());
    let m = input.next().unwrap();
    let n = input.next().unwrap();
    let h = input.next().unwrap();

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let r#box: Vec<i32> = buf
        .split_whitespace()
        .map(|str| str.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let r#box: Vec<&[i32]> = r#box.chunks(m).collect();
    let mut r#box: Vec<&[&[i32]]> = r#box.chunks(n).collect();

    let ans = bfs(&mut r#box, m, n, h).unwrap_or(-1);

    println!("{ans}");

    Ok(())
}
