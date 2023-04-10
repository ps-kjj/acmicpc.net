use std::io;

fn main() {
    let mut t = String::new();

    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut values:Vec<f64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        let d = ((values[0] - values[3]) * (values[0] - values[3]) + (values[1] - values[4]) * (values[1] - values[4])).sqrt();
        let sub_r = (values[2] - values[5]).abs();
        let add_r = (values[2] + values[5]).abs();

        if values[0] == values[3] && values[1] == values[4] && values[2] == values[5] {
            println!("{}", -1);
        } else if d < sub_r {
            println!("{}", 0);
        } else if d == sub_r {
            println!("{}", 1);
        } else if d < add_r {
            println!("{}", 2);
        } else if d == add_r {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }

        values.clear();
    }

}
