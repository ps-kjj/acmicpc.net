// 6588 골드바흐의 추측
use std::fmt::Write;
use std::io;

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

fn is_prime(prime: &mut [bool]) {
    let mut i = 2;
    while i * i <= 1000000 {
        if prime[i] {
            let mut k = i * i;
            while k <= 1000000 {
                prime[k] = false;
                k += i;
            }
        }
        i += 1;
    }
}

fn foo(n: usize, prime: &[bool]) -> Result<(usize, usize), ()> {
    for a in 3..=1000000 {
        if prime[a] {
            let b = n - a;
            if prime[b] {
                return Ok((a, b));
            }
        }
    }

    Err(())
}

fn main() {
    let mut prime = [true; 1000001];
    is_prime(&mut prime);

    let mut output = String::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let n: usize = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        match foo(n, &prime) {
            Ok((a, b)) => writeln!(output, "{n} = {a} + {b}").unwrap(),
            Err(_) => writeln!(output, "Goldbach's conjecture is wrong.").unwrap(),
        };
    }
    println!("{output}");
}
