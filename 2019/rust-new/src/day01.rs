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
    run_part(&mut reader, part_one)?;

    //part 2
    let mut reader = io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;
    Ok(())
}

pub fn run_part<F, T>(input: &mut T, func: F) -> Result<(), Error>
where
    F: Fn(usize) -> usize,
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut total = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        let n = buffer.trim().parse::<usize>()?;

        let fuel = func(n);
        total += fuel;

        buffer.clear();
    }
    println!("{}", total);
    Ok(())
}

fn part_one(n: usize) -> usize {
    match (n / 3).checked_sub(2) {
        Some(i) => i,
        None => 0,
    }
}

fn part_two(mut n: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (n / 3).checked_sub(2) {
            Some(i) => i,
            None => break total,
        };
        total += m;
        n = m;
    }
}
