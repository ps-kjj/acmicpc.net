use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // scanf
    let n = input.trim().parse::<usize>().unwrap();

    let mut nums: [usize; 10000] = [0; 10000];
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // scanf
        nums[input.trim().parse::<usize>().unwrap() - 1] += 1;
    }

    let mut output = io::BufWriter::with_capacity(4096000, io::stdout());
    for i in 0..10000 {
        if 0 < nums[i] {
            for _ in 0..nums[i] {
                writeln!(output, "{}", i + 1).unwrap();
            }
        }
    }
}
