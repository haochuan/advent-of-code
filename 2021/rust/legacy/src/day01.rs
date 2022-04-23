use crate::error::Error;
use std::io;

pub fn run<T>(mut input: T) -> Result<(), Error>
where
    T: io::BufRead,
{
    println!("here");
    let mut content = Vec::new();
    println!("22");
    println!("{:?}", input);
    input.read_to_end(&mut content)?;

    println!("1");

    //part 1
    let mut reader = io::BufReader::new(&content[..]);
    part_one(&mut reader)?;

    println!("2");

    //part 2
    let mut reader = io::BufReader::new(&content[..]);
    part_two(&mut reader)?;

    println!("3");
    Ok(())
}

fn part_one<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut list = Vec::new();
    let mut result = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        let n = buffer.trim().parse::<i32>()?;
        list.push(n);

        buffer.clear();
    }

    for (i, _) in list.iter().enumerate() {
        if i != list.len() - 1 {
            if list[i] - list[i + 1] < 0 {
                result += 1;
            }
        }
    }
    println!("Part 1: {}", result);
    Ok(())
}

fn part_two<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut list = Vec::new();
    let mut result = 0;
    let mut previous = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        let n = buffer.trim().parse::<i32>()?;
        list.push(n);

        buffer.clear();
    }

    for i in 2..list.len() {
        let current = list[i] + list[i - 1] + list[i - 2];
        if previous != 0 && current > previous {
            result += 1;
        }
        previous = current;
    }

    println!("Part 2: {}", result);
    Ok(())
}
