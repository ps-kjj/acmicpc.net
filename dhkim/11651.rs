// 좌표 정렬하기2
use std::io;
use std::fmt::Write;
use std::cmp::Ordering;

struct Point {
    x: i32,
    y: i32

}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => {
                self.x.cmp(&other.x)
            },
            ordering => ordering
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut points = Vec::new();
    for _ in 0..n {
        let mut p = String::new();
        io::stdin().read_line(&mut p).unwrap();
        let v: Vec<i32> = p.as_str().split_whitespace().map(|v| v.parse().unwrap()).collect();
        points.push(Point::new(v[0], v[1]));
    }

    points.sort_by(Point::cmp);

    points.iter().for_each(|p| writeln!(output, "{} {}", p.x, p.y).unwrap());

    println!("{output}");
}
