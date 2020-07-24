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
    let data: Vec<usize> = buffer
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let result = execute(&data, 12, 2);
    println!("{}", result);
    Ok(())
}

pub fn run_part_2<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let data: Vec<usize> = buffer
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            if execute(&data, noun, verb) == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
            }
        }
    }

    Ok(())
}

fn execute(data: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut data_current = data.clone();
    data_current[1] = noun;
    data_current[2] = verb;
    let mut i = 0; // index
    while i < data_current.len() - 3 && data_current[i] != 99 {
        let pos1 = data_current[i + 1];
        let pos2 = data_current[i + 2];
        let pos3 = data_current[i + 3];
        if data_current[i] == 1 {
            data_current[pos3] = data_current[pos1] + data_current[pos2];
        } else if data_current[i] == 2 {
            data_current[pos3] = data_current[pos1] * data_current[pos2];
        }
        i += 4;
    }
    data_current[0]
}
