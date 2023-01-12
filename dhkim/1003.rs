// 1003 피보나치 함수
use std::fmt::Write;
use std::io;

/*
int fibonacci(int n) {
   if (n == 0) {
       printf("0");
       return 0;
   } else if (n == 1) {
       printf("1");
       return 1;
   } else {
       return fibonacci(n‐1) + fibonacci(n‐2);
   }
}
 */

fn fibonacci_count(dp: &mut [(usize, usize)], n: usize) -> (usize, usize) {
    if (0, 0) < dp[n] {
        return dp[n];
    }

    match n {
        0 => (1, 0),
        1 => (0, 1),
        _ => {
            dp[n - 1] = fibonacci_count(dp, n - 1);
            dp[n - 2] = fibonacci_count(dp, n - 2);
            dp[n] = (dp[n - 1].0 + dp[n - 2].0, dp[n - 1].1 + dp[n - 2].1);
            dp[n]
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t: usize = input.trim().parse().unwrap();
    let mut dp = [(0, 0); 40 + 1];
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let (zero, one) = fibonacci_count(&mut dp, n);
        writeln!(output, "{zero} {one}").unwrap();
    }

    println!("{output}");
}
