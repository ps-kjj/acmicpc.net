// 2636 치즈
use std::{collections::VecDeque, error::Error, io::Read};

const DIFF: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn bfs(mut r#box: Vec<Vec<usize>>, r: usize, c: usize) -> (usize, usize) {
    let mut time = 0;
    let mut cheese = 0;

    loop {
        let mut melted = 0;
        let mut visited = vec![vec![false; c]; r];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let curr = queue.pop_front().expect("empty");

            for (dr, dc) in DIFF {
                let (nr, nc) = (curr.0 + dr, curr.1 + dc);

                if (0..r as i32).contains(&nr)
                    && (0..c as i32).contains(&nc)
                    && !visited[nr as usize][nc as usize]
                {
                    visited[nr as usize][nc as usize] = true;
                    if r#box[nr as usize][nc as usize] == 0 {
                        queue.push_back((nr, nc));
                    } else if r#box[nr as usize][nc as usize] == 1 {
                        r#box[nr as usize][nc as usize] = 0;
                        melted += 1;
                    }
                }
            }
        }
        time += 1;

        if melted == 0 {
            break;
        }

        cheese = melted;
    }

    (time - 1, cheese)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input
        .split_whitespace()
        .map(|str| str.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let r = input[0];
    let c = input[1];

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let r#box: Vec<usize> = input
        .split_whitespace()
        .map(|str| str.parse())
        .collect::<Result<_, _>>()?;
    let r#box: Vec<Vec<usize>> = r#box.chunks(c).map(|chunk| chunk.into()).collect();

    let (time, cheese) = bfs(r#box, r, c);

    println!("{time}\n{cheese}");

    Ok(())
}
