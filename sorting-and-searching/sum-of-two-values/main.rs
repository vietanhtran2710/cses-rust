use std::{io::{BufRead, BufReader}, collections::HashMap};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let mut items = line.split_whitespace().into_iter();
    items.next(); let sum: usize = items.next().unwrap().parse().unwrap();
    line = "".to_string(); input.read_line(&mut line).unwrap();
    let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut index: HashMap<u32, usize> = HashMap::new();
    let mut found = false;
    for _i in 0..numbers.len() {
        if index.contains_key(&(sum as u32 - numbers[_i])) {
            print!("{} {}", index.get(&(sum as u32 - numbers[_i])).unwrap() + 1, _i + 1);
            found = true;
            break;
        }
        if !index.contains_key(&numbers[_i]) {
            index.insert(numbers[_i], _i);
        }
    }
    if !found {
        print!("IMPOSSIBLE");
    }   
}