// 1016 제곱 ㄴㄴ수
use std::io;

fn get_numbers(min: usize, max: usize) -> usize {
    let mut numbers = vec![true; max - min + 1];
    let mut i: usize = 2;
    while i * i <= max {
        let square = i * i;
        let mut k = (min / square + usize::from(min % square != 0)) * (square);
        while k <= max {
            numbers[k - min] = false;
            k += square;
        }
        i += 1;
    }

    numbers.iter().enumerate().filter(|(_, &b)| b).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|str| str.parse().unwrap());

    let min = input.next().unwrap();
    let max = input.next().unwrap();

    println!("{}", get_numbers(min, max));
}
