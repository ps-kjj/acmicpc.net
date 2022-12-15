// 1929 소수 구하기
use std::io;
use std::fmt::Write;

fn is_prime(num: usize) -> bool {
    if num == 1 { return false; }

    let mut k: usize = 2;
    while k * k <= num {
        if num % k == 0 {
            return false;
        }
        k += 1;
    }

    true
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|str| str.parse().unwrap());
    let m: usize = input.next().unwrap();
    let n: usize = input.next().unwrap();

    for i in m..=n {
        if is_prime(i) {
            writeln!(output, "{i}").unwrap();
        }
    }

    println!("{output}");
}
