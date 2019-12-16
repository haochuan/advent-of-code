use std::collections::HashSet;
use std::fs;

fn get_input(filepath: &str) -> (i32, i32) {
    let content = fs::read_to_string(filepath).expect("Cannnot read input file");
    let numbers: Vec<i32> = content
        .split('-')
        .map(|s| s.trim().parse().expect("failed to parse in put"))
        .collect();
    (numbers[0], numbers[1])
}

fn modify_digit(mut number: i32, digit: u8, modifier: i32) -> i32 {
    println!("old number: {}", number);
    let mut tmp = number;
    let mut count: u8 = 1;
    let base: i32 = 10;

    while tmp / 10 != 0 {
        count += 1;
        tmp = tmp / 10;
    }
    // number has {count} digits

    number = number + (modifier * base.pow((count - digit).into()));
    println!("new number: {}", number);
    println!("======");
    number
}

fn is_valid(number: i32) -> i32 {
    let mut double = false;
    let mut min_number = 0;
    let map: HashSet<char> = HashSet::new();
    let number_string = number.to_string();
    for (i, c) in number_string.chars().enumerate() {
        if map.contains(&c) {
            double = true;
        }
        if c.to_digit(10).unwrap() > min_number {
            min_number = c.to_digit(10).unwrap();
        } else {
            return i as i32;
        }
    }

    if double {
        -1
    } else {
        number_string.len() as i32 - 1
    }
}

pub fn part_1() {
    let (mut start, mut end) = get_input("input/day4.txt");
    let mut count = 0;
    while start <= end {
        let result = is_valid(start);
        if result == -1 {
            count += 1;
            start += 1;
        } else {
            start = modify_digit(start, result as u8 + 1, 1);
        }
    }
}
