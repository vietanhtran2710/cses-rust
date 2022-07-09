use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let (mut max, mut length) = (0_u128, 1_u128);
    let mut chs = line.chars();
    let mut current = chs.next().unwrap();
    loop { 
        let new_char = chs.next();
        if new_char.is_none() {
            break;
        }
        if new_char.unwrap() == current {
            length += 1;
        }
        else {
            if max < length {
                max = length;
            }
            length = 1;
        }
        current = new_char.unwrap();
    }
    if max < length { max = length};
    print!("{}", max);
}