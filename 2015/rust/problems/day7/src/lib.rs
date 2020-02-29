use std::collections::HashMap;
use std::fs;

struct Circuit<'a> {
    map: HashMap<String, u16>,
    queue: Vec<&'a String>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Circuit {
            map: HashMap::new(),
            queue: Vec::new(),
        }
    }
    // how to parse either number or key of hashmap?
    fn get_value(&mut self, input: String) -> u16 {
        if input.parse::<u16>().is_ok() {
            input.parse().unwrap()
        } else {
            if let Some(value) = self.map.get(&input) {
                *value
            } else {
                self.queue.push(&input);
                0
            }
        }
    }

    fn flush(&mut self) {
        if self.queue.len() != 0 {
            for l in self.queue.clone() {
                self.read(l.to_string());
            }
        }
    }

    fn connect(&mut self, target: String, value: u16) {
        println!("connect {} to {}", value, target);
        self.map.insert(target, value);
    }

    fn read(&mut self, instruction: String) {
        let instruction: Vec<String> = instruction
            .split("->")
            .map(|s| s.trim().to_string())
            .collect();

        let target = instruction[1].clone();
        let commands = instruction[0].clone();
        let commands_arr: Vec<String> = commands.split(" ").map(|s| s.trim().to_string()).collect();

        match commands_arr.len() {
            1 => {
                // 321 -> x
                let c: &String = &commands_arr[0];
                let value = self.get_value(c.to_string());
                self.connect(target, value);
            }
            2 => {
                // NOT x -> h
                let c: &String = &commands_arr[0];
                let mut value = self.get_value(c.to_string());
                value = !value;
                self.connect(target, value);
            }
            3 => {
                // x AND y -> d
                // x OR y -> e
                // x LSHIFT 2 -> f
                // y RSHIFT 2 -> g
                let operator = &commands_arr[1];
                if operator == "AND" {
                    let c1: &String = &commands_arr[0];
                    let c2: &String = &commands_arr[2];
                    let a = self.get_value(c1.to_string());
                    let b = self.get_value(c2.to_string());
                    let value = a & b;
                    self.connect(target, value);
                } else if operator == "OR" {
                    let c1: &String = &commands_arr[0];
                    let c2: &String = &commands_arr[2];
                    let a = self.get_value(c1.to_string());
                    let b = self.get_value(c2.to_string());
                    let value = a | b;
                    self.connect(target, value);
                } else if operator == "LSHIFT" {
                    let c: &String = &commands_arr[0];
                    let source_value = self.get_value(c.to_string());
                    let steps: u16 = commands_arr[2].parse().unwrap();
                    let value = source_value << steps;
                    self.connect(target, value);
                } else {
                    let c: &String = &commands_arr[0];
                    let source_value = self.get_value(c.to_string());
                    let steps: u16 = commands_arr[2].parse().unwrap();
                    let value = source_value >> steps;
                    self.connect(target, value);
                }
            }
            _ => (),
        }
    }
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> u16 {
    let mut curcuit = Circuit::new();
    for l in input.lines() {
        curcuit.read(l.to_string());
    }
    *curcuit.map.get("a").unwrap()
}

pub fn part_2(input: &str) -> u16 {
    1
}

pub fn run() {
    let input = get_input("problems/day7/input.txt");
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
        let mut curcuit = Circuit::new();
        curcuit.read("123 -> x".to_string());
        curcuit.read("456 -> y".to_string());
        curcuit.read("x AND y -> d".to_string());
        curcuit.read("x OR y -> e".to_string());
        curcuit.read("x LSHIFT 2 -> f".to_string());
        curcuit.read("y RSHIFT 2 -> g".to_string());
        curcuit.read("NOT x -> h".to_string());
        curcuit.read("NOT y -> i".to_string());
        assert_eq!(*curcuit.map.get("d").unwrap(), 72);
        assert_eq!(*curcuit.map.get("e").unwrap(), 507);
        assert_eq!(*curcuit.map.get("f").unwrap(), 492);
        assert_eq!(*curcuit.map.get("g").unwrap(), 114);
        assert_eq!(*curcuit.map.get("h").unwrap(), 65412);
        assert_eq!(*curcuit.map.get("i").unwrap(), 65079);
        assert_eq!(*curcuit.map.get("x").unwrap(), 123);
        assert_eq!(*curcuit.map.get("y").unwrap(), 456);
    }
    #[test]
    fn part_2_works() {}
}
