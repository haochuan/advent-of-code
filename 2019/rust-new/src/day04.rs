use crate::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

pub fn run<T>(mut input: T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    //part 1
    let mut reader = io::BufReader::new(&content[..]);
    run_part_1(&mut reader)?;

    // part 2
    let mut reader = io::BufReader::new(&content[..]);
    run_part_2(&mut reader)?;
    Ok(())
}

pub fn run_part_1<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let data: Vec<i32> = buffer
        .trim()
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut count = 1;
    let mut start = data[0];
    let end = data[1];

    while start <= end {
        let (result, index) = is_valid_part_1(start);
        if result {
            count += 1;
            start += 1;
        } else {
            start = modify_digit(start, index, 1);
        }
    }
    println!("part 1: {}", count);
    Ok(())
}
pub fn run_part_2<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let data: Vec<i32> = buffer
        .trim()
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut count = 1;
    let mut start = data[0];
    let end = data[1];

    while start <= end {
        let (result, index) = is_valid_part_2(start);
        if result {
            count += 1;
            start += 1;
        } else {
            start = modify_digit(start, index, 1);
        }
    }
    println!("part 2: {}", count);
    Ok(())
}

// modify a sinle digit {digit} in i32
fn modify_digit(mut number: i32, digit: u8, modifier: i32) -> i32 {
    let mut tmp = number;
    let mut count: u8 = 1;
    let base: i32 = 10;

    while tmp / 10 != 0 {
        count += 1;
        tmp = tmp / 10;
    }

    number = number + (modifier * base.pow((count - digit).into()));
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
                let _count = map.entry(c).or_insert(1);
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
