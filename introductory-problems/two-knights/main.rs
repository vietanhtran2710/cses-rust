use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    println!("0");
    let mut attack_sum: usize = 0;
    for _i in 2..=n {
        attack_sum += 8 * _i - 16;
        println!("{}", _i.pow(2) * (_i.pow(2) - 1) / 2 - attack_sum);
    }
}