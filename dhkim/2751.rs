use std::io;
use std::fmt::Write;

fn main() {
    let mut n = String::new();
    let mut output = String::new();

    io::stdin().read_line(&mut n).unwrap(); // scanf

    let mut nums: Vec<u16> = Vec::new();
    let n = n.trim().parse().unwrap();
    for _i in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap(); // scanf
        nums.push(num.trim().parse().unwrap());
    }

    nums.sort();

    nums.iter().for_each(|item| writeln!(output, "{item}").unwrap());
    println!("{output}");
}