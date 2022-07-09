use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: u128 = line.strip_suffix("\n").unwrap().parse().unwrap();
    let mut sum: u128 = n * (n + 1) / 2;
    line = "".to_string(); input.read_line(&mut line).unwrap();
    for item in line.split_whitespace() {
        sum -= item.parse::<u128>().unwrap()
    }
    print!("{}", sum);
}