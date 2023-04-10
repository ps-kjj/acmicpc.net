use std::io;

fn main() {
    for _ in 0..3 {
        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();

        let data = data.trim().split(" ");
        let mut cnt: u32 = 0;
        for item in data {
            if item == "0" {
                cnt += 1;
            }
        }

        match cnt {
            0 => println!("E"),
            1 => println!("A"),
            2 => println!("B"),
            3 => println!("C"),
            4 => println!("D"),
            _ => (),
        }
    }
}