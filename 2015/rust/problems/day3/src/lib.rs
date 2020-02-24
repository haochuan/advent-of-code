use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn move_to(&mut self, dir: char) {
        match dir {
            '^' => self.y -= 1,
            'v' => self.y += 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }
}

struct Santa {
    current_location: Point,
    visited: HashMap<Point, bool>,
}

impl Santa {
    fn new() -> Self {
        Santa {
            current_location: Point::new(0, 0),
            visited: HashMap::new(),
        }
    }

    fn move_to(&mut self, dir: char) {
        self.current_location.move_to(dir);
        self.drop_gift();
    }

    fn drop_gift(&mut self) {
        let key = self.current_location;
        match self.visited.get(&key) {
            None => {
                self.visited.insert(key, true);
                ()
            }
            _ => (),
        }
    }
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

fn merge_hash(map1: HashMap<Point, bool>, map2: HashMap<Point, bool>) -> HashMap<Point, bool> {
    map1.into_iter().chain(map2).collect()
}

pub fn part_1(input: &str) -> u32 {
    let mut santa = Santa::new();
    santa.drop_gift();
    for c in input.chars() {
        santa.move_to(c);
    }
    santa.visited.keys().len() as u32
}

pub fn part_2(input: &str) -> u32 {
    let mut santa = Santa::new();
    let mut santa_bot = Santa::new();
    santa.drop_gift();
    santa_bot.drop_gift();
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa.move_to(c);
        } else {
            santa_bot.move_to(c);
        }
    }
    merge_hash(santa.visited, santa_bot.visited).keys().len() as u32
}

pub fn run() {
    let input = get_input("problems/day3/input.txt");
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
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
