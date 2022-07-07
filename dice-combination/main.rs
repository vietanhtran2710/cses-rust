use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    let mut dp: Vec<u32> = vec![0; n + 1];
    dp[0] = 1;
    for _i in 0..n {
        for delta in 1..=6.min(n - _i) {
            dp[_i + delta] = (dp[_i + delta] + dp[_i]) % (10_u32.pow(9) + 7);
        }
    }
    print!("{}", dp[n]);
}