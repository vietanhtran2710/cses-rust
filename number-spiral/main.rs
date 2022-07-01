use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let t: u128 = line.strip_suffix("\n").unwrap().parse().unwrap();
    for _i in 0..t {
        line = "".to_string(); input.read_line(&mut line).unwrap();
        let item: Vec<u128> = line.split_whitespace().map(|x| x.parse::<u128>().unwrap()).collect();
        let a = item[0]; let b = item[1];
        let max = a.max(b);
        let value = (max - 1).pow(2) + 1;
        let row: u128; let col: u128;
        if max % 2 == 0 {
            row = 1; col = max;
            println!("{}", value + (a - row) + (col - b));
        }
        else {
            row = max; col = 1;
            println!("{}", value + (row - a) + (b - col));
        }
    }
}