// 2529 부등호
use std::collections::HashSet;
use std::io::{self, Read};

fn check(signs: &[&str], nums: &[char]) -> bool {
    (0..nums.len() - 1)
        .all(|i| {
            match signs[i] {
                "<" => nums[i] < nums[i + 1],
                ">" => nums[i] > nums[i + 1],
                _ => panic!("invalid sign"),
            }
        })
}

fn go(signs: &[&str], unused: &mut HashSet<char>, nums: &mut Vec<char>, ans: &mut Vec<String>) {
    if nums.len() == signs.len() + 1 {
        // 모든 숫자를 사용했을 때
        ans.push(nums.iter().collect());
        return;
    }

    let will_use = unused.clone();

    for ch in will_use {
        nums.push(ch);
        unused.remove(&ch);
        // 부등호를 만족할 때
        if check(signs, nums) {
            go(signs, unused, nums, ans);
        }
        nums.pop();
        unused.insert(ch);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("input error");
    let strs = input.split_whitespace().skip(1);

    let signs: Vec<&str> = strs.collect();
    let mut unused: HashSet<char> = ('0'..='9').collect();
    let mut num = Vec::new();
    let mut ans = Vec::new();
    go(&signs, &mut unused, &mut num, &mut ans);

    let max = ans.iter().max().expect("no answer");
    let min = ans.iter().min().expect("no answer");
    println!("{max}\n{min}");
}
