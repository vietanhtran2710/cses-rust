use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let mut items = line.split_whitespace().into_iter();
    let _n: usize = items.next().unwrap().parse().unwrap();
    let q: usize = items.next().unwrap().parse().unwrap();

    line = "".to_string(); input.read_line(&mut line).unwrap();
    let arr: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut sum: Vec<usize> = Vec::new(); let mut s: usize = 0;
    for _i in 0..arr.len() {
        s += arr[_i] as usize; sum.push(s);
    }

    for _i in 0..q {
        line = "".to_string(); input.read_line(&mut line).unwrap();
        let mut items = line.split_whitespace().into_iter();
        let a: usize = items.next().unwrap().parse().unwrap();
        let b: usize = items.next().unwrap().parse().unwrap();
        if a == 1 {
            println!("{}", sum[b - 1]);
        }
        else {
            println!("{}", sum[b - 1] - sum[a - 2]);
        }
    }
}   