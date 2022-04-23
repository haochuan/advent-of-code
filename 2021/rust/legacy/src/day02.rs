use crate::error::Error;
use itertools::Itertools;
use std::io;

pub fn run<T>(mut input: T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    //part 1
    let mut reader = io::BufReader::new(&content[..]);
    part_one(&mut reader)?;

    //part 2
    let mut reader = io::BufReader::new(&content[..]);
    part_two(&mut reader)?;
    Ok(())
}

fn part_one<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut horizontal = 0;
    let mut depth = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        if let Some((command, value)) = buffer.trim().split_whitespace().collect_tuple() {
            match command {
                "forward" => horizontal += value.parse::<i32>()?,
                "down" => depth += value.parse::<i32>()?,
                "up" => depth -= value.parse::<i32>()?,
                _ => (),
            }
            if command == "forward" {}
        }

        buffer.clear();
    }

    println!("Part 1: {}", horizontal * depth);
    Ok(())
}

fn part_two<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        if let Some((command, value)) = buffer.trim().split_whitespace().collect_tuple() {
            match command {
                "forward" => {
                    horizontal += value.parse::<i32>()?;
                    depth += value.parse::<i32>()? * aim;
                }
                "down" => aim += value.parse::<i32>()?,
                "up" => aim -= value.parse::<i32>()?,
                _ => (),
            }
            if command == "forward" {}
        }

        buffer.clear();
    }

    println!("Part 2: {}", horizontal * depth);
    Ok(())
}
