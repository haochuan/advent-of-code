use std::collections::HashMap;
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
    // println!("old number: {}", number);
    let mut tmp = number;
    let mut count: u8 = 1;
    let base: i32 = 10;

    while tmp / 10 != 0 {
        count += 1;
        tmp = tmp / 10;
    }
    // number has {count} digits

    number = number + (modifier * base.pow((count - digit).into()));
    // println!("new number: {}", number);
    // println!("======");
    number
}

fn is_valid_part_1(number: i32) -> (bool, u8) {
    let mut double = false;
    let mut min_number = 0;
    let mut map: HashSet<char> = HashSet::new();
    let number_string = number.to_string();
    for (i, c) in number_string.chars().enumerate() {
        if map.contains(&c) {
            double = true;
        } else {
            map.insert(c);
        }
        if c.to_digit(10).unwrap() >= min_number {
            min_number = c.to_digit(10).unwrap();
        } else {
            return (false, i as u8 + 1);
        }
    }

    if double {
        (true, 0)
    } else {
        (false, number_string.len() as u8 - 1)
    }
}

fn is_valid_part_2(number: i32) -> (bool, u8) {
    let mut min_number = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    let number_string = number.to_string();
    for (i, c) in number_string.chars().enumerate() {
        match map.get(&c) {
            Some(_) => {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
            None => {
                let count = map.entry(c).or_insert(1);
            }
        }
        if c.to_digit(10).unwrap() >= min_number {
            min_number = c.to_digit(10).unwrap();
        } else {
            return (false, i as u8 + 1);
        }
    }

    for (_, val) in map.iter() {
        if *val == 2 {
            return (true, 0);
        }
    }

    (false, 6)
}

pub fn part_1() {
    let (mut start, end) = get_input("input/day4.txt");
    let mut count = 0;
    while start <= end {
        let (result, index) = is_valid_part_1(start);
        if result {
            count += 1;
            start += 1;
        } else {
            start = modify_digit(start, index, 1);
        }
    }
    println!("there are {} different password.", count);
}

pub fn part_2() {
    let (mut start, end) = get_input("input/day4.txt");
    let mut count = 0;
    while start <= end {
        let (result, index) = is_valid_part_2(start);
        if result {
            count += 1;
            start += 1;
        } else {
            start = modify_digit(start, index, 1);
        }
    }
    println!("there are {} different password.", count);
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn modify_digit_works() {
        assert_eq!(modify_digit(183564, 3, 1), 184564);
        assert_eq!(modify_digit(189564, 3, 1), 190564);
    }
    #[test]
    fn is_valid_part_2_works() {
        assert_eq!(is_valid_part_2(112233), (true, 0));
        assert_eq!(is_valid_part_2(123444), (false, 6));
        assert_eq!(is_valid_part_2(111122), (true, 0));
    }
}
