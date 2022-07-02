use std::io::{BufRead, BufReader};

fn power_mod(b: u128) -> u128 {
    if b <= 64 {
        2_u128.pow(b as u32)
    }
    else {
        let _const = 10_u128.pow(9) + 7;
        if b % 2 == 0 {
            (power_mod(b / 2) % _const * power_mod(b / 2) % _const) % _const
        }
        else {
            (power_mod(b / 2) % _const * power_mod(b / 2 + 1) % _const) % _const
        }
    }
}

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: u128 = line.strip_suffix("\n").unwrap().parse().unwrap();
    print!("{}", power_mod(n));
}