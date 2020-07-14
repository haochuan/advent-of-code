use regex::Regex;
use std::fs;

struct Lights {
    grid: Vec<Vec<bool>>,
}

impl Lights {
    fn new() -> Self {
        let row: Vec<bool> = vec![false; 1000];
        let grid: Vec<Vec<bool>> = vec![row; 1000];
        Self { grid }
    }

    fn read_instruction<'a>(&self, instruction: &'a str) -> (&'a str, Vec<usize>, Vec<usize>) {
        let re = Regex::new(r"(on|off|toggle)\s(\d{1,3},\d{1,3}).*\s(\d{1,3},\d{1,3})").unwrap();
        let caps = re.captures(instruction).unwrap();
        let command = caps.get(1).map_or("", |m| m.as_str());
        let start: Vec<usize> = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let end: Vec<usize> = caps
            .get(3)
            .map_or("", |m| m.as_str())
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        (command, start, end)
    }

    pub fn execution(&mut self, instruction: &str) {
        let (command, start, end) = self.read_instruction(instruction);
        for i in start[0]..end[0] + 1 {
            for j in start[1]..end[1] + 1 {
                match command {
                    "on" => self.grid[i][j] = true,
                    "off" => self.grid[i][j] = false,
                    "toggle" => self.grid[i][j] = !self.grid[i][j],
                    _ => (),
                }
            }
        }
    }
    pub fn check_on(&self) -> i32 {
        let mut counter = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] {
                    counter += 1;
                }
            }
        }
        counter
    }
}

struct Lights2 {
    grid: Vec<Vec<u32>>,
}

impl Lights2 {
    fn new() -> Self {
        let row: Vec<u32> = vec![0; 1000];
        let grid: Vec<Vec<u32>> = vec![row; 1000];
        Self { grid }
    }

    fn read_instruction<'a>(&self, instruction: &'a str) -> (&'a str, Vec<usize>, Vec<usize>) {
        let re = Regex::new(r"(on|off|toggle)\s(\d{1,3},\d{1,3}).*\s(\d{1,3},\d{1,3})").unwrap();
        let caps = re.captures(instruction).unwrap();
        let command = caps.get(1).map_or("", |m| m.as_str());
        let start: Vec<usize> = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let end: Vec<usize> = caps
            .get(3)
            .map_or("", |m| m.as_str())
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        (command, start, end)
    }

    pub fn execution(&mut self, instruction: &str) {
        let (command, start, end) = self.read_instruction(instruction);
        for i in start[0]..end[0] + 1 {
            for j in start[1]..end[1] + 1 {
                match command {
                    "on" => self.grid[i][j] += 1,
                    "off" => {
                        if self.grid[i][j] != 0 {
                            self.grid[i][j] -= 1;
                        }
                    }
                    "toggle" => self.grid[i][j] += 2,
                    _ => (),
                }
            }
        }
    }
    pub fn check_brightness(&self) -> u32 {
        let mut counter = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                counter += self.grid[i][j];
            }
        }
        counter
    }
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> i32 {
    let mut lights = Lights::new();
    for l in input.trim().lines() {
        lights.execution(l);
    }
    lights.check_on()
}

pub fn part_2(input: &str) -> i32 {
    let mut lights = Lights2::new();
    for l in input.trim().lines() {
        lights.execution(l);
    }
    lights.check_brightness() as i32
}

pub fn run() {
    let input = get_input("problems/day6/input.txt");
    let part1 = part_1(&input);
    println!("result for part 1: {}", part1);
    let part2 = part_2(&input);
    println!("result for part 2: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_works() {
        let mut lights = Lights::new();
        lights.execution("turn on 0,0 through 999,999");
        assert_eq!(lights.check_on(), 1000 * 1000);
        lights.execution("toggle 0,0 through 999,0");
        assert_eq!(lights.check_on(), 1000 * 1000 - 1000);
        lights.execution("turn off 499,499 through 500,500");
        assert_eq!(lights.check_on(), 1000 * 1000 - 1000 - 4);
    }
    #[test]
    fn part_2_works() {}
}
