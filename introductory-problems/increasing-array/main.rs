use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    line = "".to_string(); input.read_line(&mut line).unwrap();
    let mut count = 0_u128;
    let mut arr = line.split_whitespace();
    let mut current = arr.next().unwrap().parse::<u128>().unwrap();
    loop {
        let next_number = arr.next();
        if next_number.is_none() {
            break;
        }
        let number = next_number.unwrap().parse::<u128>().unwrap();
        if number < current {
            count += current - number;
        }
        else {
            current = number;
        }
    }
    print!("{}", count);
}