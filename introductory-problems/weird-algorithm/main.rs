use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut n: u64 = line.strip_suffix("\n").unwrap().parse().unwrap();
    print!("{} ", n);
    while n != 1 {
        if n % 2 == 0 { n /= 2; }
        else { n = n * 3 + 1; }
        print!("{} ", n);
    }
}