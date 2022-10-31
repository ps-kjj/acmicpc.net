// 6603 로또
use std::collections::BTreeSet;
use std::fmt::Write;
use std::io;

fn op(v: &[usize], set: &mut BTreeSet<usize>, output: &mut String) {
    if set.len() == 6 {
        for num in set.iter() {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();
        return;
    }

    for (idx, &num) in v.iter().enumerate() {
        set.insert(num);
        op(&v[(idx + 1)..], set, output);
        set.remove(&num);
    }
}

fn main() {
    let mut output = String::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let v: Vec<usize> = input
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|i| i.parse().unwrap())
            .collect();
        
        let mut s = BTreeSet::new();

        op(&v, &mut s, &mut output);
        writeln!(output, "").unwrap();

        if v.is_empty() { break; }
    }
    
    println!("{output}");
}
