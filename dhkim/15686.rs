// 15686 치킨 배달
use std::io;

type Point = (usize, usize);

enum Type {
    EMPTY,
    HOUSE,
    CHICKEN,
}

impl From<&usize> for Type {
    fn from(val: &usize) -> Self {
        match val {
            0 => Type::EMPTY,
            1 => Type::HOUSE,
            2 => Type::CHICKEN,
            _ => panic!(),
        }
    }
}

struct ChickenDelivery {
    city: Vec<Vec<usize>>,
    chickens: Vec<Point>,
    houses: Vec<Point>,
    choosed: Vec<Point>,
    n: usize,
    m: usize,
}

impl ChickenDelivery {
    fn new() -> Self {
        ChickenDelivery {
            city: Vec::new(),
            chickens: Vec::new(),
            houses: Vec::new(),
            choosed: Vec::new(),
            n: 0,
            m: 0,
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input.split_whitespace().map(|str| str.parse().unwrap());
        self.n = input.next().unwrap();
        self.m = input.next().unwrap();

        for _ in 0..(self.n) {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            self.city.push(
                input
                    .split_whitespace()
                    .map(|str| str.parse().unwrap())
                    .collect(),
            );
        }

        self.init();
    }

    fn init(&mut self) {
        for (r, row) in self.city.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                match Type::from(col) {
                    Type::HOUSE => self.houses.push((r, c)),
                    Type::CHICKEN => {
                        self.chickens.push((r, c));
                    },
                    _ => (),
                }
            }
        }
    }

    fn op(&mut self, index: usize, ans: &mut usize) {
        if self.choosed.len() == self.m {
            let res = self.calc_distance();
            
            if &res < ans { *ans = res };

            return;
        }

        for i in index..(self.chickens.len()) {
            self.choosed.push(self.chickens[i]);
            self.op(i + 1, ans);
            self.choosed.pop();
        }
    }

    fn calc_distance(&self) -> usize {
        let mut dist = 0usize;

        for (house_r, house_c) in &self.houses {
            let mut min = usize::MAX;
            for &(chicken_r, chicken_c) in &self.choosed {
                min = min.min(house_r.abs_diff(chicken_r) + house_c.abs_diff(chicken_c));
            }
            dist += min;
        }

        dist
    }
}

fn main() {
    let mut cd = ChickenDelivery::new();
    cd.input();

    let mut ans = usize::MAX;
    cd.op(0, &mut ans);
    println!("{ans}");
}
