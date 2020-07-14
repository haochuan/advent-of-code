use std::fs;
use std::io::{self, BufRead};

pub fn run() -> usize {
    let file = fs::File::open("data/01.txt").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = String::new();
    let mut result = 0;

    loop {
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }
        let n = buffer.trim().parse::<usize>().unwrap();

        result += match (n / 3).checked_sub(2) {
            Some(i) => i,
            None => 0,
        };

        buffer.clear();
    }
    println!("{}", result);
    42
}
