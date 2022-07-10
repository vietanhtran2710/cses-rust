use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().unwrap();
    for _i in 0..n {
        line = "".to_string(); input.read_line(&mut line).unwrap();
        let mut items = line.split_whitespace().into_iter();
        let a: u64 = items.next().unwrap().parse().unwrap();
        let b: u64 = items.next().unwrap().parse().unwrap();
        if (a + b) % 3 != 0 {
            println!("NO");
            continue;
        }
        let (min, max) = (a.min(b), a.max(b));
        if (min - (max - min) + max - 2 * (max - min)) % 3 != 0 {
            println!("NO");
        }
        else {
            println!("YES");
        }
    }
}