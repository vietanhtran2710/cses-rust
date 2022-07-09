use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: u128 = line.strip_suffix("\n").unwrap().parse().unwrap();
    if n == 2 || n == 3 {
        print!("NO SOLUTION");
    }
    else {
        for i in (2..=n).step_by(2) {
            print!("{} ", i);
        }
        for i in (1..=n).step_by(2) {
            print!("{} ", i);
        }
    }
}