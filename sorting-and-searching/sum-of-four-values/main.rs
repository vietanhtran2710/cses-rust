use std::{io::{BufRead, BufReader}, collections::HashMap};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string(); input.read_line(&mut line).unwrap();
    let mut items = line.split_whitespace().into_iter();
    items.next(); let sum: usize = items.next().unwrap().parse().unwrap();
    line = "".to_string(); input.read_line(&mut line).unwrap();
    let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut index: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    let mut found = false;
    if numbers.len() < 4 {
        print!("IMPOSSIBLE");
        return;
    }
    for _i in 0..numbers.len() - 1 {
        for _j in _i + 1..numbers.len() {
            if !index.contains_key(&(numbers[_i] + numbers[_j])) {
                index.insert(numbers[_i] + numbers[_j], [(_i, _j)].to_vec());
            }
            else {
                index.get_mut(&(numbers[_i] + numbers[_j])).unwrap().push((_i, _j));
            }
            if numbers[_i] + numbers[_j] < sum as u32 && index.contains_key(&(sum as u32 - numbers[_i] - numbers[_j])) {
                for pair in index.get(&(sum as u32 - numbers[_i] - numbers[_j])).unwrap() {
                    if pair.0 != _i && pair.1 != _j && pair.0 != _j && pair.1 != _i {
                        print!("{} {} {} {}", pair.0 + 1, pair.1 + 1, _i + 1, _j + 1);
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }
    if !found {
        print!("IMPOSSIBLE");
    }   
}