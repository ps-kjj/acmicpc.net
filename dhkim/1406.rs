use std::io;

fn main() {
    let mut txt = String::new();
    let mut input = String::new();

    io::stdin().read_line(&mut txt).unwrap(); // text
    let mut head: Vec<char> = txt.trim().chars().collect();
    let mut tail: Vec<char> = Vec::new();

    io::stdin().read_line(&mut input).unwrap(); // M

    let n: usize = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut cmd = input.split_whitespace();

        match cmd.next().unwrap() {
            "L" => if let Some(ch) = head.pop() {
                tail.push(ch)
            },
            "D" => if let Some(ch) = tail.pop() {
                head.push(ch)
            },
            "B" => {
                head.pop();
            },
            "P" => {
                head.push(cmd.next().unwrap().chars().next().unwrap())
            },
            _ => { }
        }
    }

    tail.reverse();
    head.append(&mut tail);
    let res: String = head.iter().collect();
    println!("{res}");
}
