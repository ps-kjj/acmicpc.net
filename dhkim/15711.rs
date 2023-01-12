// 15711 환상의 짝꿍
use std::io;
use std::fmt::Write;

const N: usize = 4_000_037;

fn get_prime_numbers() -> Vec<usize> {
    let mut prime = vec![true; N + 1];
    let mut i: usize = 2;
    while i * i <= N {
        if prime[i] {
            let mut k = i * i;
            while k <= N {
                prime[k] = false;
                k += i;
            }
        }
        i += 1;
    }

    prime
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, b)| b.then_some(i))
        .collect()
}

fn is_prime_number(n: usize, prime_numbers: &[usize]) -> bool {
    if prime_numbers.binary_search(&n).is_ok() {
        return true;
    }

    for p in prime_numbers {
        if n < p * p { break; }

        if n % p == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut output = String::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse().unwrap();

    let prime_numbers = get_prime_numbers();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap());

        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let n = a + b;

        if n < 4 {
            writeln!(output, "NO").unwrap();
        } else if n & 1 == 0 || is_prime_number(n - 2, &prime_numbers) {
            writeln!(output, "YES").unwrap();
        } else {
            writeln!(output, "NO").unwrap();
        }
    }
    println!("{output}");
}
