use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Input {
    pub filepath: &'static str,
    pub data: Vec<i32>,
}

impl Input {
    pub fn read_data(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(self.filepath)?;
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            let line = line?;
            let number: i32 = line.parse()?;
            self.data.push(number);
        }
        Ok(())
    }
}

fn part_1(data: &Vec<i32>) -> i32 {
    let mut result = 0;
    for number in data {
        result += get_fuel(*number);
    }
    result
}

fn part_2(data: &Vec<i32>) -> i32 {
    let mut result = 0;
    for number in data {
        let mut current = *number;
        while get_fuel(current) > 0 {
            let next = get_fuel(current);
            result += next;
            current = next;
        }
    }
    result
}

fn get_fuel(fuel: i32) -> i32 {
    fuel / 3 - 2
}

pub fn run() {
    let filepath = "input/day1.txt";
    let mut input = Input {
        filepath,
        data: vec![],
    };
    input.read_data().expect("Failed to load input data");
    let result_1 = part_1(&input.data);
    let result_2 = part_2(&input.data);

    println!("part 1 result is {}", result_1);
    println!("part 2 result is {}", result_2);
}
