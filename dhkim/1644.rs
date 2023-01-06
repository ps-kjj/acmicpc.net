// 1644 소수의 연속합
use std::{cmp::Ordering, io};

/*
   https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Algorithm_and_variants
   algorithm Sieve of Eratosthenes is
   input: an integer n > 1.
   output: all prime numbers from 2 through n.

   let A be an array of Boolean values, indexed by integers 2 to n,
   initially all set to true.

   for i = 2, 3, 4, ..., not exceeding √n do
       if A[i] is true
           for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
               set A[j] := false

   return all i such that A[i] is true.
*/

const N: usize = 4_000_037; // 마지막 소수 처리 어케하지...

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

fn go(prime_numbers: &[usize], n: usize) -> usize {
    let mut from = 0;
    let mut to = 1;
    let mut sum = prime_numbers[0];
    let mut cnt = 0;

    while to < prime_numbers.len() {
        match sum.cmp(&n) {
            Ordering::Greater => {
                sum -= prime_numbers[from];
                from += 1;
            }
            Ordering::Less => {
                sum += prime_numbers[to];
                to += 1;
            }
            Ordering::Equal => {
                cnt += 1;
                
                sum += prime_numbers[to];
                to += 1;
            }
        }
    }

    cnt
}

fn main() {
    let prime_numbers = get_prime_numbers();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let ans = go(&prime_numbers, n);
    println!("{ans}");
}
