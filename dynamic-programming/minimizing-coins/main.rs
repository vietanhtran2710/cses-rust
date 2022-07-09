use std::{io::{BufRead, BufReader}};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let mut items = line.split_whitespace().into_iter();
    items.next(); let x: u32 = items.next().unwrap().parse().unwrap();
    line = "".to_string(); input.read_line(&mut line).unwrap();
    let coins: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut count: Vec<i64> = vec![-1; x as usize + 1];
    count[0] = 0;
    for _i in 0..x as usize {
        if count[_i] != -1 {
            for coin in &coins {
                if _i as u32 + *coin <= x {
                    if count[_i + *coin as usize] == -1 || count[_i + *coin as usize] > count[_i] + 1 {
                        count[_i + *coin as usize] = count[_i] + 1;
                    }
                }
            }
        }
    }
    print!("{}", count[x as usize]);
}