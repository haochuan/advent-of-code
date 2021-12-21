use crate::bail;
use crate::error::Error;
use itertools::Itertools;
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
    // let mut reader = io::BufReader::new(&content[..]);
    // run_part_2(&mut reader)?;
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
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // get all phase combinations -> get all permutations of 01234
    let mut max = 0;
    // let permutations = (0..=4).permutations(5);
    // for item in permutations.into_iter() {
    //     let mut input = 0;
    //     for i in item {
    //         input = calculate(&data, input, i)?;
    //     }
    //     if input > max {
    //         max = input;
    //     }
    // }
    // println!("{:?}", max);
    let permutations = vec![4, 3, 2, 1, 0];
    let mut input = 0;
    for i in permutations {
        input = calculate(&data, input, i)?;
        println!("input: {}", input);
    }
    if input > max {
        max = input;
    }
    println!("{:?}", max);
    Ok(())
}

pub fn calculate(data: &Vec<i32>, input: i32, phase: i32) -> Result<i32, Error> {
    let mut data = data.clone();
    println!("data: {:?}", data);

    let mut index = 0;
    let mut memery = input;
    data[0] = phase;
    // data[1] = input;
    println!("data: {:?}", data);
    while index < data.len() {
        let instruction = format!("{:0>4}", data[index]);
        let opcode = instruction[2..].parse::<i32>().unwrap();
        let mode_1 = instruction[1..2].parse::<i32>().unwrap();
        let mode_2 = instruction[0..1].parse::<i32>().unwrap();

        match opcode {
            1 | 2 => {
                let first = get_value(&data, mode_1, data[index + 1] as usize);
                let second = get_value(&data, mode_2, data[index + 2] as usize);
                let target_index = data[index + 3] as usize;
                if opcode == 1 {
                    data[target_index] = first + second;
                } else {
                    data[target_index] = first * second;
                }
                index += 4;
            }
            3 => {
                let target_index = data[index + 1] as usize;
                data[target_index] = memery;
                index += 2;
            }
            4 => {
                let target_index = data[index + 1] as usize;
                memery = data[target_index];
                index += 2;
            }
            5 | 6 => {
                let first = get_value(&data, mode_1, data[index + 1] as usize) as usize;
                if opcode == 5 && first != 0 {
                    index = get_value(&data, mode_2, data[index + 2] as usize) as usize;
                } else if opcode == 6 && first == 0 {
                    index = get_value(&data, mode_2, data[index + 2] as usize) as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let first = get_value(&data, mode_1, data[index + 1] as usize);
                let second = get_value(&data, mode_2, data[index + 2] as usize);
                let target_index = data[index + 3] as usize;
                if first < second {
                    data[target_index] = 1;
                } else {
                    data[target_index] = 0;
                }
                index += 4;
            }
            8 => {
                let first = get_value(&data, mode_1, data[index + 1] as usize);
                let second = get_value(&data, mode_2, data[index + 2] as usize);
                let target_index = data[index + 3] as usize;
                if first == second {
                    data[target_index] = 1;
                } else {
                    data[target_index] = 0;
                }
                index += 4;
            }
            99 => {
                break;
            }
            _ => break,
        }
    }
    Ok(memery)
}

fn get_value(data: &Vec<i32>, mode: i32, index: usize) -> i32 {
    if mode == 0 {
        data[index]
    } else {
        index as i32
    }
}
