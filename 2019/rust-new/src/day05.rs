use crate::error::Error;
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
    let mut data: Vec<i32> = buffer
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut index = 0;
    let mut memery = 1;
    while index < data.len() {
        // deal with single digit and multi digit
        // 1, 2, 3, 4
        // 101, 102, 103, 104,
        // 1001, 1002, 1003, 1004,
        // 1101, 1102, 1103, 1104,
        // 10001...
        // 10101...
        // 11001...
        // 11101...
        // Parameters that an instruction writes to will never be in immediate mode.
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
            99 => {
                break;
            }
            _ => break,
        }
    }
    println!("Part 1: Last output before hault is: {}", memery);
    Ok(())
}
pub fn run_part_2<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let mut data: Vec<i32> = buffer
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    // println!("{:?}", data);

    let mut index = 0;
    let mut memery = 5;
    while index < data.len() {
        // deal with single digit and multi digit
        // 1, 2, 3, 4
        // 101, 102, 103, 104,
        // 1001, 1002, 1003, 1004,
        // 1101, 1102, 1103, 1104,
        // 10001...
        // 10101...
        // 11001...
        // 11101...
        // Parameters that an instruction writes to will never be in immediate mode.
        let instruction = format!("{:0>4}", data[index]);
        let opcode = instruction[2..].parse::<i32>().unwrap();
        let mode_1 = instruction[1..2].parse::<i32>().unwrap();
        let mode_2 = instruction[0..1].parse::<i32>().unwrap();

        // println!("instruction: {}, opcode: {}", instruction, opcode);

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
        // println!("{:?}", data);
    }
    println!("Part 2: Last output before hault is: {}", memery);
    Ok(())
}

fn get_value(data: &Vec<i32>, mode: i32, index: usize) -> i32 {
    if mode == 0 {
        data[index]
    } else {
        index as i32
    }
}
