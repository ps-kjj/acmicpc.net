// 2156 포도주 시식
use std::{
    error::Error,
    io::{self, Read},
};

/*
 * https://hongcoding.tistory.com/48
 * 즉, 현재 포도주를 마실 지 말지를 결정 할 때는
 * 현재 포도주와 이전 포도주를 마시고 전전 포두주는 마시지 않는다. ( wines[i]+wines[i-1]+dp[i-3] )
 * 현재 포도주와 전전 포도주를 마시고 이전 포도주는 마시지 않는다. ( wines[i]+dp[i-2] )
 * 현재 포도주를 마시지 않는다. ( dp[i-1] )
 */

fn go(wines: &[usize], dp: &mut [Option<usize>], curr: usize) -> usize {
    if curr == 0 {
        dp[0] = Some(wines[0]);
        return dp[0].unwrap();
    } else if curr == 1 {
        dp[1] = Some(wines[0] + wines[1]);
        return dp[1].unwrap();
    } else if curr == 2 {
        dp[2] = Some(
            (wines[0] + wines[2])
                .max(wines[1] + wines[2])
                .max(go(wines, dp, 1)),
        );
        return dp[2].unwrap();
    }

    if let Some(memo) = dp[curr] {
        return memo;
    }

    let case1 = wines[curr] + wines[curr - 1] + go(wines, dp, curr - 3);
    let case2 = wines[curr] + go(wines, dp, curr - 2);
    let case3 = go(wines, dp, curr - 1);
    let max = case1.max(case2).max(case3);

    dp[curr] = Some(max);
    max
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let wines = input
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let mut dp = vec![None; wines.len()];

    let ans = go(&wines, &mut dp, wines.len() - 1);
    println!("{ans}");

    Ok(())
}
