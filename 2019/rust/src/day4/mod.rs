use std::cmp;
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
    let mut tmp = number;
    let mut count: u8 = 1;
    let base: i32 = 10;

    while tmp / 10 != 0 {
        count += 1;
        tmp = tmp / 10;
    }
    // number has {count} digits

    number = number + (modifier * base.pow((count - digit).into()));
    println!("number has {} digits", { count });
    println!("number has {} digits", { number });
    number
}

pub fn part_1() {
    let (mut start, mut end) = get_input("input/day4.txt");
    start = modify_digit(start, 1, 1);
    println!("{}", start);
}
