// 9205 맥주 마시면서 걸어가기
use std::{collections::VecDeque, fmt::Write, io::Read};

enum Type {
    HOME,
    FESTIVAL,
    CONVENIENCE,
}

struct Point {
    place: Type,
    r: i32,
    c: i32,
}

fn get_distance(a: &Point, b: &Point) -> i32 {
    (a.r - b.r).abs() + (a.c - b.c).abs()
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read error");
    let mut input = input
        .split_whitespace()
        .map(|str| str.parse::<i32>().expect("parse error"));

    let test_case = input.next().expect("test_case error");

    for _ in 0..test_case {
        let mut queue = VecDeque::new();
        let mut points = vec![];
        let mut visited;
        let convenience_count = input.next().expect("convenience_count error");

        // home
        let home_c = input.next().expect("home_x error");
        let home_r = input.next().expect("home_y error");
        queue.push_back(Point {
            place: Type::HOME,
            r: home_r,
            c: home_c,
        });

        // convenience
        for _ in 0..convenience_count {
            let convenience_c = input.next().expect("convenience_x error");
            let convenience_r = input.next().expect("convenience_y error");
            points.push(Point {
                place: Type::CONVENIENCE,
                r: convenience_r,
                c: convenience_c,
            });
        }

        // festival
        let festival_c = input.next().expect("festival_x error");
        let festival_r = input.next().expect("festival_y error");
        points.push(Point {
            place: Type::FESTIVAL,
            r: festival_r,
            c: festival_c,
        });

        visited = vec![false; points.len()];

        'outer: while !queue.is_empty() {
            let current = queue.pop_front().expect("queue pop error");

            for (idx, visited) in visited.iter_mut().enumerate().filter(|(_, &mut visited)| !visited) {
                let next = &points[idx];
                let distance = get_distance(&current, next);

                if distance <= 1000 {
                    *visited = true;
                    match next.place {
                        Type::FESTIVAL => {
                            break 'outer;
                        }
                        _ => {
                            queue.push_back(Point {
                                place: Type::CONVENIENCE,
                                r: next.r,
                                c: next.c,
                            });
                        }
                    }
                }
            }
        }

        if let Some(visit) = visited.last() {
            match visit {
                true => writeln!(output, "happy").expect("write error"),
                false => writeln!(output, "sad").expect("write error"),
            }
        }
    }

    print!("{output}");
}
